use log::info;
use log::error;

struct Packet
{
    version: String,
    variables: Vec<PacketVariable>,
    body: String
}

struct PacketVariable (String, String);

impl PacketVariable
{
    fn deserialise() -> Vec<PacketVariable>
    {
        let result: Vec<PacketVariable> = Vec::new();

        // get the number of equals characters
        // get the number of semicolon characters
        // if "=" != ";" : Error

        // seperate into a vec, with the content from ther first 0A
        // byte to the first semicolon, then from semicolon to semicolon
        // removing them as you go.

        // for iterations (number of equals characters)
        // {
        //      run single_from()
        //      push PacketVariable to the vec
        // }

        return result;
    }

    fn single_from(string: &str) -> PacketVariable
    {
        // find the equals character
        // split the string into two strings: before and after the equals
        // (equals itself can be discarded)
        // return the struct with them seperated.
        return PacketVariable ("meow".to_string(), "purr".to_string());
    }
}

impl Packet
{
    fn from(version: &str, variables: &str, body: &str) -> Packet
    {
        return Packet
        {
            version: "1.0".to_string(),
            variables: Vec::from([PacketVariable::single_from("a")]),
            body: "purr".to_string()
        }
    }
}

enum ResourceIdentifierVariant
{
    Guid,
    Dim,
}

struct ResourceIdentifier
{
    variant: ResourceIdentifierVariant,
    path: Vec<String>
}

impl ResourceIdentifier
{
    fn from(variant: &str, body: &str) -> ResourceIdentifier
    {
        return ResourceIdentifier
        {
            variant: ResourceIdentifierVariant::Dim,
            path: Vec::from(["meow".to_string(), "purr".to_string()])
        }
    }
}

enum RequestMethod
{
    Get,
    Edit,
    Post,
    Remove
}

pub struct RequestPacket
{
    method: RequestMethod,
    resource: ResourceIdentifier,
    packet: Packet
}

impl RequestPacket
{
    pub fn deserialise(packet: &str) -> RequestPacket
    {
        // <Version> SP <RequestMethod> SP <ResourceIdentifier> LF (0A)
        // <Variables> LF (0A)
        // <Body (optional)>

        // split into 3 variables: 1 before the first 0A, then before
        // the next 0A, then everything after that.

        // line1.split_at(" ")
        // version = line1.0
        // request_type = line1.1 matched to the method
        // resource = lines1.2 resource (parse seperately)

        // line2 just gets thrown into PacketVariable::deserialise()

        // line3 is explict content that can be ignored most of the time, or copied exactly.
        // since there is no reason to run code from it, and never ever gets executed!

        let mut version: &str = "";
        let mut request_type: &str = "";
        let mut requested_resource: &str = "";
        let mut header_flags: &str = "";
        let mut message: &str = "";

        // Deserialize packet into structure for future use

        packet.to_string();

        // let packet = packet.split_at(packet.clone().find(" ").unwrap());
        // version = packet.0;

        // let packet = packet.1.split_at(packet.1.clone().find(" ").unwrap());
        // request_type = packet.0;

        // // TODO: Needs to handle GUIDs and usernames seperately as well as telling if group or user
        // let packet = packet.1.split_at(packet.1.clone().find(" ").unwrap());
        // requested_resource = packet.0;

        // Header flags

        // Message

        match request_type
        {
            // its ugly but we want to shadow request_type to save memory
            "GET"    => { let request_type: RequestMethod = RequestMethod::Get; },
            "POST"   => { let request_type: RequestMethod = RequestMethod::Post; },
            "EDIT"   => { let request_type: RequestMethod = RequestMethod::Edit; },
            "REMOVE" => { let request_type: RequestMethod = RequestMethod::Remove; },
            // _        => ResponsePacket::error_response(
            //     stream, "1.0",
            //     401,
            //     "Invalid Method",
            //     ""
            // ),
            _ => { error!("OOPSY DOOPSY ! connection/packets.rs line ~150") }
        }

        return RequestPacket
        {
            method: RequestMethod::Get,
            resource: ResourceIdentifier::from("guid", "nil"),
            packet: Packet::from("1.0", "NOT IMPLEMENETED YET", "NOT IMPLEMENTED YET")
        }
    }

    pub fn debug() -> RequestPacket
    {
        return RequestPacket
        {
            method: RequestMethod::Edit,
            resource: ResourceIdentifier
            {
                variant: ResourceIdentifierVariant::Dim,
                path: Vec::from([
                    "group".to_string(),
                    "name".to_string(),
                    "category".to_string(),
                    "channel".to_string(),
                    "thread".to_string(),
                    "message".to_string(),
                    "etc".to_string()])
            },
            packet: Packet::from("1.0", "NOT IMPLEMENETED YET", "NOT IMPLEMENTED YET")
        }
    }
}

pub struct ResponsePacket
{
    code: u16,
    message: String,
    packet: Packet
}

impl ResponsePacket
{
    pub fn debug() -> ResponsePacket
    {
        return ResponsePacket
        {
            code: u16::MAX,
            message: "DEBUG PACKET: DO NOT USE IN PROD!".to_string(),
            packet: Packet::from("1.0", "NOT IMPLEMENETED YET", "NOT IMPLEMENTED YET")
        }
    }
}