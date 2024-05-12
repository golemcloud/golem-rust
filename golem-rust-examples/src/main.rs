use golem_rust::*;
use linkme::distributed_slice;

fn main() {}

golem_gen!();

// disable unused code for now
#[allow(dead_code)]
mod trial {
    use super::*;

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
        Red,
        Green,
        Blue,
        BlueGreen,
    }

    #[golem()]
    enum VariantTest {
        V1(String, u32),
        V2(Vec<String>),
    }

    #[golem()]
    struct BidderId {
        pub bidder_id: std::result::Result<Color, String>,
        pub verified: bool,
    }

    #[golem()]
    fn create_bidder(full_name: String, address: Vec<String>, age: Option<u16>) -> BidderId {
        let _ = full_name;
        let _ = address;
        let _ = age;

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
            color: Color::Blue,
        }
    }
}

#[test]
fn test_iter() {
    let all_wit = ALL_WIT_TYPES_FOR_GOLEM
        .iter()
        .map(|f| f())
        .collect::<Vec<_>>();

    println!(
        "{}",
        export_wit_interface("api", "golem-world", all_wit.as_slice()).unwrap()
    );
}
