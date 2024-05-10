pub trait HasWitMetadata {
    const IDENT: &'static str;
    const WIT: &'static WitMeta;
}

/**
 * AST TYPES
 */

pub type WitRef = &'static WitMeta;

#[derive(Debug)]
pub enum WitMeta {
    Record(RecordMeta),
    Variant(VariantMeta),
    Enum(EnumMeta),
    FlagMeta(FlagMeta),
    Result(ResultMeta), // Tuple(Vec<WitMeta>)
    Option(WitRef),
    List(WitRef),
    Tuple(TupleMeta),
    Primitive(PrimitiveMeta),
    Function(FunctionMeta),
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
    pub args: &'static [(&'static str, WitRef)],
    pub result: WitRef,
}

#[derive(Debug)]
pub struct RecordMeta {
    pub name: Ident,
    pub fields: &'static [(&'static str, WitRef)],
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
    pub ok: WitRef,
    pub err: WitRef,
}

#[derive(Debug)]
pub struct VariantMeta {
    pub name: Ident,
    pub fields: &'static [VariantOption],
}

#[derive(Debug)]
pub struct VariantOption {
    pub name: Ident,
    pub fields: &'static [WitRef],
}

#[derive(Debug)]
pub struct TupleMeta {
    pub items: &'static [WitRef],
}

#[macro_export]
macro_rules! golem_gen {
    () => {
        #[distributed_slice]
        pub static ALL_WIT_TYPES_FOR_GOLEM: [fn() -> &'static WitMeta];
    };
}

mod primitives {
    use crate::generate_for_tuples;

    use super::*;

    macro_rules! impl_has_wit_metadata {
    ($($type:ty => $ident:expr => $primitive_meta:expr),+) => {
        $(
            impl HasWitMetadata for $type {
                const IDENT: &'static str = $ident;
                const WIT: &'static WitMeta = &WitMeta::Primitive($primitive_meta);
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

    impl<T, E> HasWitMetadata for Result<T, E>
    where
        T: HasWitMetadata,
        E: HasWitMetadata,
    {
        const IDENT: &'static str = "Result";

        const WIT: &'static WitMeta = &WitMeta::Result(ResultMeta {
            ok: T::WIT,
            err: E::WIT,
        });
    }

    impl<T> HasWitMetadata for Option<T>
    where
        T: HasWitMetadata,
    {
        const IDENT: &'static str = "Option";

        const WIT: &'static WitMeta = &WitMeta::Option(T::WIT);
    }

    impl<T> HasWitMetadata for Vec<T>
    where
        T: HasWitMetadata,
    {
        const IDENT: &'static str = "List";

        const WIT: &'static WitMeta = &WitMeta::List(T::WIT);
    }

    macro_rules! impl_has_wit_metadata_for_tuple {
        ($($ty:ident),*) => {
            impl<$($ty),*> HasWitMetadata for ($($ty,)*)
            where
                $($ty: HasWitMetadata),*
            {
                const IDENT: &'static str = "Tuple";
                const WIT: &'static WitMeta = &WitMeta::Tuple(TupleMeta { items: &[$($ty::WIT),*]});
            }
        };
    }

    generate_for_tuples!(impl_has_wit_metadata_for_tuple);
}
