/* 
 *      This file is part of:
 *      Codename Bonfire Instant Messanger
 *
 *      Copyright (C) 2023 or later - Project Bonfire Contributers
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 * 
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// use crate::terminal_out;

use std::net::TcpStream;
use std::io::prelude::*;

// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };

pub fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    warning!("Stream is read and then unwrapped! Don't unwrap!");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    warning!("Stream is written to and then unwrapped! Don't unwrap!");

    stream.flush().unwrap();
    warning!("Stream flush is unwrapped! Don't unwrap!");
}

#[derive(Debug)]
enum Operation
{ 
    Connect,
    Create, 
    Notify,
    Edit,
    Remove,
    Delete,
    Destroy,
    Get,

    InvalidRequest,
}

enum Address 
{ 
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
struct Packet 
{ 
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

impl Packet 
{ 
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
    fn raw_to_struct(packet_string: &str) -> Packet 
    { 
        // ☣️ UNTRUTED DATA WARNING! 
        // `packet_string` is direct from the network and is therefore 
        // UNTRUSTED USER DATA! and MUST be properly sanitised before we can
        // continue with this function! 

        log!("Parsing packet");

        let mut packet_vector: Vec<&str> = vec![];

        for part in packet_string.split(' ') 
        { 
            packet_vector.push(part);
        }

        // split every newline aswell

        // now we remove all the whitespace (""). 
        // ❗ ALERT: The performance of this has not been measured. 
        // It should execute in O(n + 1) time though.
        let mut index: usize = 0;
        let mut popped_number: usize = 0;

        for item in packet_vector.clone().iter() 
        { 
            index += 1;

            if item != &"" { continue }
            packet_vector.remove(index - popped_number - 1);
            popped_number += 1;
        }

        // .into_string() does the same but with less memory? 
        // https://stackoverflow.com/questions/27201086/comparing-string-in-rust
        if !matches!(packet_vector[0], 
            "CONNECT" |
            "CREATE"  |
            "NOTIFY"  |
            "EDIT"    |
            "REMOVE"  |
            "DELETE"  |
            "DESTROY" |
            "GET"     ) 
        {
            error!("Did not find OPERATION in PACKET HEADER. Returning `401 Bad_Request`");
            return Packet 
            {
                edition: String::from("ERROR"),
                operation: Operation::InvalidRequest,
                target: Vec::new(),
                content: String::from("ERROR")
            };
        }

        // if !matches!(packet_vector[1], 
        //     "CONNECT" |
        //     "CREATE"  |
        //     "NOTIFY"  |
        //     "EDIT"    |
        //     "REMOVE"  |
        //     "DELETE"  |
        //     "DESTROY" |
        //     "GET"     ) 
        // {
        //     error!("Did not find TYPE in PACKET HEADER. Returning `401 Bad_Request`");
        // }

        if !matches!(packet_vector[2], "FROM") 
        {
            error!("Did not find `FROM` keyword in PACKET HEADER. Returning `401 Bad_Request`");
        }

        // if !matches!(packet_vector[3], "TARGET") {
        //     error!("Did not find TARGET in PACKET HEADER. Returning `401 Bad_Request`");
        // }

        if !matches!(packet_vector[4], "WITH") 
        {
            error!("Did not find `WITH` keyword in PACKET HEADER. Returning `401 Bad_Request`");
        }

        if !matches!(packet_vector[5], "dim/2023") 
        {
            error!("Did not find EDITION in PACKET HEADER. Returning `401 Bad_Request`");
        }

        // if !matches!(packet_vector[6], "AND") 
        // {
        //     log!("Did not find `WITH` keyword in PACKET HEADER.");
        // }

        // if !matches!(packet_vector[7], "aes" | "rsa") 
        // {
        //     log!("Did not find `WITH` keyword in PACKET HEADER.");
        // }

        // optional: Encryption info

        // println!("{:?}", packet_vector);

        // println!("{:?}", hiss);
        // println!("{:?}", packet_string.split(' '));

        return Packet 
        { 
            edition: String::from("2023"),
            operation: Operation::Get,
        
            target: Vec::new(),
            content: String::from(packet_string),
        
            // signature: Signature,
        }
    }

    fn new(edition: &str, operation: Operation, target: Vec<Guid>, content: &str) -> Packet 
    { 
        return Packet 
        { 
            edition: String::from(edition),
            operation: operation,
        
            target: target,
            content: String::from(content),
        
            // signature: Signature,
        }
    }
}

// struct ConnectRequest 
// { 
//     edition: String,

//     target: Address,

//     signature: Signature
// }

// struct GetRequest 
// { 
//     edition: String,

//     target: Vec<Guid>,

//     signature: Signature
// }

// struct CreateRequest 
// { 
//     edition: String,

//     target: Guid, // channel 
//     content: String,

//     signature: Signature
// }

// struct EditRequest 
// { 
//     edition: String,

//     target: Guid, // object (message, wiki page, etc)
//     content: String,

//     signature: Signature
// }

// struct RemoveRequest 
// { 
//     edition: String,

//     target: String,
// }

// struct DeleteRequest 
// { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct DestroyRequest 
// { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct NotifyRequest 
// { 
//     edition: String,

//     target: String,
//     content: String,

//     signature: Signature
// }

// impl Packet for GetRequest 
// { 
//     fn edition(&self) -> String 
//     { 
//         return self.edition; 
//     }

//     fn target(&self) -> Vec<Guid> 
//     { 
//         return self.target;
//     }

//     fn validate_signature(&self, _public_key: Key) -> bool 
//     {
//         return false;
//     }
// }

pub fn handle_request() 
{ 
    log!("Handle Request Called");
    
    let packet;
    packet = Packet::raw_to_struct(
        "GET messages FROM 99f97c79dfae4520a650df014d665be7 WITH dim/2023 AND aes
        content: | 
        \"This is my content
        uwU
        I love you!!\"
        
        SIGNED \"9320ea11f6d427aec4949634dc8676136b2fa8cdad289d22659b44541abb8c51fbeb6b678ded0c9c8a0eec2313192d3a2352b93b4a0e7dbfe29eb5e8dd2e0dcd7f6daf2377a6cbbae6cefdd132536988ad4cea2d36b8334b0a1d928df2341120\"
        ");

    if packet.edition == "ERROR".to_string() 
    { 
        log!("Found an `ERROR` in the packets edition. Returning!");
        return;
    }

    // println!("{:?}", packet); 



    let _packet;
    _packet = Packet::raw_to_struct(
        "NOTIFY messages FROM 99f97c79dfae4520a650df014d665be7 WITH dim/2023 AND aes
        content: | 
        \"This is my content
        uwU
        I love you!!\"
        
        SIGNED \"9320ea11f6d427aec4949634dc8676136b2fa8cdad289d22659b44541abb8c51fbeb6b678ded0c9c8a0eec2313192d3a2352b93b4a0e7dbfe29eb5e8dd2e0dcd7f6daf2377a6cbbae6cefdd132536988ad4cea2d36b8334b0a1d928df2341120\"
        ");

    // println!("{:?}", packet); 
}

#[cfg(test)]
mod tests
{   
    use crate::connection_handler::Packet;

    #[test]
    fn notify_1()
    {
        let packet = Packet::raw_to_struct(
            "NOTIFY messages FROM 99f97c79dfae4520a650df014d665be7 WITH dim/2023 AND aes
            content: | 
            \"This is my content
            uwU
            I love you!!\"
            
            SIGNED \"9320ea11f6d427aec4949634dc8676136b2fa8cdad289d22659b44541abb8c51fbeb6b678ded0c9c8a0eec2313192d3a2352b93b4a0e7dbfe29eb5e8dd2e0dcd7f6daf2377a6cbbae6cefdd132536988ad4cea2d36b8334b0a1d928df2341120\"
            ");
        
        assert_eq!(packet.edition, "dim/2023");
    }

    #[test]
    fn notify_2()
    {
        warning!("bonfire:Not_Implemented_Error for `mod tests` -> `notify_2` in bonfire:packer_handler.rs")
    }
}
