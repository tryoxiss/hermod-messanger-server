

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

struct Connect { 
    edition: String,

    target: Address,

    signature: Signature
}

struct Get { 
    edition: String,

    target: Vec,

    signature: Signature
}

struct Create { 
    edition: String,

    target: Guid, // channel 
    content: String,

    signature: Signature
}

struct Edit { 
    edition: String,

    target: Guid, // object (message, wiki page, etc)
    content: String,

    signature: Signature
}

struct Remove { 
    edition: String,

    target: String,
}

struct Delete { 
    edition: String,

    target: String,

    signature: Signature
}

struct Destroy { 
    edition: String,

    target: String,

    signature: Signature
}

struct Notify { 
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