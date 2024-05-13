use golem_rust::*;

fn main() {}

// disable unused code for now
#[allow(dead_code)]
mod trial {
    use super::*;

    #[golem()]
    struct Person {
        name: String,
        address: Address,
    }

    #[golem]
    struct SimpleStruct {
        name: String,
        age: u16,
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
        V1(String),
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
    include!("../.golem/bindgen.rs");
}

#[test]
fn test_iter() {
    fn write_to_file(
        path: impl AsRef<std::path::Path>,
        content: &str,
    ) -> Result<(), std::io::Error> {
        use std::io::Write;
        if let Some(parent) = path.as_ref().parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut file = std::fs::File::create(path.as_ref())?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    let all_wit = get_all_wit_types();

    let wit = export_wit_interface(
        &all_wit,
        WitOptions {
            interface: "api",
            world: "test-worker",
            package_namespace: "golem",
            package_name: "test",
            version: None,
        },
    )
    .unwrap();

    write_to_file(".golem/api.wit", &wit).unwrap();

    let wit_bindgen = format!(
        r#"
wit_bindgen::generate!({{
    inline: "
{wit}
",
}});
"#
    );

    write_to_file(".golem/bindgen.rs", &wit_bindgen).unwrap();
}
