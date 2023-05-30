// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };

// enum Operation { 
//     Connect,
//     Create, 
//     Notify,
//     Edit,
//     Remove,
//     Delete,
//     Destroy,
//     Get
// }

// enum Address { 
//     DIMAddress,
//     IPv6Address,
// }

// struct Guid ( u128 );
// struct Signature ( u128 );

// struct IPv4 ( u8, u8, u8, u8 ); // ew                      (u8.u8.u8.u8)
// struct IPv6 ( u16, u16, u16, u16, u16, u16, u16, u16 ); // (u16:u16:u16:u16:u16:u16:u16:u16)

// trait Packet {
//     fn edition(&self) -> String;

//     fn target(&self) -> Vec<Guid>;

//     fn validate_signature(&self, _public_key: String) -> bool;
// }

// struct ConnectRequest { 
//     edition: String,

//     target: Address,

//     signature: Signature
// }

// struct GetRequest { 
//     edition: String,

//     target: Vec<Guid>,

//     signature: Signature
// }

// struct CreateRequest { 
//     edition: String,

//     target: Guid, // channel 
//     content: String,

//     signature: Signature
// }

// struct EditRequest { 
//     edition: String,

//     target: Guid, // object (message, wiki page, etc)
//     content: String,

//     signature: Signature
// }

// struct RemoveRequest { 
//     edition: String,

//     target: String,
// }

// struct DeleteRequest { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct DestroyRequest { 
//     edition: String,

//     target: String,

//     signature: Signature
// }

// struct NotifyRequest { 
//     edition: String,

//     target: String,
//     content: String,

//     signature: Signature
// }

// impl Packet for GetRequest { 
//     fn edition(&self) -> String { 
//         return self.edition;
//     }

//     fn target(&self) -> Vec<Guid> { 
//         return self.target;
//     }

//     fn validate_signature(&self, _public_key: Key) -> bool {
//         return false;
//     }
// }

// fn parse_packet() { 

// }

// pub fn handle_request() { 
//     let key = Aes256GcmSiv::generate_key(&mut OsRng);
//     let cipher = Aes256GcmSiv::new(&key);
//     let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//     let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref()) ?;
//     let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()) ?;
//     assert_eq!(&plaintext, b"plaintext message");

//     log!("Cuddles");
// }