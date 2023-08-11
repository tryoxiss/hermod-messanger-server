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

enum RequestMethod
{
    Get,
    Edit,
    Post,
    Remove
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

pub struct RequestPacket
{
    method: RequestMethod,
    resource: ResourceIdentifier,
    packet: Packet
}

impl RequestPacket
{
    pub fn deserialise() -> RequestPacket
    {
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