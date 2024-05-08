mod generated;

fn main() {
    // let empty = Empty {};

    // let wit_empty: WitEmpty = empty.into();
    // let me = Person {
    //     name: "Jaro".to_owned(),
    //     age: 32,
    // };

    //let converted: WitPerson = me.into();

    // let yellow = Colors::Yellow;
    // let wit_collors: WitColors = yellow.into();

    // let bid = BidResult::Someone {
    //     name: "Ema".to_string(),
    //     age: 10,
    // };
    // let bid_converted: WitBidResult = bid.into();
}

use golem_rust::*;

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

//#[golem()]
// pub trait UserService {
//     fn get_person() -> Person;
// }

#[distributed_slice]
pub static ALL_WIT_TYPES: [fn() -> &'static WitMeta];

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

#[distributed_slice(ALL_WIT_TYPES)]
static ADDRESS_WIT: fn() -> &'static WitMeta = || Address::WIT;

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

#[distributed_slice(ALL_WIT_TYPES)]
static PERSON_WIT: fn() -> &'static WitMeta = || Person::WIT;

impl HasWitMetadata for Person {
    const IDENT: &'static str = "Person";

    const WIT: &'static WitMeta = &WitMeta::Struct(StructMeta {
        name: Ident("Person"),
        fields: &[("name", String::WIT), ("address", Address::WIT)],
    });
}

#[test]
fn test_iter() {
    ALL_WIT_TYPES.iter().for_each(|f| {
        let wit = f();
        println!("{wit:?}");
    });
}

//[House, HouseService]

fn test() {

    //House::to_wit();
}

// #[derive(golem_rust::WIT_From_Into)]
// #[wit_type_name(WitEmpty)]
struct Empty {}

// TODO
// golem_rust::from (do from both ways)
//#[derive(WIT_From_Into)]
// #[golem(wit = WitPerson)]
// //#[wit_type_name(WitPerson)]
// pub struct Person {
//     // #rename

//     // darling
//     //#[rename_field("name2")]
//     pub name: String,

//     pub age: i32,
// }

//#[derive(golem_rust::WIT_From_Into)]
pub enum Colors {
    Red,
    White,

    //#[rename_field("Yellow2")]
    Yellow,
}

//#[derive(golem_rust::WIT_From_Into)]
pub enum BidResult {
    //#[rename_field("Success2")]
    Success,

    //#[rename_field("Failure2")]
    Failure(String, u32),

    //#[rename_field("Someone2")]
    Someone { name: String, age: u32 },
}

//uncomment
//#[golem_rust::create_wit_file("golem_component")]
mod golem_component {

    enum IpAddrEmpty {
        V4,
        V6,
    }

    enum IpAddr {
        V4(String),
        V6(String),
    }

    pub struct X {
        SoMe_Array: Option<f64>,
        another: [String], // Vec -> list, Box<_>
    }

    pub struct BidderId {
        pub bidder_id: std::result::Result<IpAddrEmpty, String>,
        pub verified: bool,
    }

    trait AuctionService {
        fn create_bidder(full_name: String, address: String, age: u16) -> BidderId;

        fn register() -> ();

        fn register2() -> X;

        fn register3();
    }
}

//uncomment
//#[golem_rust::create_wit_file("example.wit")]
mod golem_component2 {

    pub struct BidderId {
        pub bidder_id: String,
        pub verified: bool,
    }
    trait AuctionService {
        fn create_bidder(full_name: String, address: String, age: u16) -> BidderId;
    }
}

//uncomment
//#[golem_rust::create_wit_file("auction_app.wit")]
mod auction_app {

    struct BidderId {
        bidder_id: String,
    }

    struct AuctionId {
        auction_id: String,
    }

    struct Auction {
        auction_id: Option<AuctionId>,
        name: String,
        description: String,
        starting_price: f32,
        deadline: Deadline,
    }

    enum BidResult {
        Failure(String),
        Success,
    }

    type Deadline = u64;

    trait AuctionService {
        fn initialize(auction: Auction);

        fn bid(bidder_id: BidderId, price: f32) -> BidResult;

        fn close_auction() -> Option<BidderId>;

        fn create_bidder(name: String, address: String) -> BidderId;

        fn create_auction(
            name: String,
            description: String,
            starting_price: f32,
            deadline: u64,
        ) -> AuctionId;

        fn get_auctions() -> Vec<Auction>;
    }
}

//uncomment
//#[golem_rust::create_wit_file]
mod package_name {}
