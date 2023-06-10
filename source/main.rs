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

        // 🚨 Refactor REQUIRED! 
        // This is a TERRIBLE solution and MUST be fixed. Probably with
        // a time library or proper time units. 

        wait_time += 1;  // 🚩 A lot of workfor an idle state. 

        // when a request comes in, currently just a demo block.
        if false { 
            wait_time = 0;  // 🚩 Possibly wasting a lot of work
            cycles = 1;     // 🚩 Possibly wasting a lot of work
        };

        if WAIT_FOR_MESSAGE_TIME <= wait_time { 
            // println!("{cycles}; {wait_time}; ");

            wait_time = 0; // 🚩 Possibly wasting a lot of work
            cycles += 1;   // 🚩 A lot of workfor an idle state. 

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
