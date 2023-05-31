// why is this so hard...
use crate::terminal_out;

// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };

#[derive(Debug)]
enum Operation { 
    Connect,
    Create, 
    Notify,
    Edit,
    Remove,
    Delete,
    Destroy,
    Get
}

enum Address { 
    DIMAddress,
    IPv6Address,
}

#[derive(Debug)]
struct Guid ( u128 );
#[derive(Debug)]
struct Signature ( u128 );

struct IPv4 ( u8, u8, u8, u8 ); // ew                      (u8.u8.u8.u8)
struct IPv6 ( u16, u16, u16, u16, u16, u16, u16, u16 ); // (u16:u16:u16:u16:u16:u16:u16:u16)

#[derive(Debug)]
struct Packet { 
    edition: String,
    operation: Operation,

    target: Vec<Guid>,
    content: String,

    // signature: Signature,
}

// impl Signature { 
//     fn new() -> Signature { 
//         return (1);
//     }
// }

impl Packet { 
    fn parse_to_struct(packet_string: &str) -> Packet { 
        log!("Parsing packet");

        return Packet { 
            edition: String::from("2023"),
            operation: Operation::Get,
        
            target: Vec::new(),
            content: String::from(packet_string),
        
            // signature: Signature,
        }
    }

    fn new() -> Packet { 
        return Packet { 
            edition: String::from("2023"),
            operation: Operation::Get,
        
            target: Vec::new(),
            content: String::from("packet_string"),
        
            // signature: Signature,
        }
    }
}

// struct ConnectRequest { 
//     edition: String,

//     target: Address,

//     signature: Signature
// }

// struct GetRequest { 
//     edition: String,

//     target: Vec<Guid>,

//     signature: Signature
// }

// struct CreateRequest { 
//     edition: String,

//     target: Guid, // channel 
//     content: String,

//     signature: Signature
// }

// struct EditRequest { 
//     edition: String,

//     target: Guid, // object (message, wiki page, etc)
//     content: String,

//     signature: Signature
// }

// struct RemoveRequest { 
//     edition: String,

//     target: String,
// }

// struct DeleteRequest { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct DestroyRequest { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct NotifyRequest { 
//     edition: String,

//     target: String,
//     content: String,

//     signature: Signature
// }

// impl Packet for GetRequest { 
//     fn edition(&self) -> String { 
//         return self.edition;
//     }

//     fn target(&self) -> Vec<Guid> { 
//         return self.target;
//     }

//     fn validate_signature(&self, _public_key: Key) -> bool {
//         return false;
//     }
// }

pub fn handle_request() { 
    log!("Handle Request Called");
    
    let packet;
    packet = Packet::parse_to_struct("Hiss");

    println!("{:?}", packet); 



    let packet;
    packet = Packet::parse_to_struct("I want cuddles");

    println!("{:?}", packet); 
}