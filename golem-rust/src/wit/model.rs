/// Exports that will go into generated WIT file.
pub trait HasWitExport {
    const EXPORT: WitExport;
}

/// Non exported types, usually will be used for references.
pub trait HasWitMeta {
    const REF: WitMeta;
}

#[derive(Debug)]
pub enum WitExport {
    Record(RecordMeta),
    Variant(VariantMeta),
    Enum(EnumMeta),
    Flag(FlagMeta),
    Function(FunctionMeta),
}

impl WitExport {
    pub fn name(&self) -> &'static str {
        match self {
            WitExport::Record(meta) => &meta.name.0,
            WitExport::Variant(meta) => &meta.name.0,
            WitExport::Enum(meta) => &meta.name.0,
            WitExport::Flag(meta) => &meta.name.0,
            WitExport::Function(meta) => &meta.name.0,
        }
    }
}

/**
 * AST TYPES
 */

pub type WitMetaRef = &'static WitMeta;

#[derive(Debug)]
pub enum WitMeta {
    Result(ResultMeta),
    Option(WitMetaRef),
    List(WitMetaRef),
    Tuple(TupleMeta),
    Primitive(PrimitiveMeta),
    Record(RecordMeta),
    Variant(VariantMeta),
    Enum(EnumMeta),
    FlagMeta(FlagMeta),
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
pub struct FunctionMeta {
    pub name: Ident,
    pub args: &'static [(&'static str, WitMetaRef)],
    pub result: WitMetaRef,
}

#[derive(Debug)]
pub struct RecordMeta {
    pub name: Ident,
    pub fields: &'static [(&'static str, WitMetaRef)],
}

#[derive(Debug)]
pub struct EnumMeta {
    pub name: Ident,
    pub variants: &'static [Ident],
}

#[derive(Debug)]
pub struct FlagMeta {
    pub name: Ident,
    pub variants: &'static [Ident],
}

#[derive(Debug)]
pub struct ResultMeta {
    pub ok: WitMetaRef,
    pub err: WitMetaRef,
}

#[derive(Debug)]
pub struct VariantMeta {
    pub name: Ident,
    pub fields: &'static [VariantOption],
}

#[derive(Debug)]
pub struct VariantOption {
    pub name: Ident,
    pub field: Option<WitMetaRef>,
}

#[derive(Debug)]
pub struct TupleMeta {
    pub items: &'static [WitMetaRef],
}

mod primitives {
    use crate::generate_for_tuples;

    use super::*;

    macro_rules! impl_has_wit_metadata {
    ($($type:ty => $ident:expr => $primitive_meta:expr),+) => {
        $(
            impl HasWitMeta for $type {
                const REF: WitMeta= WitMeta::Primitive($primitive_meta);
            }
        )+
    };
}

    impl_has_wit_metadata! {
        i8 => "s8" => PrimitiveMeta::S8,
        i16 => "s16" => PrimitiveMeta::S16,
        i32 => "s32" => PrimitiveMeta::S32,
        i64 => "s64" => PrimitiveMeta::S64,
        isize => "s64" => PrimitiveMeta::S64,

        u8 => "u8" => PrimitiveMeta::U8,
        u16 => "u16" => PrimitiveMeta::U16,
        u32 => "u32" => PrimitiveMeta::U32,
        u64 => "u64" => PrimitiveMeta::U64,
        usize => "u64" => PrimitiveMeta::U64,

        f32 => "f32" => PrimitiveMeta::F32,
        f64 => "f64" => PrimitiveMeta::F64,

        bool => "bool" => PrimitiveMeta::Bool,
        char => "char" => PrimitiveMeta::Char,
        // TODO: Support all String types.
        String => "String" => PrimitiveMeta::String
    }

    impl<T, E> HasWitMeta for Result<T, E>
    where
        T: HasWitMeta,
        E: HasWitMeta,
    {
        const REF: WitMeta = WitMeta::Result(ResultMeta {
            ok: &T::REF,
            err: &E::REF,
        });
    }

    impl<T> HasWitMeta for Option<T>
    where
        T: HasWitMeta,
    {
        const REF: WitMeta = WitMeta::Option(&T::REF);
    }

    impl<T> HasWitMeta for Vec<T>
    where
        T: HasWitMeta,
    {
        const REF: WitMeta = WitMeta::List(&T::REF);
    }

    macro_rules! impl_has_wit_metadata_for_tuple {
        ($($ty:ident),*) => {
            impl<$($ty),*> HasWitMeta for ($($ty,)*)
            where
                $($ty: HasWitMeta),*
            {
                const REF: WitMeta = WitMeta::Tuple(TupleMeta { items: &[$(&$ty::REF),*]});
            }
        };
    }

    generate_for_tuples!(impl_has_wit_metadata_for_tuple);
}
