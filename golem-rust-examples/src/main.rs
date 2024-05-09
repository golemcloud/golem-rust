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

struct GetAddress {}

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

// #[test]
// fn test_iter() {
//     ALL_WIT_TYPES_FOR_GOLEM.iter().for_each(|f| {
//         let wit_meta = f();
//         use WitMeta::*;

//         let to_print = match wit_meta {
//             Struct(struct_meta) => println!("STRUCT {}", struct_meta.name.0),
//             Function(function_meta) => println!("FUNCTION {}", function_meta.name.0),
//             _ => println!("todo implement"),
//         };

//         println!("{to_print:?}");
//         println!("\n")
//     });
// }
