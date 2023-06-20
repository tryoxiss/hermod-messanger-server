#[macro_use]
mod terminal_out;

mod threading;
use threading::ThreadPool;

use std::net::TcpListener;

mod connection_handler;

fn main()
{
    info!("This is the program speaking now!");

    // warn_not_linux();

    info!("Initialising the Master Process");

    verify_file_integrity();
    check_updates();

    log!("Initalising TCP Stream");
    let network_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    warning!("TCP Is NOT ENCRYPTED and NOT SPEC COMPLIANT! DIM protocol
             is actually built in TLS! This is just for testing!");
    warning!("network_lister is bound to an UNWRAPPED VALUE!");

    // main portion

    let mut packets_handled: u128 = 0;
    let thread_pool = ThreadPool::new(4);

    // This automatically persists indefintely.
    for stream in network_listener.incoming() //.take(2) // test shutdown
    {
        let stream = stream.unwrap();

        warning!("stream is bound to an UNWRAPPED VALUE!");

        packets_handled += 1;
        thread_pool.run(|| { connection_handler::handle_connection(stream); });

        log!(format!("{packets_handled} packets handled"));
    }

    info!("Begining server shutdown ...");
}

fn verify_file_integrity()
{
    warning!("The function `verify_file_integrity()` currently has no functionality.");
    log!("Veryfying file integrity")
}

fn check_updates()
{
    warning!("The function `check_updaes()` currently has no functionality.");
    log!("Checking for Updates");
}
