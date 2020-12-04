use actix::prelude::*;

use super::master;
use super::worker;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Attack {
    pub addr: Addr<master::Master>,
    pub attack_type: AttackType,
}

pub enum AttackType {
    Static,
    WithPow,
    WithoutPow,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopSup(pub Addr<master::Master>);

pub struct Sup {}

impl Default for Sup {
    fn default() -> Self {
        Sup {}
    }
}

impl Actor for Sup {
    type Context = Context<Self>;
}

impl Handler<Attack> for Sup {
    type Result = ();

    fn handle(&mut self, msg: Attack, ctx: &mut Self::Context) -> Self::Result {
        match msg.attack_type {
            AttackType::Static => {
                let attack = async move {
                    for _ in 0..300 {
                        let wrk = worker::Worker::new(msg.addr.clone()).start();
                        wrk.send(worker::Attack::Static).await.unwrap();
                    }
                    println!("1000 workers started");
                }
                .into_actor(self);
                ctx.wait(attack);
            }
            AttackType::WithoutPow => {
                let attack = async move {
                    for _ in 0..300 {
                        let wrk = worker::Worker::new(msg.addr.clone()).start();
                        wrk.send(worker::Attack::WithoutPow).await.unwrap();
                    }
                    println!("1000 workers started");
                }
                .into_actor(self);
                ctx.wait(attack);
            }

            AttackType::WithPow => {
                let attack = async move {
                    for _ in 0..300 {
                        let wrk = worker::Worker::new(msg.addr.clone()).start();
                        wrk.send(worker::Attack::WithPow).await.unwrap();
                    }
                    println!("1000 workers started");
                }
                .into_actor(self);
                ctx.wait(attack);
            }
        }
    }
}

impl Handler<StopSup> for Sup {
    type Result = ();
    fn handle(&mut self, _msg: StopSup, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
