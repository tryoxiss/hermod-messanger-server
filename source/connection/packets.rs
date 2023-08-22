use log::error;
use log::info;

#[derive(Debug)]
#[derive(PartialEq)]
struct Packet
{
    version: String,
    variables: Vec<PacketVariable>,
    body: String
}

#[derive(Debug)]
#[derive(PartialEq)]
struct PacketVariable (String, String);

impl PacketVariable
{
    fn from(string: &str) -> Vec<PacketVariable>
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
            variables: Vec::from([PacketVariable::single_from("k=v;")]),
            body: "purr".to_string()
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct ResourceIdentifier
{
    path: Vec<String>
}

impl ResourceIdentifier
{
    fn from(body: &str) -> ResourceIdentifier
    {
        return ResourceIdentifier
        {
            path: Vec::from(["meow".to_string(), "purr".to_string()])
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum RequestMethod
{
    Get,
    Edit,
    Post,
    Remove
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct RequestPacket
{
    method: RequestMethod,
    resource: ResourceIdentifier,
    packet: Packet
}

#[derive(Debug)]
pub enum RequestError
{
    Unknown,          // ????
    HeaderTooLong,    // The header had more than 3 fields (<version> <method> <message>)
    InvalidMethod,    // Method did not match GET, EDIT, POST, or REMOVE.
}

impl RequestPacket
{
    pub fn from(packet: &str) -> Result<RequestPacket, RequestError>
    {
        // <Version> SP <RequestMethod> SP <ResourceIdentifier> LF (0A)
        // <Variables> LF (0A)
        // <Body (optional)>

        // split into 3 variables: 1 before the first 0A, then before
        // the next 0A, then everything after that.

        // line2 just gets thrown into PacketVariable::deserialise()

        // Deserialize packet into structure for future use

        packet.to_string();

        // only the first two lines matter, after thats its `body`
        let mut lines = ("", "");
        let mut iterations: u8 = 0;
        let mut body: String = "".to_string();

        // split into lines
        for part in packet.split("\n")
        {
            if iterations == 0
            {
                lines.0 = part;
            }
            else if iterations == 1
            {
                lines.1 = part;
            }
            else
            {
                // I had this working, then my solution got deleted and I
                // forgot how I did it. This is a much worse solution than
                // what I had though.
                body = format!("{}{}\n", body, part);
            }

            iterations += 1;
        }

        let body = body.as_str();

        // Dosen't this just prevent messages longer than a line? Also the RequestError is wrong.
        // if iterations <= 2
        // {
        //     return Result::Err(RequestError::HeaderTooLong);
        // }

        // if we ever return an empty thing, we got problems.
        // but I believe this is safe since we confirm that iterations is 2.
        let mut method = "";
        let mut version = "";
        let mut resource: ResourceIdentifier;

        // parse the first line
        let mut iterations: u8 = 0;
        for part in lines.0.split(" ")
        {
            if iterations == 0
            {
                version = part;
            }
            else if iterations == 1
            {
                method = part;
            }
            else if iterations == 2
            {
                resource = ResourceIdentifier::from(part);
            }

            iterations += 1;
        }

        if iterations != 3 // for some reason a valid one has 3 iterations, not 2.
        {
            return Err(RequestError::HeaderTooLong);
        }

        match method
        {
            // its ugly but we want to shadow method to save memory
            "GET"    => { let method: RequestMethod = RequestMethod::Get; },
            "POST"   => { let method: RequestMethod = RequestMethod::Post; },
            "EDIT"   => { let method: RequestMethod = RequestMethod::Edit; },
            "REMOVE" => { let method: RequestMethod = RequestMethod::Remove; },
            _ =>
            {
                error!("Invalid Method! connection/packets.rs line ~150");
                return Err(RequestError::InvalidMethod);
            }
        }

        let variables = PacketVariable::from(lines.1);

        return Result::Ok(RequestPacket
        {
            method: RequestMethod::Get,
            resource: ResourceIdentifier::from("nil"),
            packet: Packet::from(version, "NOT IMPLEMENETED YET", "NOT IMPLEMENTED YET")
        })
    }

    /// Never returns None, its an Option to make it the same interface
    /// for debug pourposes.
    pub fn debug() -> Result<RequestPacket, RequestError>
    {
        return Result::Ok(RequestPacket
        {
            method: RequestMethod::Edit,
            resource: ResourceIdentifier
            {
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
        })
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
            packet: Packet::from("1.0", "TODO: NOT IMPLEMENETED YET", "TODO: NOT IMPLEMENTED YET")
        }
    }

    pub fn error(code: u16, message: &str) -> ResponsePacket
    {
        return ResponsePacket
        {
            code: code,
            message: message.to_string(),
            packet: Packet::from("1.0", "TODO: NOT IMPLEMENETED YET", "TODO: NOT IMPLEMENTED YET")
        }
    }
}

#[cfg(test)]
mod request_tests
{
    use super::ResourceIdentifier;
    use super::RequestPacket;
    use super::Packet;
    use super::RequestMethod;

    #[test]
    fn get_group_guid_guid_message()
    {
        assert_eq!(
            RequestPacket::from("dim/1.0 GET group/category/channel/message\n\nThis is my\n Content\n   Meow!").unwrap(),
            RequestPacket
            {
                method: RequestMethod::Get,
                resource: ResourceIdentifier
                {
                    path: Vec::from(["group".to_string(), "category".to_string(), "channel".to_string(), "message".to_string()])
                },
                packet: Packet
                {
                    version: "1.0".to_string(),
                    variables: Vec::from([]),
                    body: "This is my\n Content\n   Meow!".to_string()
                }
            }
        );
    }
}