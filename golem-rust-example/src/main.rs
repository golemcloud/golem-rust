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

//#[golem_rust::create_wit_file("golem_component")]
mod golem_component {

    enum IpAddrEmpty {
        V4,
        V6,
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

//#[golem_rust::create_wit_file]
mod package_name {
}
