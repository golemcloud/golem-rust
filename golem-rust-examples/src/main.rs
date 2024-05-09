golem_gen!();

use golem_rust::*;
use linkme::distributed_slice;

fn main() {}

#[golem()]
struct Person {
    name: String,
    address: Address,
}

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

#[golem()]
enum VariantTest {
    V1(String, u32),
    V2(Vec<String>),
}

pub enum IpAddrEmpty {
    V4,
    V6,
}

pub struct BidderId {
    pub bidder_id: std::result::Result<IpAddrEmpty, String>,
    pub verified: bool,
}

// impl HasWitMetadata for IpAddrEmpty {
//     const IDENT: &'static str = "IpAddrEmpty";

//     const WIT: &'static WitMeta = &WitMeta::Enum(EnumMeta {
//         name: Ident("IpAddrEmpty"),
//         variants: &[ Ident("V4"), Ident("V6")],
//     });
// }

// impl HasWitMetadata for BidderId {
//     const IDENT: &'static str = "BidderId";

//     const WIT: &'static WitMeta = &WitMeta::Record(RecordMeta {
//         name: Ident("BidderId"),
//         fields: &[
//             ("bidder_id", Result::<IpAddrEmpty, String>::WIT),
//             ("verified", bool::WIT),
//         ]
//     });
// }

// struct CreateBidder {}

// impl HasWitMetadata for CreateBidder {
//     const IDENT: &'static str = "create_bidder";

//     const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
//         name: Ident("create_bidder"),
//         args: &[],
//         result: &WitMeta::Tuple(&[BidderId::WIT, f32::WIT]),
//     });
// }

//#[golem()]
fn create_bidder(full_name: String, address: &[String], age: Option<u16>) -> (BidderId, f32) {
    (
        BidderId {
            bidder_id: Result::Err("hello".to_owned()),
            verified: false,
        },
        35.6,
    )
}

//#[golem()]
fn get_address() -> Address {
    Address {
        street: Option::Some("".to_owned()),
        city: Some(Ok("".to_owned())),
        state: "".to_owned(),
        zip: "".to_owned(),
        color: Color::BLUE,
    }
}

// struct GetAddress {}

// impl HasWitMetadata for GetAddress {
//     const IDENT: &'static str = "get_address";

//     const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
//         name: Ident("Address"),
//         args: &[],
//         result: Address::WIT,
//     });
// }

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
