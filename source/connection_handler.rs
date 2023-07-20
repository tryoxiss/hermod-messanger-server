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

use log::{trace, debug, info, warn, error};

use std::net::TcpStream;
use std::io::prelude::*;
use native_tls::TlsStream;

use aes_gcm_siv::{
    aead::{/* Aead, */ KeyInit, OsRng},
    Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
};

// use rand_chacha::ChaCha20Rng;

use crate::{ENDBLOCK, CODE_START, INDENT};

pub fn handle_connection(mut stream: TlsStream<TcpStream>)
{
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    // let cipher = Aes256GcmSiv::new(&key);
    // let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    // let ciphertext: Vec<u8> = cipher.encrypt(nonce, b"plaintext message".as_ref()).unwrap();
    // let plaintext = cipher.decrypt(nonce, ciphertext.as_ref());
    // assert_eq!(&plaintext.unwrap(), b"plaintext message");

    // trace!("{:?}, {:?}", &plaintext.unwrap(), b"plaintext message");

    const MAX_PACKET_LENGTH: usize = 1048576;
    let mut buffer = [0; MAX_PACKET_LENGTH];

    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let mut response_variables: Vec<HeaderVariable> = vec![];

        if buffer[MAX_PACKET_LENGTH - 1] != "\x00".as_bytes()[0]
        {
            response_variables.push(HeaderVariable::new("encyption", "aes"));
            response_variables.push(HeaderVariable::new("force_encryption", "t"));
    
            let response: String = ResponsePacket::create(
                String::from("1.0"),
                411,
                String::from("Payload Too Large"),
                response_variables,
                String::from("max_length=1_048_575 ; Our maximum packet length is 1_048_575 bytes (1 MiB - 1 byte). If your content is larger than this, please use a packet series. You can do this by adding the `set=<u64>;`, and `index=<u64>` variable in the header to designate thier order. Alternatively, you may choose to load media through alternate sources such as HTTPS.")
                );
    
            stream.write(&response.as_bytes())
                .expect("Failed to write to TCP Stream!");
    
            return;
        }
    
        let mut header_variables: Vec<HeaderVariable> = vec![];
    
        let header_length: usize = 0;
        for variable in 0..header_length
        {
            header_variables.push(HeaderVariable::new("key", "value"));
        }
    
        header_variables.push(HeaderVariable::new("encyption", "aes"));
        header_variables.push(HeaderVariable::new("force_encryption", "t"));
        header_variables.push(HeaderVariable::new("author", "8d1a0cfb13df4ca3bdb0e912be01863b"));
        header_variables.push(HeaderVariable::new("target", "none"));
        header_variables.push(HeaderVariable::new("channel", "20026f0a1c484f95a0063d148c8898f9"));
        header_variables.push(HeaderVariable::new("channel_type", "text_message"));
        header_variables.push(HeaderVariable::new("content_mime_type", "text/plain"));
        header_variables.push(HeaderVariable::new("content_formatting", "none"));
        header_variables.push(HeaderVariable::new("time_sent", "2023-06-25 12:25:22"));
    
        /*
         * SUPPORTED TYPES for `content_formatting`
         * AAA Support (Virtually Required and officailly endorsed)
         * - none (Plain Text)
         * - rich-markdown (see DIM Markdown Specification)
         * - wikitext
         * - variables (INI Format)
         *      Chosen because, even if its not your prefered format,
         *      it's dead simple and does everything we need it to do.
         *      it dosen't have a bunch of fancy stuff, just 
         *      key = value ; comment
         *      NOTE: comments with # are NOT ALLOWED!!
         *
         * AA Support (Probably some fancier clients, not offically endorsed)
         * - commonmark
         *
         * A Support (Nieche/Ehh?)
         * - universal-chess-interface
         *
         * E Support (Deprecated)
         * - None!
         *
         * F Support (Actively Discouraged)
         * - html - DIM Clients are not web browsers!!
         * - <Any Code> - Use a code block in markdown!!
         */
    
        // DIM
        let response = ResponsePacket::create(
            String::from("1.0"),
            200,
            String::from("Serving"),
            header_variables,
            String::from("Manically laughs at the futility of life. Oh also I got DIM packets sorta being contructed!")
            );
        
        // Writes some prefix of the byte string, not necessarily all of it.
        stream.write(response.as_bytes()).unwrap();
    
        // let plaintext = cipher.decrypt(nonce, response.as_ref()).unwrap();
    
    //     match stream.write(&plaintext)
    //     {
    //         Ok(message) =>
    //         {
    //             trace!("Wrote to the TCP Stream");
    //         }
    
    //         Err(error) =>
    //         {
    //             error!("The TCP Stream write failed!
    // {INDENT}{CODE_START}connection_handler.rs::handle_connection(){ENDBLOCK}
    // {INDENT}Here we provide the compilers error:
    // {error} ");
    //             panic!("Why would the TCP stream flush panic !");
    //         }
    //     }
    
        match stream.flush()
        {
            Ok(_message) =>
            {
                trace!("TCP Stream Flushed");
            }
    
            Err(error)  =>
            {
                error!("The TCP Stream flush failed!
    {INDENT}{CODE_START}connection_handler.rs::handle_connection(){ENDBLOCK}
    {INDENT}Here we provide the compilers error:
    {error} ");
                panic!("Why would the TCP stream flush panic !");
            }
        }
}

// pub fn handle_connection(mut stream: TcpStream)
// {



//     let key = Aes256GcmSiv::generate_key(&mut OsRng);
//     // let cipher = Aes256GcmSiv::new(&key);
//     // let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//     // let ciphertext: Vec<u8> = cipher.encrypt(nonce, b"plaintext message".as_ref()).unwrap();
//     // let plaintext = cipher.decrypt(nonce, ciphertext.as_ref());
//     // assert_eq!(&plaintext.unwrap(), b"plaintext message");

//     // trace!("{:?}, {:?}", &plaintext.unwrap(), b"plaintext message");

//     let cipher = Aes256GcmSiv::new(&key);
//     let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

//     let mut response_variables: Vec<HeaderVariable> = vec![];

//     if buffer[MAX_PACKET_LENGTH - 1] != "\x00".as_bytes()[0]
//     {
//         response_variables.push(HeaderVariable::new("encyption", "aes"));
//         response_variables.push(HeaderVariable::new("force_encryption", "t"));

//         let response: String = ResponsePacket::create(
//             String::from("1.0"),
//             411,
//             String::from("Payload Too Large"),
//             response_variables,
//             String::from("max_length=1_048_575 ; Our maximum packet length is 1_048_575 bytes (1 MiB - 1 byte). If your content is larger than this, please use a packet series. You can do this by adding the `set=<u64>;`, and `index=<u64>` variable in the header to designate thier order. Alternatively, you may choose to load media through alternate sources such as HTTPS.")
//             );

//         stream.write(&response.as_bytes())
//             .expect("Failed to write to TCP Stream!");

//         return;
//     }

//     let mut header_variables: Vec<HeaderVariable> = vec![];

//     let header_length: usize = 0;
//     for variable in 0..header_length
//     {
//         header_variables.push(HeaderVariable::new("key", "value"));
//     }

//     header_variables.push(HeaderVariable::new("encyption", "aes"));
//     header_variables.push(HeaderVariable::new("force_encryption", "t"));
//     header_variables.push(HeaderVariable::new("author", "8d1a0cfb13df4ca3bdb0e912be01863b"));
//     header_variables.push(HeaderVariable::new("target", "none"));
//     header_variables.push(HeaderVariable::new("channel", "20026f0a1c484f95a0063d148c8898f9"));
//     header_variables.push(HeaderVariable::new("channel_type", "text_message"));
//     header_variables.push(HeaderVariable::new("content_mime_type", "text/plain"));
//     header_variables.push(HeaderVariable::new("content_formatting", "none"));
//     header_variables.push(HeaderVariable::new("time_sent", "2023-06-25 12:25:22"));

//     /*
//      * SUPPORTED TYPES for `content_formatting`
//      * AAA Support (Virtually Required and officailly endorsed)
//      * - none (Plain Text)
//      * - rich-markdown (see DIM Markdown Specification)
//      * - wikitext
//      * - variables (INI Format)
//      *      Chosen because, even if its not your prefered format,
//      *      it's dead simple and does everything we need it to do.
//      *      it dosen't have a bunch of fancy stuff, just 
//      *      key = value ; comment
//      *      NOTE: comments with # are NOT ALLOWED!!
//      *
//      * AA Support (Probably some fancier clients, not offically endorsed)
//      * - commonmark
//      *
//      * A Support (Nieche/Ehh?)
//      * - universal-chess-interface
//      *
//      * E Support (Deprecated)
//      * - None!
//      *
//      * F Support (Actively Discouraged)
//      * - html - DIM Clients are not web browsers!!
//      * - <Any Code> - Use a code block in markdown!!
//      */

//     // DIM
//     let response = ResponsePacket::create(
//         String::from("1.0"),
//         200,
//         String::from("Serving"),
//         header_variables,
//         String::from("Manically laughs at the futility of life. Oh also I got DIM packets sorta being contructed!")
//         );

//     // let plaintext = cipher.decrypt(nonce, response.as_ref()).unwrap();

// //     match stream.write(&plaintext)
// //     {
// //         Ok(message) =>
// //         {
// //             trace!("Wrote to the TCP Stream");
// //         }

// //         Err(error) =>
// //         {
// //             error!("The TCP Stream write failed!
// // {INDENT}{CODE_START}connection_handler.rs::handle_connection(){ENDBLOCK}
// // {INDENT}Here we provide the compilers error:
// // {error} ");
// //             panic!("Why would the TCP stream flush panic !");
// //         }
// //     }

//     match stream.flush()
//     {
//         Ok(_message) =>
//         {
//             trace!("TCP Stream Flushed");
//         }

//         Err(error)  =>
//         {
//             error!("The TCP Stream flush failed!
// {INDENT}{CODE_START}connection_handler.rs::handle_connection(){ENDBLOCK}
// {INDENT}Here we provide the compilers error:
// {error} ");
//             panic!("Why would the TCP stream flush panic !");
//         }
//     }
// }

#[derive(Debug)]
struct HeaderVariable
{
    key: String,
    value: String,
}

impl HeaderVariable
{
    fn new(key: &str, value: &str) -> HeaderVariable
    {
        HeaderVariable
        {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

/// 🚧 Temperary
/// We will want this for ease of parsing incoming pacekets, but it is
/// not currently used.
#[allow(dead_code)]
struct RequestPacket
{
    version: String,
    request_type: String,
    request_target: String,
    header_flags: Vec<HeaderVariable>,

    message: String,
}

/// 📔 Note
/// The reason none of these variables is read is because this object exists
/// for easier packet construction. We make the packet, and then write to
/// the stream `.as_bytes()`!
#[allow(dead_code)]
struct ResponsePacket
{
    version: String,
    response_code: u16,
    response_message: String,
    header_flags: Vec<HeaderVariable>,

    message: String,
}

impl ResponsePacket
{
    fn create(version: String,
        response_code: u16,
        response_message: String,
        header_variables: Vec<HeaderVariable>,
        content: String) -> String
    {
        let response_header = format!("dim/{version} {code} {response_message}\n", code=response_code.to_string());
        let mut response_variables: String = String::from("");

        for variable in header_variables.iter()
        {
            trace!("variable: {:?}", variable);

            // I can't += this for some reason ...
            response_variables = format!("{pre}{key}={value};", pre=response_variables, key=variable.key, value=variable.value);
        }

        trace!("{:?}", response_variables);

        let packet = format!("{response_header}{response_variables}\n{content}");

        trace!("{:?}", packet);

        return packet.to_string();

        // return response_header.as_bytes() + response_variables.as_bytes() + response_message.as_bytes();
    }
}

//
// Tests!
//

#[cfg(test)]
mod tests
{
    use crate::connection_handler::ResponsePacket;
    use crate::connection_handler::HeaderVariable;

    #[test]
    fn no_header_variables()
    {
        let response_variables: Vec<HeaderVariable> = vec![];

        assert_eq!(
            "dim/1.0 200 Serving\nTest content",
            ResponsePacket::create(String::from("1.0"), 200, String::from("Serving"), response_variables, String::from("Test content"))
        )
    }
}
