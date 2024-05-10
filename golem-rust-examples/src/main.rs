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

        let wit_string = into_wit::into_wit(wit_meta).unwrap();

        println!("{wit_string}");
        println!("\n")
    });
}

#[cfg(test)]
mod into_wit {
    use golem_rust::{
        PrimitiveMeta,
        WitMeta::{self, *},
    };
    use heck::ToKebabCase;
    use std::io::Write;

    pub fn into_wit(meta: &WitMeta) -> Res<String> {
        let mut writer = Vec::new();
        let mut serializer = WitSerializer::new(&mut writer);
        serializer.into_wit(meta)?;
        Ok(unsafe { String::from_utf8_unchecked(writer) })
    }

    struct WitSerializer<W> {
        writer: W,
        indent: &'static str,
    }

    #[derive(Debug)]
    pub enum SerializeError {
        Unsupported(&'static str),
        Io(std::io::Error),
    }

    impl From<std::io::Error> for SerializeError {
        fn from(e: std::io::Error) -> Self {
            SerializeError::Io(e)
        }
    }

    type Res<T> = std::result::Result<T, SerializeError>;

    impl<W: Write> WitSerializer<W> {
        fn new(writer: W) -> Self {
            WitSerializer {
                writer,
                indent: "   ",
            }
        }

        fn into_wit(&mut self, meta: &WitMeta) -> Res<()> {
            match meta {
                Record(meta) => {
                    self.write_str("record ")?;
                    self.write_kebab(&meta.name.0)?;
                    self.write_str(" {\n")?;
                    self.interleave(
                        &self.indent,
                        ",\n",
                        meta.fields.iter().map(|(name, wit)| {
                            |w: &mut Self| {
                                w.write_str(name.as_ref())?;
                                w.write_str(": ")?;
                                w.wit_ref(wit)
                            }
                        }),
                    )?;
                    self.write_str("\n}")?;
                }
                Variant(meta) => {
                    self.write_str("variant ")?;
                    self.write_kebab(&meta.name.0)?;
                    self.write_str(" {\n")?;

                    self.interleave(
                        &self.indent,
                        ",\n",
                        meta.fields.iter().map(|option| {
                            |w: &mut Self| {
                                w.write_kebab(&option.name.0)?;
                                if option.fields.is_empty() {
                                    Ok(())
                                } else {
                                    w.write_str("(")?;
                                    w.interleave(
                                        "",
                                        ", ",
                                        option
                                            .fields
                                            .iter()
                                            .map(|field| |w: &mut Self| w.wit_ref(field)),
                                    )?;
                                    w.write_str(")")
                                }
                            }
                        }),
                    )?;

                    self.write_str("\n}")?;
                }
                Enum(meta) => {
                    self.write_str("enum ")?;
                    self.write_kebab(&meta.name.0)?;
                    self.write_str(" {\n")?;

                    self.interleave(
                        &self.indent,
                        ",\n",
                        meta.variants
                            .iter()
                            .map(|variant| |w: &mut Self| w.write_kebab(&variant.0)),
                    )?;
                    self.write_str("\n}")?;
                }
                Function(meta) => {
                    self.write_kebab(&meta.name.0)?;
                    self.write_str(": func(")?;
                    self.interleave(
                        "",
                        ", ",
                        meta.args.iter().map(|(name, wit)| {
                            |w: &mut Self| {
                                w.write_str(name.as_ref())?;
                                w.write_str(": ")?;
                                w.wit_ref(wit)
                            }
                        }),
                    )?;
                    self.write_str(") -> ")?;
                    self.wit_ref(&meta.result)?;
                }
                _ => {
                    return Err(SerializeError::Unsupported(
                        "Unsupported top level wit type",
                    ))
                }
            }
            Ok(())
        }

        fn wit_ref(&mut self, wit: &WitMeta) -> Res<()> {
            match wit {
                Record(meta) => self.write_kebab(&meta.name.0),
                Variant(meta) => self.write_kebab(&meta.name.0),
                Enum(meta) => self.write_kebab(&meta.name.0),
                FlagMeta(meta) => self.write_kebab(&meta.name.0),
                Result(meta) => {
                    self.write_str("result<")?;
                    self.wit_ref(&meta.ok)?;
                    self.write_str(", ")?;
                    self.wit_ref(&meta.err)?;
                    self.write_str(">")
                }
                Option(meta) => {
                    self.write_str("option<")?;
                    self.wit_ref(meta)?;
                    self.write_str(">")
                }
                List(meta) => {
                    self.write_str("list<")?;
                    self.wit_ref(meta)?;
                    self.write_str(">")
                }
                Tuple(meta) => {
                    self.write_str("tuple<")?;
                    for (i, wit) in meta.items.iter().enumerate() {
                        if i > 0 {
                            self.write_str(", ")?;
                        }
                        self.wit_ref(wit)?;
                    }
                    self.write_str(">")
                }
                Primitive(meta) => self.write_str(primitive_wit(meta)),
                Function(_) => {
                    return Err(SerializeError::Unsupported("Unsupported Reference Type"))
                }
            }
        }

        #[inline]
        fn write_kebab(&mut self, s: &str) -> Res<()> {
            // TODO: Don't collect to string, and write to buffer directly
            self.write_str(&s.to_kebab_case())
        }

        #[inline]
        fn write_str(&mut self, s: &str) -> Res<()> {
            self.writer.write_all(s.as_bytes())?;
            Ok(())
        }

        #[inline]
        fn interleave<I>(
            &mut self,
            prefix: &str,
            separator: &str,
            iter: impl Iterator<Item = I>,
        ) -> Res<()>
        where
            I: FnOnce(&mut Self) -> Res<()>,
        {
            let mut first = true;
            for item in iter {
                if first {
                    first = false;
                } else {
                    self.write_str(separator)?;
                }
                self.write_str(prefix)?;
                item(self)?;
            }
            Ok(())
        }
    }

    fn primitive_wit(meta: &PrimitiveMeta) -> &'static str {
        match meta {
            PrimitiveMeta::S8 => "s8",
            PrimitiveMeta::S16 => "s16",
            PrimitiveMeta::S32 => "s32",
            PrimitiveMeta::S64 => "s64",
            PrimitiveMeta::U8 => "u8",
            PrimitiveMeta::U16 => "u16",
            PrimitiveMeta::U32 => "u32",
            PrimitiveMeta::U64 => "u64",
            PrimitiveMeta::F32 => "f32",
            PrimitiveMeta::F64 => "f64",
            PrimitiveMeta::Bool => "bool",
            PrimitiveMeta::Char => "char",
            PrimitiveMeta::String => "string",
        }
    }
}
