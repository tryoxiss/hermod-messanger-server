use log::{debug, error, info, trace, warn};
use log4rs;

// #[macro_use]
mod terminal_out;
use terminal_out::ask_yes_no;

mod threading;
use threading::ThreadPool;

use std::net::TcpListener;

mod connection_handler;

fn main()
{
    log4rs::init_file("logging_config.yml", Default::default()).unwrap();

    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning you unwrap too much");
    error!("error message here");

    info!("This is the program speaking now!");

    info!("Initialising the Master Process");

    // fatal!(901, "I had bad grammer and now I need to fix it");

    // Define an automatic-restart threashold. 
    let max_requests: usize = usize::MAX;

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
    for stream in network_listener.incoming().take(max_requests) // test shutdown
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
    // get repo from config files
    // get SHA-2 hash of files
    // Get desired SHA-2 hash from repo
    // compare them

    let our_hash = "Good2Go";
    let server_hash = "Good2Go";

    info!("Checking file integrity ...");
    if our_hash == server_hash
    {
        info!("File integrity good!");
    }
    else
    {
        error!("File hashes do not match. If you just set this server up, 
             check that the repo listed in your `bonfire.config` file is 
             the same as where you got your source code from. (If you 
             used an install script, it's most likely correct)
             
             if it is not a fresh install, this likely means your files 
             are either corrupted or tampered with!");

        let fix_files = ask_yes_no("Would you like to fix your files?");
        // todo!("Implement this!");
    }

    warn!("The function `verify_file_integrity()` currently has 
             no functionality.");
    trace!("Veryfying file integrity")
}

fn check_updates()
{
    // get repo from config file
    // get version from config file
    // Get desired hash from repo
    // get version from repo
    // compare version
    // if version is less than (patch, A.A.THIS): 
    //      Suggest Update
    // if version is less than (secrity A.A.THIS AND A.A.CURRENT_VERSION is DEPRECATED or YANKED):
    //      *strongly* suggest update
    // if version is less than (major): 
    //      suggest update when next doing major admin stuff/setting up new servers

    warn!("The function `check_updaes()` currently has no functionality.");
    trace!("Checking for Updates");
}
