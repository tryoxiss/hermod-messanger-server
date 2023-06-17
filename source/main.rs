#[macro_use]
mod terminal_out;

mod packet_handler;
mod network_manager;

use network_manager::NetworkConnection;

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
    check_updaes();

    let network_stream: NetworkConnection = network_manager::NetworkConnection::establish_connection();

    // log!("test (en)decrypt");

    packet_handler::handle_request();

    // let mut cycles = 0;

    loop 
    { 
        // on packet recieved: 
        // packet_handler::handle_request();

        // 🚨 Refactor RECCOMMENDED! 
        // This should increment every 10 secconds to show the wait time.
        // and should reset if any action is done. 

        // waiting!(cycles)
        // cycles += 1
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

fn check_updaes()
{
    warning!("The function `check_updaes()` currently has no functionality.");
    log!("Checking for Updates");
}