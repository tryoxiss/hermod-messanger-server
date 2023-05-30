

enum Operation { 
    Connect,
    Create, 
    Notify,
    Edit,
    Remove,
    Delete,
    Destroy,
    Get
}

enum Address { 
    DIMAddress,
    IPv6Address,
}

struct Guid ( u128 );
struct Signature ( u128 );

struct IPv4 ( u8, u8, u8, u8 ); // ew                      (u8.u8.u8.u8)
struct IPv6 ( u16, u16, u16, u16, u16, u16, u16, u16 ); // (u16:u16:u16:u16:u16:u16:u16:u16)

trait Packet {
    fn edition(&self) -> String;

    fn target(&self) -> String;
}

struct ConnectRequest { 
    edition: String,

    target: Address,

    signature: Signature
}

struct GetRequest { 
    edition: String,

    target: Vec,

    signature: Signature
}

struct CreateRequest { 
    edition: String,

    target: Guid, // channel 
    content: String,

    signature: Signature
}

struct EditRequest { 
    edition: String,

    target: Guid, // object (message, wiki page, etc)
    content: String,

    signature: Signature
}

struct RemoveRequest { 
    edition: String,

    target: String,
}

struct DeleteRequest { 
    edition: String,

    target: String,

    signature: Signature
}

struct DestroyRequest { 
    edition: String,

    target: String,

    signature: Signature
}

struct NotifyRequest { 
    edition: String,

    target: String,
    content: String,

    signature: Signature
}



impl Packet for Get { 
    fn edition() { 
        return self.edition;
    }
}