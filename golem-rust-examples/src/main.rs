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
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[golem()]
fn get_address() -> Address {
    Address {
        street: "".to_owned(),
        city: "".to_owned(),
        state: "".to_owned(),
        zip: "".to_owned(),
    }
}

trait HasWitMetadata {
    const IDENT: &'static str;
    const WIT: &'static WitMeta;
}

mod primitives {
    macro_rules! impl_has_wit_metadata {
    ($($type:ty => $ident:expr => $primitive_meta:expr),+) => {
        $(
            impl HasWitMetadata for $type {
                const IDENT: &'static str = $ident;
                const WIT: &'static WitMeta = &WitMeta::Primitive($primitive_meta);
            }
        )+
    };
}
    use crate::HasWitMetadata;
    use golem_rust::{PrimitiveMeta, WitMeta};

    impl_has_wit_metadata! {
        i8 => "i8" => PrimitiveMeta::S8,
        i16 => "i16" => PrimitiveMeta::S16,
        i32 => "i32" => PrimitiveMeta::S32,
        i64 => "i64" => PrimitiveMeta::S64,

        u8 => "u8" => PrimitiveMeta::U8,
        u32 => "u32" => PrimitiveMeta::U32,
        u64 => "u64" => PrimitiveMeta::U64,

        f32 => "f32" => PrimitiveMeta::F32,
        f64 => "f64" => PrimitiveMeta::F64,

        bool => "bool" => PrimitiveMeta::Bool,
        char => "char" => PrimitiveMeta::Char,
        // TODO: Support all String types.
        String => "String" => PrimitiveMeta::String
    }
}

struct GetAddress {}

impl HasWitMetadata for GetAddress {
    const IDENT: &'static str = "get_address";

    const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
        name: Ident("Address"),
        args: &[],
        result: Address::WIT,
    });
}

impl HasWitMetadata for Address {
    const IDENT: &'static str = "Address";

    const WIT: &'static WitMeta = &WitMeta::Struct(StructMeta {
        name: Ident("Address"),
        fields: &[
            ("street", String::WIT),
            ("city", String::WIT),
            ("state", String::WIT),
            ("zip", String::WIT),
        ],
    });
}

impl HasWitMetadata for Person {
    const IDENT: &'static str = "Person";

    const WIT: &'static WitMeta = &WitMeta::Struct(StructMeta {
        name: Ident("Person"),
        fields: &[("name", String::WIT), ("address", Address::WIT)],
    });
}

#[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
static PERSON_WIT: fn() -> &'static WitMeta = || Person::WIT;

#[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
static ADDRESS_WIT: fn() -> &'static WitMeta = || Address::WIT;

#[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
static FUN_WIT: fn() -> &'static WitMeta = || GetAddress::WIT;

#[test]
fn test_iter() {
    ALL_WIT_TYPES_FOR_GOLEM.iter().for_each(|f| {
        let wit_meta = f();
        use WitMeta::*;

        let to_print = match wit_meta {
            Struct(struct_meta) => println!("STRUCT {}", struct_meta.name.0),
            Function(function_meta) => println!("FUNCTION {}", function_meta.name.0),
            _ => println!("todo implement"),
        };

        println!("{to_print:?}");
        println!("\n")
    });
}
