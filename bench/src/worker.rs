use actix::prelude::*;
use actix_web::client::Client;
use lazy_static::lazy_static;
use pow_sha256::PoW;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref WITHOUT_POW: Payload = Payload::new(2, "aa");
}

static STATIC: &str = "http://localhost:5000/";
static REGISTER: &str = "http://localhost:5000/api/signup";
static POW: &str = "http://localhost:5000/api/pow";

use super::master;

#[derive(Message)]
#[rtype(result = "()")]
pub enum Attack {
    Static,
    WithPow,
    WithoutPow,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopWorker;

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

    fn handle(&mut self, msg: Attack, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address();
        let m_addr = self.master_addr.clone();

        let client = Client::default();
        match msg {
            Attack::Static => {
                let attack = async move {
                    loop {
                        if let Ok(r) = client.get(STATIC).send().await {
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
            Attack::WithoutPow => {
                let attack = async move {
                    loop {
                        if let Ok(r) = client.post(REGISTER).send_json(&*WITHOUT_POW).await {
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

            Attack::WithPow => {
                let attack = async move {
                    loop {
                        if let Ok(mut res) = client.get(POW).send().await {
                            let pow_config: std::result::Result<
                                PowConfig,
                                actix_web::client::JsonPayloadError,
                            > = res.json().await;

                            if let Ok(pow_config) = pow_config {
                                let payload: Payload = pow_config.into();

                                if let Ok(r) = client.post(REGISTER).send_json(&payload).await {
                                    let status = r.status().as_u16() as usize;
                                    if let Err(_) = &m_addr.send(master::AddStatus(status)).await {
                                        &addr.do_send(StopWorker);
                                    }
                                }
                            }
                        }
                    }
                }
                .into_actor(self);

                ctx.spawn(attack);
            }
        }
    }
}

fn gen_pow(difficulty_factor: u32, secret: &str) -> PoW<Vec<u8>> {
    let difficulty = u128::max_value() - u128::max_value() / difficulty_factor as u128;
    PoW::prove_work(&secret.as_bytes().to_vec(), difficulty).unwrap()
}

#[derive(Debug, Serialize)]
pub struct Payload {
    username: String,
    password: String,
    pow: PoW<Vec<u8>>,
    phrase: String,
}

#[derive(Deserialize)]
struct PowConfig {
    difficulty: u32,
    phrase: String,
}

impl From<PowConfig> for Payload {
    fn from(c: PowConfig) -> Self {
        Payload::new(c.difficulty, &c.phrase)
    }
}

impl Payload {
    pub fn new(difficulty: u32, phrase: &str) -> Self {
        let pow = gen_pow(difficulty, phrase);
        Payload {
            username: "aaa".into(),
            password: "aaa".into(),
            pow,
            phrase: phrase.to_string(),
        }
    }
}
