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

use log::{debug, error, info, trace, warn};
use log4rs;

use std::net::TcpStream;
use std::io::prelude::*;

pub fn handle_connection(mut stream: TcpStream)
{
    // [0; u16::MAX] (we just cant put a u16 in there because type mismacth.)
    // A u16::MAX is the maximum packet length. Anything longer must be sent 
    // in multiple packets consecurtive packets. This is specified with a 
    // header flag `packet_series="Name";` and `packet_index=u32`
    //
    // This makes the maximum data transfer be:
    // 
    // Approx: 281,470,681,700,000 Bytes
    // Approx: or 256 Terrabytes
    // 
    // be the maximum data transfer in a single packet, but realistically because
    // of headers and such its only ...
    // 
    // Approx: 214,748,364,700,000 bytes
    // Approx: 195.31 Terrabytes
    // 
    // But if you need to transfer that much data in one sequence I don't think
    // this is the protocol to do it with, and if it is, then you can just have
    // something that specifies this continues a previous packet series. 
    // 
    // The packet size limit is to ensure a timely connection for everyone, so
    // a thread dosen't get stuck on just one request for a long time (making 
    // it easier to DoS or DDoS), it will still get around to other requests 
    // reasonably quickly before returning to processing the bigger request.
    // 
    // THIS TECHNHECALLY ISN'T DEFINED IN THE SPEC!!! IT DEFINES THAT YOU **MAY**
    // HAVE A PAYLOAD SIZE LIMIT, AND WAYS TO SPLIT IT INTO MULTIPLE SEPERATE
    // PAYLOADS, BUT DOES NOT IMPOSE ANY PARTICULAR SIZE LIMITATION!!
    // 
    // THIS JUST HAPPENS TO BE A CONVIENIENT AND REASONABLE SIZE LIMITATION!!
    // 
    // This also means machines don't need as much memory per thread. 
    // (Each thread takes about 65.535 or 66 KB of memory at most)

    // TODO: With this current implementation we simply drop extra bytes rather
    // than returning 411 Payload_Too_Large !!

    warn!("When a payload is too lagre (over {} bytes), we simply 
             drop the extra bytes rather than returning a 411 Payload_Too_Large!", u16::MAX);

    let mut buffer = [0; 65_535];

    trace!("MEOW");

    stream.read(&mut buffer).unwrap();
    warn!("Stream is read and then unwrapped! Don't unwrap!");

    // Valid HTTP
    // let response = format!("HTTP/1.1 200 OK\r\n\r\n Connection established! \n Bufer_Length: {}; \n Packet_Length: {};", buffer.len(), "Unknown");

    let mut response_variables: Vec<HeaderFlag> = vec![];

    response_variables.push(HeaderFlag::new(String::from("encyption"), String::from("aes")));
    response_variables.push(HeaderFlag::new(String::from("you_are_cute"), String::from("t")));

    // DIM
    let response = ResponsePacket::create(
        String::from("1.0"),
        200,
        String::from("Serving"), 
        response_variables, 
        String::from("Monically laughs at the futility of life. Oh also I got DIM packets sorta being contructed!"));

//     // DIM
//     let response = format!(
// "dim/1.0 200 Serving\nHeaderFlag=True;\n---
// Bufer_Length: {};
// Packet_Length: {};
// ---", 
//     buffer.len(),
//     "Unknown");

    stream.write(response.as_bytes()).unwrap();

    warn!("Stream is written to and then unwrapped! Don't unwrap!");

    stream.flush().unwrap();
    warn!("Stream flush is unwrapped! Don't unwrap!");
}

#[derive(Debug)]
struct HeaderFlag 
{
    key: String,
    value: String,
}

impl HeaderFlag 
{
    fn new(key: String, value: String) -> HeaderFlag
    {
        HeaderFlag
        {
            key: key,
            value: value,
        }
    }
}

struct RequestPacket 
{
    version: String,
    request_type: String,
    request_target: String,
    header_flags: Vec<HeaderFlag>,

    message: String,
}

// We need this so that I can impl it
// It currently does not do anything
// I may remove this and make it just like CreateResponsePacket or something
struct ResponsePacket 
{
    version: String,
    response_code: u16,
    response_message: String,
    header_flags: Vec<HeaderFlag>,

    message: String,
}

impl ResponsePacket 
{
    fn create(version: String,
        response_code: u16,
        response_message: String,
        header_variables: Vec<HeaderFlag>,
        content: String) -> String
    {
        let response_header = format!("dim/{version} {} {response_message}\n", response_code.to_string());
        let mut response_variables: String = String::from("");

        for variable in header_variables.iter()
        {
            trace!("variable: {:?}", variable);

            // I can't += this for some reason ... 
            response_variables = format!("{pre}{key}={value};", pre=response_variables, key=variable.key, value=variable.value);
        }

        trace!("{:?}", response_variables);

        let packet = format!("{response_header}{response_variables}\n---\n{content}\n---");

        trace!("{:?}", packet);

        return packet.to_string();

        // return response_header.as_bytes() + response_variables.as_bytes() + response_message.as_bytes();
    }
}