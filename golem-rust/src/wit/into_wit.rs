use crate::{PrimitiveMeta, WitExport, WitMeta};
use std::io::Write;

pub fn export_wit_interface(
    interface_name: &str,
    world_name: &str,
    exports: impl AsRef<[WitExport]>,
) -> Res<String> {
    let mut writer = Vec::new();
    let mut serializer = WitSerializer::new(&mut writer, 1);

    for meta in exports.as_ref().iter() {
        serializer.into_wit(meta)?;
        serializer.write_str("\n")?;
    }

    let exports = unsafe { String::from_utf8_unchecked(writer) };

    Ok(format!(
        "interface {interface_name} {{\n{exports}\n}}\n\nworld {world_name} {{\n   export {interface_name};\n}}",
    ))
}

pub fn into_wit(meta: &WitExport) -> Res<String> {
    let mut writer = Vec::new();
    let mut serializer = WitSerializer::new(&mut writer, 0);
    serializer.into_wit(meta)?;
    Ok(unsafe { String::from_utf8_unchecked(writer) })
}

#[derive(Debug)]
pub enum SerializeError {
    Io(std::io::Error),
}

impl From<std::io::Error> for SerializeError {
    fn from(e: std::io::Error) -> Self {
        SerializeError::Io(e)
    }
}

struct WitSerializer<W> {
    writer: W,
    indent: &'static str,
    level: usize,
}

type Res<T> = std::result::Result<T, SerializeError>;

impl<W: Write> WitSerializer<W> {
    fn new(writer: W, level: usize) -> Self {
        WitSerializer {
            writer,
            indent: "   ",
            level,
        }
    }

    fn into_wit(&mut self, meta: &WitExport) -> Res<()> {
        use crate::WitExport::*;
        match meta {
            Record(meta) => {
                self.write_indentation()?;
                self.write_str("record ")?;
                self.write_kebab(&meta.name.0)?;
                self.write_str(" {")?;
                self.inc_indent();
                self.new_line()?;

                self.interleave(
                    Self::write_indentation,
                    "\n",
                    meta.fields.iter().map(|(name, wit)| {
                        |w: &mut Self| {
                            w.write_str(name.as_ref())?;
                            w.write_str(": ")?;
                            w.wit_ref(wit)
                        }
                    }),
                )?;
                self.new_line()?;
                self.dec_indent();
                self.write_indent_line("}")?;
            }
            Variant(meta) => {
                self.write_indentation()?;
                self.write_str("variant ")?;
                self.write_kebab(&meta.name.0)?;
                self.write_str(" {")?;
                self.new_line()?;
                self.inc_indent();

                self.interleave(
                    Self::write_indentation,
                    ",\n",
                    meta.fields.iter().map(|option| {
                        |w: &mut Self| {
                            w.write_kebab(&option.name.0)?;
                            if option.fields.is_empty() {
                                Ok(())
                            } else {
                                w.write_str("(")?;
                                w.interleave(
                                    |_| Ok(()),
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

                self.new_line()?;
                self.dec_indent();
                self.write_indent_line("}")?;
            }
            Enum(meta) => {
                self.write_indentation()?;
                self.write_str("enum ")?;
                self.write_kebab(&meta.name.0)?;
                self.write_str(" {")?;
                self.new_line()?;
                self.inc_indent();

                self.interleave(
                    Self::write_indentation,
                    ",\n",
                    meta.variants
                        .iter()
                        .map(|variant| |w: &mut Self| w.write_kebab(&variant.0)),
                )?;
                self.new_line()?;
                self.dec_indent();
                self.write_indent_line("}")?;
            }
            Function(meta) => {
                self.write_indentation()?;
                self.write_kebab(&meta.name.0)?;
                self.write_str(": func(")?;
                self.interleave(
                    |_| Ok(()),
                    ", ",
                    meta.args.iter().map(|(name, wit)| {
                        |w: &mut Self| {
                            w.write_kebab(*name)?;
                            w.write_str(": ")?;
                            w.wit_ref(wit)
                        }
                    }),
                )?;
                self.write_str(") -> ")?;
                self.wit_ref(&meta.result)?;
                self.new_line()?;
            }
            Flag(_) => todo!(),
        }
        Ok(())
    }

    fn wit_ref(&mut self, wit: &WitMeta) -> Res<()> {
        use WitMeta::*;
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
                self.interleave(
                    |_| Ok(()),
                    ", ",
                    meta.items.iter().map(|wit| |w: &mut Self| w.wit_ref(wit)),
                )
            }
            Primitive(meta) => self.write_str(primitive_wit(meta)),
        }
    }

    fn write_kebab(&mut self, s: &str) -> Res<()> {
        // keep track of prev to know when to add a hyphen.
        let mut prev_char_uppercase = false;

        let s = s.trim_start_matches('_').trim_end_matches('_');

        for (index, c) in s.char_indices() {
            if c.is_uppercase() {
                if index > 0 && !prev_char_uppercase {
                    self.writer.write_all(b"-")?;
                }
                let lowered = c.to_lowercase().to_string();
                self.writer.write_all(lowered.as_bytes())?;
                prev_char_uppercase = true;
            } else if c == '_' {
                self.writer.write_all(&[b'-'])?;
                prev_char_uppercase = false;
            } else {
                self.writer.write_all(&[c as u8])?;
                prev_char_uppercase = false;
            }
        }

        Ok(())
    }

    #[inline]
    fn write_indentation(&mut self) -> Res<()> {
        for _ in 0..self.level {
            self.write_str(self.indent)?;
        }
        Ok(())
    }

    #[inline]
    fn inc_indent(&mut self) {
        self.level += 1;
    }

    #[inline]
    fn dec_indent(&mut self) {
        self.level -= 1;
    }

    #[inline]
    fn new_line(&mut self) -> Res<()> {
        self.write_str("\n")
    }

    #[inline]
    fn write_indent_line(&mut self, s: &str) -> Res<()> {
        self.write_indentation()?;
        self.write_str(s)?;
        self.new_line()
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> Res<()> {
        self.writer.write_all(s.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn interleave<P, I>(
        &mut self,
        mut prefix: P,
        separator: &str,
        iter: impl Iterator<Item = I>,
    ) -> Res<()>
    where
        P: FnMut(&mut Self) -> Res<()>,
        I: FnOnce(&mut Self) -> Res<()>,
    {
        let mut first = true;
        for item in iter {
            if first {
                first = false;
            } else {
                self.write_str(separator)?;
            }
            prefix(self)?;
            item(self)?;
        }
        Ok(())
    }
}

#[inline]
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
