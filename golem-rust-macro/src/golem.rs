use darling::{ast, util::IdentString, FromDeriveInput};
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub mod structure {
    use darling::FromField;

    use super::*;

    pub fn expand(tokens: &TokenStream, input: &syn::DeriveInput) -> syn::Result<TokenStream> {
        let struct_meta = StructContainer::from_derive_input(input)?;

        let struct_ident = IdentString::new(struct_meta.ident);
        let struct_name = struct_ident.as_str();

        let fields = struct_meta
            .data
            .take_struct()
            .expect("Struct to have fields");

        let fields = fields
            .iter()
            .map(|field| {
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("Field to have name")
                    .to_string();
                let type_wit_const = make_type_wit_const(&field.ty, true)?;

                Ok(quote! {
                    (#field_name, #type_wit_const)
                })
            })
            .collect::<Result<Vec<TokenStream>, syn::Error>>()?;

        let has_wit_metadata = quote! {
            impl ::golem_rust::HasWitMetadata for #struct_ident {
                const IDENT: &'static str = #struct_name;

                const WIT: &'static ::golem_rust::WitMeta = &::golem_rust::WitMeta::Record(::golem_rust::RecordMeta {
                    name: ::golem_rust::Ident(#struct_name),
                    fields: &[
                        #( #fields ),*
                    ],
                });

            }
        };

        let distributed_slice = make_distributed_slice(&struct_ident);

        Ok(quote! {
            #tokens
            #has_wit_metadata
            #distributed_slice
        })
    }

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(golem), supports(struct_named))]
    pub struct StructContainer {
        pub ident: syn::Ident,
        pub data: ast::Data<(), StructField>,
    }

    #[derive(Debug, FromField)]
    #[darling(attributes(golem))]
    pub struct StructField {
        pub ty: syn::Type,
        pub ident: Option<syn::Ident>,
        pub rename: Option<syn::LitStr>,
    }
}

pub mod enumeration {
    use darling::FromVariant;

    use super::*;

    pub fn expand(tokens: &TokenStream, input: &syn::DeriveInput) -> syn::Result<TokenStream> {
        let enum_meta = EnumContainer::from_derive_input(input)?;

        let enum_ident = IdentString::new(enum_meta.ident);
        let enum_name = enum_ident.as_str();

        let variants = enum_meta.data.take_enum().expect("Enum to have variants");

        let variants = variants.iter().map(|variant| {
            let variant_name = variant.ident.to_string();

            quote!(::golem_rust::Ident(#variant_name))
        });

        let has_wit_metadata = quote! {
            impl ::golem_rust::HasWitMetadata for #enum_ident {
                const IDENT: &'static str = #enum_name;

                const WIT: &'static ::golem_rust::WitMeta = &::golem_rust::WitMeta::Enum(::golem_rust::EnumMeta {
                    name: ::golem_rust::Ident(#enum_name),
                    variants: &[
                        #( #variants ),*
                    ],
                });

            }
        };

        let distributed_slice = make_distributed_slice(&enum_ident);

        Ok(quote! {
            #tokens
            #has_wit_metadata
            #distributed_slice
        })
    }

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(golem), supports(enum_unit))]
    pub struct EnumContainer {
        pub ident: syn::Ident,
        pub data: ast::Data<EnumVariant, ()>,
    }

    #[derive(Debug, FromVariant)]
    #[darling(attributes(golem))]
    pub struct EnumVariant {
        pub ident: syn::Ident,
    }
}

pub mod variant {
    use darling::{FromField, FromVariant};

    use super::*;

