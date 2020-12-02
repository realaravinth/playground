use actix::prelude::*;
use actix_rt::System;
use awc::Client;
use futures::future;
use std::sync::atomic::AtomicBool;

fn main() {
    let mut threads = Vec::new();
    for _ in 0..12 {
        let thread = std::thread::spawn(|| {
            System::new("test").block_on(async {
                let client = Client::default();

                let mut attack_fit = Vec::new();
                for _ in 0..10_000 {
                    attack_fit.push(attack(&client));
                }
                future::join_all(attack_fit).await;
            });
        });
        println!("new sys");
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

async fn attack(client: &Client) {
    let _ = client
        .get("http://localhost:5000") // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send() // <- Send http request
        .await;
}
