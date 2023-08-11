/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 *  This file is part of:         https://github.com/tryoxiss/bonfire-server *
 *  Hermod Messanger                          https://en.ourdomain.ext/docs/ *
 *                                                                           *
 *  Copyright (C) 2023—present : Hermod Messenger Contributers (AUTHORS.md)  *
 *                                                                           *
 *  This program is free software: you can redistribute it and/or modify     *
 *  it under the terms of the GNU Affero General Public License version 3    *
 *  as published by the Free Software Foundation.                            *
 *                                                                           *
 *  This program is distributed in the hope that it will be useful,          *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of           *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the            *
 *  GNU Affero General Public License version 3 for more details.            *
 *                                                                           *
 *  You should have received a copy of the GNU Affero General Public License *
 *  along with this program.  If not, see:                                   *
 *    https://www.gnu.org/licenses/agpl-3.0                                  *
 *                                                                           *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
/* @authors
 *  *  tryoxiss 'madeline'
 *  *  khaim0919
 */
//

// use guid::GUID;

// This should be defined somewhere else
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

enum RequestType
{
    Get,
    Edit,
    Post,
    Remove
}

/// 🚧 Temperary
/// We will want this for ease of parsing incoming pacekets, but it is
/// not currently used.
#[allow(dead_code)]
struct RequestPacket
{
    version: String,
    request_type: RequestType,
    requested_resource: String,
    header_flags: Vec<HeaderVariable>,

    message: String,
}

impl RequestPacket
{
    fn parse( /* mut stream: TlsStream<TcpStream>, */ packet: &str)
    {
        let mut version: &str = "";
        let mut request_type: &str = "";
        let mut requested_resource: &str = "";
        let mut header_flags: &str = "";
        let mut message: &str = "";

        // Deserialize packet into structure for future use

        packet.to_string();
        let packet = packet.split_at(packet.clone().find(" ").unwrap());
        version = packet.0;

        let packet = packet.1.split_at(packet.1.clone().find(" ").unwrap());
        request_type = packet.0;

        // TODO: Needs to handle GUIDs and usernames seperately as well as telling if group or user
        let packet = packet.1.split_at(packet.1.clone().find(" ").unwrap());
        requested_resource = packet.0;

        // Header flags

        // Message

        match request_type
        {
            // its ugly but we want to shadow request_type to save memory
            "GET"    => { let request_type: RequestType = RequestType::Get; },
            "POST"   => { let request_type: RequestType = RequestType::Post; },
            "EDIT"   => { let request_type: RequestType = RequestType::Edit; },
            "REMOVE" => { let request_type: RequestType = RequestType::Remove; },
            // _        => ResponsePacket::error_response(
            //     stream, "1.0",
            //     401,
            //     "Invalid Method",
            //     ""
            // ),
            _ => {println!("OOPSY DOOPSY ! deserialization.rs line ~100")}
        }
    }
}

//
// Tests!
//

#[cfg(test)]
mod tests
{

// user: @username, guid
// group: guid/channel, guid/guid

    use crate::packet_handler::RequestPacket;

    #[test]
    fn deserialization()
    {
        let input_packet = "dim/1.0 GET user:ffb6735ac32f4f6caefb265352d87f6f
encyption=aes;force_encryption=t;author=8d1a0cfb13df4ca3bdb0e912be01863b;target=none;channel=20026f0a1c484f95a0063d148c8898f9;channel_type=text_message;content_mime_type=text/plain;content_formatting=none;time_sent=2023-06-25 12:25:22;
Manically laughs at the futility of life. Oh also I got DIM packets sorta being contructed!";

        let request_packet = RequestPacket::parse(input_packet);

        println!("{:?}", request_packet);
    }
}

// struct Resource
// {
//     name: String,
//     data: Vec<u8>, // byte vector
// }

enum ResourceLocation
{
    ResourceHierLocation(String),
    ResourceGuidLocation(u128), // GUID
}

trait Resource
{
    fn load() {}
}