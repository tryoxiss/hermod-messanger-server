#[macro_use]
mod terminal_out;

fn main() { 
    info!("info");
    warning!("warning");
    error!("error");
    fatal!("Fatal");

    log!("Entering main loop");

    /*
    _on_packet_recieved: 
        read_header
        find route
        store (if applicable) 
        send to route
    */
}