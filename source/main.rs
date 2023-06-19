#[macro_use]
mod terminal_out;

use std::net::TcpListener;

mod connection_handler;

fn main() 
{ 
    info!("This is the program speaking now!");

    // Silence annoying "unused import" warning.
    if true == false 
    { 
        log!("Entering main loop");
        info!("Info");
        warning!("warning");
        error!("error");
        fatal!("Fatal");
    }

    info!("Initialising the Master Process");

    verify_file_integrity();
    check_updates();

    log!("Initalising TCP Stream");
    let network_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    warning!("TCP Is NOT ENCRYPTED and NOT SPEC COMPLIANT! DIM protocol 
             is actually built in TLS! This is just for testing!");
    warning!("network_lister is bound to an UNWRAPPED VALUE!");

    // packet_handler::handle_request();

    let mut packets_handled: u128 = 0;

    // This automatically persists indefintely. 
    for stream in network_listener.incoming()
    {
        let stream = stream.unwrap();
        warning!("stream is bound to an UNWRAPPED VALUE!");

        log!("Attempting to connect ...");
        connection_handler::handle_connection(stream);

        // take note of requests handled for log files
        packets_handled += 1;
        log!(format!("{packets_handled} packets handled"));
    }

    /*
    _on_packet_recieved: 
        read_header
        find route
        store (if applicable) 
        send to route
    */
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