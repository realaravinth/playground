use actix::prelude::*;
use actix_web::client::Client;
use std::collections::HashMap;

use super::master;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Attack;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopWorker;

use crate::URL;

pub struct Worker {
    pub master_addr: Addr<master::Master>,
}

impl Worker {
    pub fn new(master_addr: Addr<master::Master>) -> Self {
        Worker { master_addr }
    }
}

impl Actor for Worker {
    type Context = Context<Self>;
}

impl Handler<StopWorker> for Worker {
    type Result = ();

    fn handle(&mut self, _msg: StopWorker, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<Attack> for Worker {
    type Result = ();

    fn handle(&mut self, _msg: Attack, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address();
        let m_addr = self.master_addr.clone();

        let client = Client::default();
        let attack = async move {
            loop {
                if let Ok(r) = client.get(URL).send().await {
                    let status = r.status().as_u16() as usize;
                    if let Err(_) = &m_addr.send(master::AddStatus(status)).await {
                        &addr.do_send(StopWorker);
                    }
                }
            }
        }
        .into_actor(self);

        ctx.spawn(attack);
    }
}
