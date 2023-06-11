extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use std::env;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };

    pinger.add_ipaddr("1.1.1.1"); // add the IP addresses to ping here
    pinger.run_pinger();

    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    error!("Idle Address {}.", addr);
                }
                Receive { addr, rtt } => {
                    info!("Receive from Address {} in {:?}.", addr, rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}
