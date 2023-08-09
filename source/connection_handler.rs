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

use crate::{ENDBLOCK, CODE_START, INDENT};

pub fn handle_connection(mut stream: TlsStream<TcpStream>)
{
    // const MAX_PACKET_LENGTH: usize = 1048576;
    // let mut buffer = [0; MAX_PACKET_LENGTH];

    // // we return an error if the packet is too long.
    // if buffer[MAX_PACKET_LENGTH - 1] != "\x00".as_bytes()[0]
    // {
    //     let mut response_variables: Vec<HeaderVariable> = vec![];

    //     response_variables.push(HeaderVariable::new("encyption", "aes"));
    //     response_variables.push(HeaderVariable::new("force_encryption", "t"));

    //     let response: String = ResponsePacket::create(
    //         "1.0",
    //         410,
    //         "Payload Too Large",
    //         response_variables,
    //         "max_length=1_048_575 ; Our maximum packet length is 1_048_575 bytes (1 MiB - 1 byte). If your content is larger than this, please use a packet series. You can do this by adding the `set=<u64>;`, and `index=<u64>` variable in the header to designate thier order. Alternatively, you may choose to load media through alternate sources such as HTTPS."
    //         );

    //     stream.write(&response.as_bytes())
    //         .expect("Failed to write to TCP Stream!");

    //     return;
    // }

    // // Process incoming packet
    // // -- TODO:

    // // Process Request
    // // -- TODO:

    // // return response packet to TlsStream
    // let response_variables = create_header_variables();

    // the request can be returned in such a way that an error was found in process_incoming()
    let (request, stream) = process_incoming(stream);

    // if there was a problem in process_incoming(), construct the error.
    let response = handle_request(request);
    return_result(response, stream)

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
}

fn create_header_variables() -> Vec<HeaderVariable>
{
    let mut header_variables: Vec<HeaderVariable> = vec![];

    let header_length: usize = 0;
    for variable in 0..header_length
    {
        header_variables.push(HeaderVariable::new("key", "value"));
    }


    // Use Aes-Gcm-Siv for Client-to-Server (aka not Peer-to-Peer)
    // ---
    // We can use a CSPRNG or Crypographically Secure Psudo-Random Number
    // Generator for encryption values. We will use ChaCha20-poly1305 since
    // - It can produce 1.8gb of randomness every seccond, making it far from
    //   a bottleneck.
    // - initalises fast (but startup times are not very important)
    // - only uses 136 bytes of perpetual memory 
    // - has been [deeply analyised](ChaCha20Analysis) 
    // 
    // [ChaCha20Analysis]: https://datatracker.ietf.org/doc/html/rfc7539#section-1
    // (Same as Above)   : https://www.cryptrec.go.jp/exreport/cryptrec-ex-2601-2016.pdf
    // (Summary)         : https://en.wikipedia.org/wiki/ChaCha20-Poly1305
    // ---

    header_variables.push(HeaderVariable::new("encyption", "aes"));
    header_variables.push(HeaderVariable::new("force_encryption", "t"));
    header_variables.push(HeaderVariable::new("author", "8d1a0cfb13df4ca3bdb0e912be01863b"));
    header_variables.push(HeaderVariable::new("target", "none"));
    header_variables.push(HeaderVariable::new("channel", "20026f0a1c484f95a0063d148c8898f9"));
    header_variables.push(HeaderVariable::new("channel_type", "text_message"));
    header_variables.push(HeaderVariable::new("content_mime_type", "text/plain"));
    header_variables.push(HeaderVariable::new("content_formatting", "none"));
    header_variables.push(HeaderVariable::new("time_sent", "2023-06-25 12:25:22"));

    return header_variables;
}

fn process_incoming(mut stream: TlsStream<TcpStream>) -> (RequestPacket, TlsStream<TcpStream>)
{
    return (RequestPacket::new(), stream);
}

fn handle_request(request: RequestPacket) -> ResponsePacket
{
    let compliance_vars = create_header_variables();
    
    return ResponsePacket
    {
        version: "1.0".to_string(),
        response_code: 501,
        header_flags: compliance_vars,
        response_message: "Not Implemented".to_string(),
        message: "Sorry :/".to_string()
    }
}

fn return_result(packet: ResponsePacket, mut stream: TlsStream<TcpStream>) -> ()
{
    let response_variables = create_header_variables();

    let response = ResponsePacket::create(
        "1.0",
        200,
        "Serving",
        response_variables,
        "Manically laughs at the futility of life. Oh also I got DIM packets sorta being contructed!"
    );

    // Writes some prefix of the byte string, not necessarily all of it.
    stream.write(response.as_bytes()).unwrap();

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
        }
    }

    return ()
}

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

enum RequestMethod
{
    Get,
    Edit,
    Post,
    Remove,

    // not an actual method, simply used to indicate that the content
    // is to be taken as a parsable error and not literal content.
    InternalServerError
}

struct RequestPacket
{
    version: String,
    method: RequestMethod,
    header_flags: Vec<HeaderVariable>,
    resource: String,     // MAYBE REQUEST DEPTH? So you can like get a group without getting all channels and messages in it.

    body: String,
}

impl RequestPacket
{
    fn new() -> RequestPacket
    {
        let compliance_vars = create_header_variables();

        return RequestPacket
        {
            version: "1.0".to_string(),
            method: RequestMethod::Get,
            header_flags: compliance_vars,
            resource: "hiss".to_string(),
            body: "meow".to_string()
        }
    }
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
    header_flags: Vec<HeaderVariable>,
    response_message: String,

    message: String,
}

impl ResponsePacket
{
    fn create(version: &str,
        response_code: u16,
        response_message: &str,
        header_variables: Vec<HeaderVariable>,
        content: &str) -> String
    {
        let response_header = format!("dim/{version} {code} {response_message}\n",
            version=version.to_string(),
            code=response_code.to_string(),
            response_message=response_message.to_string()
        );
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

    fn error_response(mut stream: TlsStream<TcpStream>, version: &str, response_code: u16, response_message: &str, content: &str)
    {
        let mut header_variables: Vec<HeaderVariable> = vec![];

        header_variables.push(HeaderVariable::new("encyption", "aes"));
        header_variables.push(HeaderVariable::new("force_encryption", "t"));
        // header_variables.push(HeaderVariable::new("author", "8d1a0cfb13df4ca3bdb0e912be01863b"));
        // header_variables.push(HeaderVariable::new("target", "none"));
        // header_variables.push(HeaderVariable::new("channel", "20026f0a1c484f95a0063d148c8898f9"));
        // header_variables.push(HeaderVariable::new("channel_type", "text_message"));
        header_variables.push(HeaderVariable::new("content_mime_type", "text/plain"));
        header_variables.push(HeaderVariable::new("content_formatting", "none"));
        header_variables.push(HeaderVariable::new("time_sent", "2023-06-25 12:25:22"));
        // -- send an error response. Do not return.
        // use a set of pre-existing 

        let response = ResponsePacket::create(
            version,
            response_code,
            response_message,
            header_variables,
            content
        );

        stream.write(response.as_bytes()).unwrap();
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
            ResponsePacket::create("1.0", 200, "Serving", response_variables, "Test content")
        )
    }
}
