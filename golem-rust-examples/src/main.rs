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

#[golem()]
pub enum IpAddrEmpty {
    V4,
    V6,
}

#[golem()]
pub struct BidderId {
    pub bidder_id: std::result::Result<IpAddrEmpty, String>,
    pub verified: bool,
}

#[golem()]
fn create_bidder(full_name: String, address: Vec<String>, age: Option<u16>) -> BidderId {
    BidderId {
        bidder_id: Result::Err("hello".to_owned()),
        verified: false,
    }
}

#[golem()]
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
