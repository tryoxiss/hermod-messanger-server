#[macro_use]
mod terminal_out;

mod packet_handler;

fn main() { 
    info!("This is the program speaking now!");

    let mut wait_time: u64 = 0;
    let mut cycles: u64 = 0;
    //                 5_000_000_000
    let WAIT_FOR_MESSAGE_TIME: u64 = 10_000_000_000; 

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
        wait_time += 1;

        // when a request comes in, currently just a demo block.
        if false { 
            wait_time = 0;
            cycles = 1;
        };

        if WAIT_FOR_MESSAGE_TIME <= wait_time { 
            // println!("{cycles}; {wait_time}; ");

            wait_time = 0;
            cycles += 1;

            waiting!(cycles)
        };
    }

    /*
    _on_packet_recieved: 
        read_header
        find route
        store (if applicable) 
        send to route
    */
}
