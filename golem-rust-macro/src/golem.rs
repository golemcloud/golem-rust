use darling::{ast, util::IdentString, FromDeriveInput};
use heck::ToPascalCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, FnArg, ReturnType};

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
                let type_wit_const = make_type_wit_const(&field.ty)?;

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
                        let type_wit_const = make_type_wit_const(&field.ty);
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

fn make_distributed_slice(ident: &IdentString) -> TokenStream {
    let slice_ident = syn::Ident::new(
        &format!("{}_WIT", ident.as_str().to_ascii_uppercase()),
        Span::call_site(),
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
// (T, U) -> (T, U)::WIT
// std::result::Result<T, E> -> std::result::Result::<T, E>::WIT
fn make_type_wit_const(ty: &syn::Type) -> syn::Result<TokenStream> {
    fn go(ty: &syn::Type, is_root: bool) -> syn::Result<TokenStream> {
        match ty {
            syn::Type::Path(syn::TypePath { path, qself: None }) => {
                let generic_args = match &path.segments.last().unwrap().arguments {
                    syn::PathArguments::AngleBracketed(args) => Some(&args.args),
                    _ => None,
                };

                let path = path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");

                let path: syn::Path = syn::parse_str(&path).unwrap();

                if let Some(generic_args) = generic_args {
                    let generic_types = generic_args
                        .iter()
                        .map(|arg| match arg {
                            syn::GenericArgument::Type(ty) => go(ty, false),
                            s => Err(syn::Error::new(
                                s.span(),
                                format!("Unsupported generic argument: {:#?}", s),
                            )),
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    if is_root {
                        Ok(quote! {
                            #path::<#(#generic_types),*>::WIT
                        })
                    } else {
                        Ok(quote! {
                            #path::<#(#generic_types),*>
                        })
                    }
                } else {
                    if is_root {
                        Ok(quote! {
                            #path::WIT
                        })
                    } else {
                        Ok(quote! {
                            #path
                        })
                    }
                }
            }
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                let generic_types = elems
                    .iter()
                    .map(|ty| go(ty, false))
                    .collect::<Result<Vec<_>, _>>()?;

                if is_root {
                    Ok(quote! {
                        (#(#generic_types),*)::WIT
                    })
                } else {
                    Ok(quote! {
                        (#(#generic_types),*)
                    })
                }
            }
            unsupported @ syn::Type::Reference(_) => Err(syn::Error::new(
                unsupported.span(),
                format!("Unsupported type: References are not allowed. All data must be owned"),
            )),
            unsupported => Err(syn::Error::new(
                unsupported.span(),
                format!("Unsupported type"),
            )),
        }
    }

    go(ty, true)
}

pub fn implement_global_function(ast: syn::ItemFn) -> syn::Result<TokenStream> {
    //get_address
    let function_name = IdentString::new(ast.sig.ident.clone());

    //GetAddress
    let struct_name = IdentString::new(syn::Ident::new(
        &(function_name.clone().to_string().to_pascal_case()),
        function_name.span(),
    ));

    let new_struct = quote! {
        struct #struct_name {}
    };

    // "get_address"
    let function_name_string = function_name.as_str();

    let output = ast.sig.output.clone();

    let output_type = (match output.clone() {
        ReturnType::Default => syn::Result::Err(syn::Error::new(
            output.span(),
            "Function needs to have explicit return type",
        )),
        ReturnType::Type(_, box_type) => make_type_wit_const(&(*box_type)),
    })?;

    let all_input_args = ast
        .sig
        .inputs
        .clone()
        .into_iter()
        .map(|tpe| match tpe {
            FnArg::Receiver(_) => syn::Result::Err(syn::Error::new(
                output.clone().span(),
                "Function needs to have explicit return type",
            )),
            FnArg::Typed(pat_type) => match *pat_type.pat {
                syn::Pat::Ident(i) => {
                    let args_name = i.ident.to_string();
                    make_type_wit_const(&(*pat_type.ty)).map(|ts| {
                        quote! {
                            (#args_name, #ts)
                        }
                    })
                }
                _ => syn::Result::Err(syn::Error::new(output.span(), "Unexpected pattern")),
            },
        })
        .collect::<::syn::Result<Vec<_>>>()?;

    let fun_args = quote! {
        &[#(#all_input_args),*]
    };

    let into_wit_impl = quote! {
        impl HasWitMetadata for #struct_name {
            const IDENT: &'static str = #function_name_string;

            const WIT: &'static WitMeta = &WitMeta::Function(FunctionMeta {
                name: Ident(#function_name_string),
                args: #fun_args,
                result: #output_type,
            });
        }
    };

    let distributed_slice = make_distributed_slice(&struct_name);

    Ok(quote!(
        #new_struct
        #into_wit_impl
        #distributed_slice
        #ast
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use std::panic::Location;
    use syn::parse_quote;

    #[track_caller]
    fn test_make_type_wit_const(ty: syn::Type, expected: proc_macro2::TokenStream) {
        let location = Location::caller();
        let result = make_type_wit_const(&ty).unwrap();
        let result_str = result.to_string();
        let expected_str = expected.to_string();
        if result_str != expected_str {
            panic!(
                "Assertion failed at {}:{}\nExpected: {}\nActual: {}",
                location.file(),
                location.line(),
                expected_str,
                result_str
            );
        }
    }

    #[test]
    fn test_basic_type() {
        test_make_type_wit_const(parse_quote!(String), quote!(String::WIT));
    }

    #[test]
    fn test_generic_type() {
        test_make_type_wit_const(parse_quote!(Option<String>), quote!(Option::<String>::WIT));
    }

    #[test]
    fn test_nested_generic_type() {
        test_make_type_wit_const(
            parse_quote!(Result<Option<String>, String>),
            quote!(Result::<Option::<String>, String>::WIT),
        );
    }

    #[test]
    fn test_tuple_type() {
        test_make_type_wit_const(parse_quote!((String, u32)), quote!((String, u32)::WIT));
    }

    #[test]
    fn test_nested_tuple_type() {
        test_make_type_wit_const(
            parse_quote!(Result<(String, u32), String>),
            quote!(Result::<(String, u32), String>::WIT),
        );
    }

    #[test]
    fn test_full_path() {
        test_make_type_wit_const(
            parse_quote!(std::result::Result<(std::option::Option<String>, u32), String>),
            quote!(std::result::Result::<(std::option::Option::<String>, u32), String>::WIT),
        );
    }
}