    pub fn expand(tokens: &TokenStream, input: &syn::DeriveInput) -> syn::Result<TokenStream> {
        let enum_meta = EnumContainer::from_derive_input(input)?;
        let enum_ident = IdentString::new(enum_meta.ident);
        let enum_name = enum_ident.as_str();

        let variants = enum_meta.data.take_enum().expect("Enum to have variants");

        let variant_options = variants
            .iter()
            .map(|variant| {
                let variant_name = variant.ident.to_string();
                let fields = variant
                    .fields
                    .fields
                    .iter()
                    .map(|field| {
                        let type_wit_const = make_type_wit_const(&field.ty, true);
                        type_wit_const
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let fields_quoted = fields.iter().map(|field| {
                    quote! { #field }
                });

                Ok(quote! {
                    ::golem_rust::VariantOption {
                        name: ::golem_rust::Ident(#variant_name),
                        fields: &[ #(#fields_quoted),* ],
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, syn::Error>>()?;

        let has_wit_metadata = quote! {
            impl ::golem_rust::HasWitMetadata for #enum_ident {
                const IDENT: &'static str = #enum_name;
                const WIT: &'static ::golem_rust::WitMeta = &::golem_rust::WitMeta::Variant(::golem_rust::VariantMeta {
                    name: ::golem_rust::Ident(#enum_name),
                    fields: &[ #(#variant_options),* ],
                });
            }
        };

        let distributed_slice = make_distributed_slice(&enum_ident);

        Ok(quote! {
            #tokens
            #has_wit_metadata
            #distributed_slice
        })
    }

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(golem), supports(enum_tuple))]
    pub struct EnumContainer {
        pub ident: syn::Ident,
        pub data: ast::Data<EnumVariant, ()>,
    }

    #[derive(Debug, FromVariant)]
    #[darling(attributes(golem))]
    pub struct EnumVariant {
        ident: syn::Ident,
        fields: darling::ast::Fields<FieldItem>,
    }

    #[derive(Debug, FromField)]
    pub struct FieldItem {
        pub ty: syn::Type,
    }
}

fn make_distributed_slice(ident: &IdentString) -> proc_macro2::TokenStream {
    let slice_ident = syn::Ident::new(
        &format!("{}_WIT", ident.as_str().to_ascii_uppercase()),
        proc_macro2::Span::call_site(),
    );

    let ident = ident.as_ident();

    quote! {
        #[distributed_slice(crate::ALL_WIT_TYPES_FOR_GOLEM)]
        static #slice_ident: fn() -> &'static ::golem_rust::WitMeta = || #ident::WIT;
    }
}

// Result<A, B> -> Result::<A, B>::WIT
// Option<T> -> Option::<T>::WIT
// Vec<T> -> Vec::<T>::WIT
// Result<Option<T>, E> -> Result::<Option::<T>, E>::WIT
fn make_type_wit_const(ty: &syn::Type, is_root: bool) -> syn::Result<proc_macro2::TokenStream> {
    match ty {
        syn::Type::Path(syn::TypePath { path, qself: None }) => {
            let ty = &path.segments.last().expect("Paths to be non-empty").ident;

            let generic_args = match &path.segments.last().unwrap().arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    Some(args.args.iter().collect::<Vec<_>>())
                }
                _ => None,
            };

            if let Some(generic_args) = generic_args {
                let generic_types = generic_args
                    .iter()
                    .map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => make_type_wit_const(ty, false),
                        s => Err(syn::Error::new(
                            s.span(),
                            format!("Unsupported generic argument: {:#?}", s),
                        )),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                if is_root {
                    Ok(quote! {
                        #ty::<#(#generic_types),*>::WIT
                    })
                } else {
                    Ok(quote! {
                        #ty::<#(#generic_types),*>
                    })
                }
            } else {
                if is_root {
                    Ok(quote! {
                        #ty::WIT
                    })
                } else {
                    Ok(quote! {
                        #ty
                    })
                }
            }
        }
        unsupported => Err(syn::Error::new(
            unsupported.span(),
            format!("Unsupported type: {:#?}", unsupported),
        )),
    }
}

pub fn implement_global_function(ast: syn::ItemFn) -> syn::Result<TokenStream> {
    // let ast = syn::parse::<syn::ItemFn>(token_stream)?;

    //get_address
    let function_name = ast.sig.ident.clone();

    //GetAddress
    let struct_name = syn::Ident::new(
        &(function_name.clone().to_string().to_pascal_case()),
        function_name.span(),
    );

    let new_struct = quote! {
        struct #struct_name {}
    };

    let mut uppercase_name = "FUN".to_owned();
    uppercase_name.push_str(&(struct_name.to_string().to_uppercase()));

    //FUN_GET_ADDRESS
    let uppercase_name_ident = syn::Ident::new(&uppercase_name, struct_name.span());

    let distributed_slice = quote! {
        #[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
        static #uppercase_name_ident: fn() -> &'static WitMeta = || #struct_name::WIT;
    };

    // "get_address"
    let function_name_string = function_name.to_string();

    let into_wit_impl = quote! {
        impl HasWitMetadata for #struct_name {
            const IDENT: &'static str = #function_name_string;

            const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
                name: Ident("Address"),
                args: &[],
                result: Address::WIT,
            });
        }
    };

    Ok(quote!(
        #new_struct

        #distributed_slice
        #ast
    ))
}

#[test]
fn test_convert() {
    println!("RESULT ");
}
