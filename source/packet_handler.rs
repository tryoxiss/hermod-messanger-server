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
struct Signature ( u128, u128, u128, u128 );

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

    /// Packet format: 
    /// ---
    /// <OPERATION or RESPONSE_NUMERICAL_CODE> <type or RESPONSE_STRING_CODE> "FROM" <target GUID or dim: protocol uri> "WITH" <subject_identifier> ["AND" <encryption algorithum>]
    /// key: value
    /// key2: value2
    /// content: | 
    /// "This is my content
    /// uwU
    /// I love you!!"
    /// 
    /// "SIGNED" """<signature>"""
    /// ---
    /// Signature always goes at the end. 
    /// So an example packet is
    /// GET messages FROM 99f97c79dfae4520a650df014d665be7 WITH bonfire-2023 AND aes
    /// content: | 
    /// "This is my content
    /// uwU
    /// I love you!!"
    /// 
    /// SIGNED "9320ea11f6d427aec4949634dc8676136b2fa8cdad289d22659b44541abb8c51fbeb6b678ded0c9c8a0eec2313192d3a2352b93b4a0e7dbfe29eb5e8dd2e0dcd7f6daf2377a6cbbae6cefdd132536988ad4cea2d36b8334b0a1d928df2341120"
    /// ---
    /// Signatre is always at the end. Content can be anywhere (the key value bits can be in any order, but the header (First line) needs to be on the first line always, and the signature always needs to be on the last line)
    fn raw_to_struct(packet_string: &str) -> Packet { 
        // ☣️ UNTRUTED DATA WARNING! 
        // `packet_string` is direct from the network and is therefore 
        // UNTRUSTED USER DATA! and MUST be properly sanitised before we can
        // continue with this function! 

        log!("Parsing packet");

        let mut packet_vector: Vec<&str> = vec![];

        for part in packet_string.split(' ') { 
            packet_vector.push(part);
        }

        // now we remove all the whitespace (""). 
        // ❗ ALERT: The performance of this has not been measured. 
        // It should execute in O(n + 1) time though.
        let mut index: usize = 0;
        let mut popped_number: usize = 0;
        for item in packet_vector.clone().iter() { 

            index += 1;

            if item != &"" { continue }
            packet_vector.remove(index - popped_number - 1);
            popped_number += 1;
        }

        println!("{:?}", packet_vector);

        // println!("{:?}", hiss);
        // println!("{:?}", packet_string.split(' '));

        return Packet { 
            edition: String::from("2023"),
            operation: Operation::Get,
        
            target: Vec::new(),
            content: String::from(packet_string),
        
            // signature: Signature,
        }
    }

    fn new(edition: &str, operation: Operation, target: Vec<Guid>, content: &str) -> Packet { 
        return Packet { 
            edition: String::from(edition),
            operation: operation,
        
            target: target,
            content: String::from(content),
        
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
    packet = Packet::raw_to_struct(
        "GET messages FROM 99f97c79dfae4520a650df014d665be7 WITH bonfire-2023 AND aes
        content: | 
        \"This is my content
        uwU
        I love you!!\"
        
        SIGNED \"9320ea11f6d427aec4949634dc8676136b2fa8cdad289d22659b44541abb8c51fbeb6b678ded0c9c8a0eec2313192d3a2352b93b4a0e7dbfe29eb5e8dd2e0dcd7f6daf2377a6cbbae6cefdd132536988ad4cea2d36b8334b0a1d928df2341120\"
        ");

    println!("{:?}", packet); 



    let packet;
    packet = Packet::raw_to_struct("I want cuddles");

    println!("{:?}", packet); 
}