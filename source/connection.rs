mod packets;
use packets::*;

use std::net::TcpStream;
use std::io::prelude::*;

use native_tls::TlsStream;

use log::trace;
use log::error;

pub fn handle(stream: TlsStream<TcpStream>)
{
    // test pourposes:
    RequestPacket::from("dim/1.0 GET groups/groupname/category/channel\n\nThis\nis my\n content!!");

    // Option::None = Malformed Packet
    // Option::Some(packet) = Successful
    let (request, stream) = process_incoming(stream);

    let response: ResponsePacket;
    match request
    {
        Some(packet) =>
        {
            response = handle_request(packet);
        },
        None =>
        {
            response = ResponsePacket::error(401, "Malformed Packet");
        }
    }

    respond(response, stream)
}

fn process_incoming(stream: TlsStream<TcpStream>) -> (Option<RequestPacket>, TlsStream<TcpStream>)
{
    // println!("{:?}", &stream); // doesn't contain request??

    // let request: &str = somehow get the request;
    // let request: RequestPacket = RequestPacket::from(request);

    let request = RequestPacket::debug();

    match request
    {
        Option::Some(packet) => { return (Option::Some(packet), stream); }
        Option::None => { return { return (Option::None, stream) }; }
    }
}

fn handle_request(request: RequestPacket) -> ResponsePacket
{
    let response: ResponsePacket = ResponsePacket::debug();

    // identify privlage
    //   Privlaged? =>
    //      Find requested data
    //      return requested data
    //   Not Privlaged? => Why?
    //      Proxy Auth Requires
    //      Not authenticated
    //      Unauthorised
    //      Forbidden
    //      Non Existent?
    //          => Return appropriate error, except non-existent which
    //             always returns forbidden.

    return response;
}

fn respond(response_object: ResponsePacket, mut stream: TlsStream<TcpStream>) -> ()
{
    // TODO: Parse the packet object into the actual packet string.
    let response_string: String = "meow".to_string();

    match stream.write(response_string.as_bytes())
    {
        Ok(_) => trace!("Wrote to TLS Stream!"),
        Err(error) => error!("Failed to write to TLS Stream! {}", error),
    }
}