use actix::prelude::*;

use std::collections::HashMap;

use super::sup;
use super::sup::AttackType;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SubscribeSup(pub Addr<sup::Sup>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Parallelize(pub sup::AttackType);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Exit;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopSup(pub Addr<sup::Sup>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reduce(pub HashMap<usize, usize>);

pub struct Master {
    pub resp: HashMap<usize, usize>,
    pub reqs: usize,
    pub num_sups: usize,
}

impl Master {
    pub fn new(num_sups: usize) -> Self {
        Master {
            resp: HashMap::new(),
            num_sups,
            reqs: 0,
        }
    }
}

impl Actor for Master {
    type Context = Context<Self>;
}

async fn start_attack(addr: Addr<Master>, atype: sup::AttackType) {
    let s = sup::Sup::default().start();
    let msg = sup::Attack {
        addr,
        attack_type: atype,
    };
    s.send(msg).await;
}

impl Handler<Parallelize> for Master {
    type Result = ();

    fn handle(&mut self, msg: Parallelize, ctx: &mut Self::Context) -> Self::Result {
        println!("Starting...");
        for _ in 0..12 {
            let addr = ctx.address().clone();
            match msg.0 {
                AttackType::Static => {
                    Arbiter::new().exec_fn(|| {
                        let fut = async {
                            start_attack(addr, AttackType::Static).await;
                        };
                        Arbiter::spawn(fut);
                    });
                }
                AttackType::WithPow => {
                    Arbiter::new().exec_fn(|| {
                        let fut = async {
                            start_attack(addr, AttackType::WithPow).await;
                        };
                        Arbiter::spawn(fut);
                    });
                }

                AttackType::WithoutPow => {
                    Arbiter::new().exec_fn(|| {
                        let fut = async {
                            start_attack(addr, AttackType::WithoutPow).await;
                        };
                        Arbiter::spawn(fut);
                    });
                }
            }
        }
    }
}

impl Handler<Exit> for Master {
    type Result = ();
    fn handle(&mut self, _msg: Exit, ctx: &mut Self::Context) -> Self::Result {
        println!();
        println!();
        println!("Total requests {:#?}", self.reqs);
        println!("Requests per second: {:#?}", self.reqs as f32 / 10 as f32);

        ctx.stop();
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddStatus(pub usize);

impl Handler<AddStatus> for Master {
    type Result = ();

    fn handle(&mut self, msg: AddStatus, _ctx: &mut Self::Context) -> Self::Result {
        //        if self.resp.contains_key(&msg.0) {
        //            let val = self.resp.get_mut(&msg.0).unwrap();
        //            *val += 1;
        //        } else {
        //            self.resp.insert(msg.0, 1);
        //        }
        //
        self.reqs += 1;
    }
}
