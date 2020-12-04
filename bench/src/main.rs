use std::sync::mpsc;
use std::time::Duration;

use actix::prelude::*;
use clap::{App, Arg};

mod master;
mod sup;
mod worker;

use sup::AttackType;

pub static URL: &str = "http://localhost:5000";

fn main() {
    lazy_static::initialize(&worker::WITHOUT_POW);
    let matches = App::new("bench - a stress-testing tool")
        .version("0.1")
        .author("Aravinth MAnivannan <realaravinth@batsense.net>")
        .arg(
            Arg::with_name("static")
                .short("-s")
                .long("--static")
                .value_name("static")
                .help("hits static endpoint")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("non-pow")
                .help("hits non-pow endpoint")
                .short("-n")
                .long("--npow")
                .value_name("npow")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("pow")
                .help("is PoW endpoint?")
                .short("-p")
                .long("--pow")
                .value_name("pow")
                .takes_value(false),
        )
        .get_matches();

    let atype;
    if matches.is_present("static") {
        atype = AttackType::Static
    } else if matches.is_present("non-pow") {
        atype = AttackType::WithoutPow
    } else {
        atype = AttackType::WithPow
    }

    let threads = num_cpus::get();

    let sys = System::new("a");

    let fut = async move {
        run(threads, atype).await;
    };

    Arbiter::spawn(fut);
    sys.run().unwrap();
}

async fn run(threads: usize, atype: AttackType) {
    use actix::clock::delay_for;
    let time = Duration::new(12, 0);
    let master = master::Master::new(threads).start();
    master.send(master::Parallelize(atype)).await;

    delay_for(time).await;
    master.send(master::Exit).await;
    System::current().stop();
}
