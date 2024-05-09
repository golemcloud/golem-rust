golem_gen!();

use golem_rust::*;
use linkme::distributed_slice;

fn main() {}

// #[golem()]
// struct Person {
//     name: String,
//     address: Address,
// }

#[golem()]
struct Address {
    street: Option<String>,
    city: Option<Result<String, String>>,
    state: String,
    zip: String,
    color: Color,
}

#[golem()]
enum Color {
    RED,
    GREEN,
    BLUE,
}

// type Addy = string;
struct Addy(String);

// #[golem()]
// fn get_address() -> Address {
//     Address {
//         street: "".to_owned(),
//         city: "".to_owned(),
//         state: "".to_owned(),
//         zip: "".to_owned(),
//     }
// }

//struct GetAddress {}

//#[golem()]
fn get_address() -> Address {
    Address {
        street: Option::Some("".to_owned()),
        city: Some(Ok("".to_owned())),
        state: "".to_owned(),
        zip: "".to_owned(),
        color: Color::BLUE
    }
}

enum IpAddrEmpty {
    V4,
    V6,
}

impl HasWitMetadata for IpAddrEmpty {
    const IDENT: &'static str = "IpAddrEmpty";

    const WIT: &'static WitMeta = &WitMeta::Enum(EnumMeta {
        name: Ident("IpAddrEmpty"),
        variants: &[ Ident("V4"), Ident("V6")],
    });
}

pub struct BidderId {
    pub bidder_id: std::result::Result<IpAddrEmpty, String>,
    pub verified: bool,
}

// impl <T, E> HasWitMetadata for Result<T,E>
//     where
//         T: HasWitMetadata,
//         E: HasWitMetadata {
//     const IDENT: &'static str = "Result<T,E>";

//     const WIT: &'static WitMeta = &WitMeta::Empty;

// }

impl HasWitMetadata for BidderId {
    const IDENT: &'static str = "BidderId";

    const WIT: &'static WitMeta = &WitMeta::Record(RecordMeta {
        name: Ident("BidderId"),
        fields: &[
            ("bidder_id", Result::<IpAddrEmpty, String>::WIT),
            ("verified", bool::WIT),
        ]
    });
}

//#[golem()]
fn create_bidder(full_name: String, address: &[String], age: Option<u16>) -> (BidderId, f32) {
    (BidderId {
        bidder_id: Result::Err("hello".to_owned()),
        verified: false
    }, 35.6)
}

struct CreateBidder {}

// impl HasWitMetadata for CreateBidder {
//     const IDENT: &'static str = "create_bidder";

//     const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
//         name: Ident("create_bidder"),
//         args: &[],
//         result: &WitMeta::Tuple(&[BidderId::WIT, f32::WIT]),
//     });
// }

// impl HasWitMetadata for GetAddress {
//     const IDENT: &'static str = "get_address";

//     const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
//         name: Ident("Address"),
//         args: &[],
//         result: Address::WIT,
//     });
// }

// impl HasWitMetadata for Address {
//     const IDENT: &'static str = "Address";

//     const WIT: &'static WitMeta = &WitMeta::Struct(StructMeta {
//         name: Ident("Address"),
//         fields: &[
//             ("street", String::WIT),
//             ("city", String::WIT),
//             ("state", String::WIT),
//             ("zip", String::WIT),
//         ],
//     });
// }

// impl HasWitMetadata for Person {
//     const IDENT: &'static str = "Person";

//     const WIT: &'static WitMeta = &WitMeta::Struct(StructMeta {
//         name: Ident("Person"),
//         fields: &[("name", String::WIT), ("address", Address::WIT)],
//     });
// }

// #[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
// static PERSON_WIT: fn() -> &'static WitMeta = || Person::WIT;

// #[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
// static ADDRESS_WIT: fn() -> &'static WitMeta = || Address::WIT;

// #[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
// static FUN_WIT: fn() -> &'static WitMeta = || GetAddress::WIT;

#[test]
fn test_iter() {
    ALL_WIT_TYPES_FOR_GOLEM.iter().for_each(|f| {
        let wit_meta = f();
        use WitMeta::*;

        // let to_print = match wit_meta {
        //     Record(struct_meta) => println!("STRUCT {}", struct_meta.name.0),
        //     Function(function_meta) => println!("FUNCTION {}", function_meta.name.0),
        //     _ => println!("todo implement"),
        // };

        println!("{wit_meta:#?}");
        println!("\n")
    });
}
