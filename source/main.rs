use log::{debug, error, info, trace, warn};
use log4rs;

// #[macro_use]
// mod terminal_out;

mod threading;
use threading::ThreadPool;

use std::net::TcpListener;

mod connection_handler;

// {format!( {{h(\x1b[1m {l})}:>} ) \x1b[0m {m}{n}

fn main()
{
    log4rs::init_file("logging_config.yml", Default::default()).unwrap();

    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning this program doesn't do much");
    error!("error message here");

    info!("This is the program speaking now!");

    info!("Initialising the Master Process");

    // fatal!(901, "I had bad grammer and now I need to fix it");

    verify_file_integrity();
    check_updates();

    trace!("Initalising TCP Stream");
    let network_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    warn!("TCP Is NOT ENCRYPTED and NOT SPEC COMPLIANT! DIM protocol
             is actually built in TLS! This is just for testing!");
    warn!("network_lister is bound to an UNWRAPPED VALUE!");

    // main portion

    let mut packets_handled: u128 = 0;
    let thread_pool = ThreadPool::new(4);

    // This automatically persists indefintely.
    for stream in network_listener.incoming() //.take(2) // test shutdown
    {
        let stream = stream.unwrap();

        warn!("stream is bound to an UNWRAPPED VALUE!");

        packets_handled += 1;
        thread_pool.run(|| { connection_handler::handle_connection(stream); });

        trace!("{packets_handled} packets handled");
    }

    info!("Begining server shutdown ...");
}

fn verify_file_integrity()
{
    warn!("The function `verify_file_integrity()` currently has no functionality.");
    trace!("Veryfying file integrity")
}

fn check_updates()
{
    warn!("The function `check_updaes()` currently has no functionality.");
    trace!("Checking for Updates");
}
