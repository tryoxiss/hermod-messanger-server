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
    RequestPacket::deserialise("dim/1.0 GET groups/groupname/category/channel\n\nThis\nis my\n content!!");

    // the request can be returned in such a way that an error was found in process_incoming()
    let (request, stream) = process_incoming(stream);

    // if there was a problem in process_incoming(), construct the error.
    let response = handle_request(request);
    respond(response, stream)
}

fn process_incoming(stream: TlsStream<TcpStream>) -> (RequestPacket, TlsStream<TcpStream>)
{
    let request: RequestPacket = RequestPacket::debug();

    // body

    return (request, stream)
}

fn handle_request(request: RequestPacket) -> ResponsePacket
{
    let response: ResponsePacket = ResponsePacket::debug();

    return response;
}

fn respond(response_object: ResponsePacket, mut stream: TlsStream<TcpStream>) -> ()
{
    let response_string: String = "meow".to_string();

    match stream.write(response_string.as_bytes())
    {
        Ok(_)  => trace!("Wrote to TLS Stream!"),
        Err(e) => error!("Failed to write to TLS Stream!"),
    }
}