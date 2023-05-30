#[macro_use]
mod terminal_out;

mod packet_handler;

fn main() { 
    info!("This is the program speaking now!");

    // Silence annoying "unused import" warning.
    if true == false { 
        log!("Entering main loop");
        info!("Info");
        warning!("warning");
        error!("error");
        fatal!("Fatal");
    }

    log!("test en/decrypt");

    /*
    _on_packet_recieved: 
        read_header
        find route
        store (if applicable) 
        send to route
    */
}
