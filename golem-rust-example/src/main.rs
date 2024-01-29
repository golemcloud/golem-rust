mod generated;

use generated::*;
use golem_rust::WIT;

fn main() {
    // struct without fields
    let empty = Empty {};

    let wit_empty: WitEmpty = empty.into();
    // struct
    let me = Person {
        name: "Jaro".to_owned(),
        age: 32,
    };

    let converted: WitPerson = me.into();

    println!("Struct converted {} {}", converted.name2, converted.age);

    // simple enum
    let yellow = Colors::Yellow;

    let wit_collors: WitColors = yellow.into();

    println!("Enum converted {} ", wit_collors);

    // named enum
    let bid = BidResult::Someone {
        name: "Ema".to_string(),
        age: 10,
    };

    let bid_converted: WitBidResult = bid.into();
}

// uncomment
//#[golem_rust::create_wit_file]
mod golem_component {

    enum IpAddrEmpty {
        V4,
        V6,
    }

    struct Op {}

    pub struct X {
        SoMe_Array: std::option::Option<f64>,
        real_result: Option<String>,
        another: [String],
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

#[derive(WIT)]
#[wit(WitEmpty)]
struct Empty {}

#[rename("dead_code")] // nonsense just to check if such attributes don't interfere
#[derive(WIT)]
#[wit(WitPerson)] // optional as "Wit + classname" is the default
pub struct Person {
    #[rustfmt::skip] // to check if other attributes don't interfere
    #[rename("name2")]
    pub name: String,

    pub age: i32,
}

#[derive(WIT)]
#[rename("dead_code")] // nonsense just to check if such attributes don't interfere
pub enum Colors {
    Red,
    White,

    // TODO check this
    #[rename("Yellow2")]
    // #[rename("Yellow2")]
    Yellow,
}

#[derive(WIT)]
#[rename("dead_code")] // nonsense just to check if such attributes don't interfere
pub enum BidResult {
    #[rename("Success2")]
    Success,

    #[rename("Failure2")]
    Failure(String, u32),

    #[rename("Someone2")]
    Someone { name: String, age: u32 },
}
