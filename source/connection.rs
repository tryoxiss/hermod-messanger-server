mod packets;
use packets::*;

use std::net::TcpStream;
use std::io::prelude::*;

use native_tls::TlsStream;

use log::trace;
use log::error;

pub fn handle(stream: TlsStream<TcpStream>)
{
    // test pourposes: (REMOVE THE UNWRAP!) (even tho this is constant and therefore always safe)
    RequestPacket::from("dim/1.0 GET groups/groupname/category/channel\nencryption=aes;force_encryption=t;\nThis\nis my\n content!!\n   UWU\n").unwrap();

    // Option::None = Malformed Packet
    // Option::Some(packet) = Successful
    let (request, stream) = process_incoming(stream);

    let response: ResponsePacket;
    match request
    {
        Ok(packet) =>
        {
            response = handle_request(packet);
        },
        Err(RequestError::Unknown) => response = ResponsePacket::error(401, "Malformed Packet"),
        Err(RequestError::HeaderTooLong) => response = ResponsePacket::error(401, "Malformed Packet"),
        Err(RequestError::InvalidMethod) => response = ResponsePacket::error(405, "Invalid Method"),

    }

    respond(response, stream)
}

#[allow(unused)] // yes the stream will need to be mutable later.
fn process_incoming(mut stream: TlsStream<TcpStream>) -> (Result<RequestPacket, RequestError>, TlsStream<TcpStream>)
{
    // println!("{:?}", &stream); // doesn't contain request??

    // let request: &str = somehow get the request;
    // let request: RequestPacket = RequestPacket::from(request);

    // // 2^16 bytes, should be more than plenty.
    // const MAX_SIZE: usize = 65_536;

    // // we want to read the exact amount of content but EOF is deprecated I think...

    // let mut buffer: Vec<u8> = Vec::new();

    // // read_to_end uses the (deprecated?) EOF!!
    // let stream_content = stream.read_to_end(&mut buffer);

    // println!("{:?}", stream_content.unwrap());
    // println!("{:?}", String::from_utf8(buffer));

    let request = RequestPacket::debug();
    // let request = RequestPacket::from();

    match request
    {
        Result::Ok(packet) => return (Result::Ok(packet), stream),
        Result::Err(e) => return (Result::Err(e), stream),
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