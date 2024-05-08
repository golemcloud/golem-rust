//use linkme::distributed_slice;

/**
 * AST TYPES
 */

pub type WitRef = &'static WitMeta;

#[derive(Debug)]
pub enum WitMeta {
    Struct(StructMeta),
    Enum(EnumMeta),
    FlagMeta(FlagMeta),
    Result(ResultMeta), // Tuple(Vec<WitMeta>)
    Option(WitRef),
    List(WitRef),

    // type buffer = list<u8>;
    Alias(WitRef),
    Primitive(PrimitiveMeta),
}

#[derive(Debug)]
pub enum PrimitiveMeta {
    S8,
    S16,
    S32,
    S64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Bool,
    Char,
    String,
}

#[derive(Debug)]
pub struct Ident(pub &'static str);

#[derive(Debug)]
pub struct StructMeta {
    pub name: Ident,
    pub fields: &'static [(&'static str, WitRef)],
}

#[derive(Debug)]
pub struct EnumMeta {
    pub name: Ident,
    pub variants: Vec<Ident>,
}

#[derive(Debug)]
pub struct FlagMeta {
    pub name: Ident,
    pub variants: Vec<Ident>,
}

#[derive(Debug)]
pub struct ResultMeta {
    pub ok: WitRef,
    pub err: WitRef,
}
