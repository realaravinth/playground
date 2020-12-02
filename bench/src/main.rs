use std::sync::mpsc;
use std::time::Duration;

use actix::prelude::*;
use clap::{App, Arg};

mod master;
mod sup;
mod worker;

pub static URL: &str = "http://localhost:5000";

fn main() {
    let _matches = App::new("bench - a stress-testing tool")
        .version("0.1")
        .author("Aravinth MAnivannan <realaravinth@batsense.net>")
        .arg(
            Arg::with_name("host")
                .short("-h")
                .long("--host")
                .value_name("endpoint url")
                .help("endpoint to stress-test")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("connections")
                .help("number of connections to use")
                .short("-c")
                .long("--connections")
                .value_name("connections(int)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("pow")
                .help("is PoW endpoint?")
                .short("-p")
                .long("--pow")
                .value_name("bool")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let threads = num_cpus::get();

    let sys = System::new("a");

    let fut = async move {
        run(threads).await;
    };

    Arbiter::spawn(fut);
    sys.run().unwrap();
}

async fn run(threads: usize) {
    use actix::clock::delay_for;
    let time = Duration::new(12, 0);
    let master = master::Master::new(threads).start();
    master.send(master::Parallelize).await;
    println!("timer start");

    delay_for(time).await;
    master.send(master::Exit).await;
    System::current().stop();
}
