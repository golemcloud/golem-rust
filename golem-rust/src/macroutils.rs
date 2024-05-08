
//use linkme::distributed_slice;

pub trait IntoWitMetadata {
    fn ident() -> &'static str;
    fn as_wit() -> WitMeta;
}

/**
 * AST TYPES
 */
#[derive(Debug)]
pub enum WitMeta {
    Struct(StructMeta),
    Enum(EnumMeta),
    FlagMeta(FlagMeta),
    Result(ResultMeta), // Tuple(Vec<WitMeta>)

    Option(Ident),
    //
    List(Ident),
    // type buffer = list<u8>;
    Alias(Ident),
    String,
    Trait(TraitMeta)
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub struct TraitMeta {
    pub name: String,
    pub args: Vec<(String, Box<WitMeta>)>,
    pub result: Box<WitMeta>
}

#[derive(Debug)]
pub struct StructMeta {
    pub name: Ident,
    pub fields: Vec<(String, Box<WitMeta>)>,
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
    pub ok: Ident,
    pub err: Ident,
}
