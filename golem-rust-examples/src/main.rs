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
pub trait UserService {
    fn get_person() -> Person;
}

impl IntoWitMetadata for Address {
    fn ident() -> &'static str {
        "Address"
    }

    fn as_wit() -> WitMeta {
        WitMeta::Struct(StructMeta {
            name: Ident("Address".to_owned()),
            fields: vec![
                ("street".to_owned(), Box::new(WitMeta::String)),
                ("city".to_owned(), Box::new(WitMeta::String)),
                ("state".to_owned(), Box::new(WitMeta::String)),
                ("zip".to_owned(), Box::new(WitMeta::String)),
            ],
        })
    }
}

impl IntoWitMetadata for Person {
    fn ident() -> &'static str {
        "Person"
    }

    fn as_wit() -> WitMeta {
        WitMeta::Struct(StructMeta {
            name: Ident("Person".to_owned()),
            fields: vec![
                ("name".to_owned(), Box::new(WitMeta::String)),
                ("address".to_owned(), Box::new(Address::as_wit())),
            ],
        })
    }
}

use linkme::distributed_slice;

#[distributed_slice]
pub static ALL_WIT_TYPES_2: [fn() -> WitMeta];

#[distributed_slice(ALL_WIT_TYPES_2)]
static ADDRESS_WIT: fn() -> WitMeta = || Address::as_wit();

#[distributed_slice(ALL_WIT_TYPES_2)]
static PERSON_WIT: fn() -> WitMeta = || Person::as_wit();

#[test]
fn test_iter() {
    ALL_WIT_TYPES_2.iter().for_each(|f| {
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
