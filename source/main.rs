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

    log!("test (en)decrypt");

    packet_handler::handle_request();

    loop { 

        // 🚨 Refactor RECCOMMENDED! 
        // This should increment every 10 secconds to show the wait time.
        // and should reset if any action is done. 

        // waiting!(cycles)
    }

    /*
    _on_packet_recieved: 
        read_header
        find route
        store (if applicable) 
        send to route
    */
}
