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
                let type_wit_const = type_wit_ref(&field.ty)?;

                Ok(quote! {
                    (#field_name, &#type_wit_const)
                })
            })
            .collect::<Result<Vec<TokenStream>, syn::Error>>()?;

        let record_meta = quote! {
            ::golem_rust::RecordMeta {
                name: ::golem_rust::Ident(#struct_name),
                fields: &[
                    #( #fields ),*
                ],
            }
        };

        let has_wit_export = quote! {
            impl ::golem_rust::HasWitExport for #struct_ident {
                const IDENT: &'static str = #struct_name;

                const WIT: ::golem_rust::WitExport = ::golem_rust::WitExport::Record(#record_meta);
            }
        };

        let has_wit_ref = quote! {
            impl ::golem_rust::HasWitMeta for #struct_ident {
                const REF: ::golem_rust::WitMeta = ::golem_rust::WitMeta::Record(#record_meta);
            }
        };

        let distributed_slice = make_distributed_slice(&struct_ident);

        Ok(quote! {
            #tokens
            #has_wit_export
            #has_wit_ref
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

        let enum_meta = quote! {
            ::golem_rust::EnumMeta {
                name: ::golem_rust::Ident(#enum_name),
                variants: &[
                    #( #variants ),*
                ],
            }
        };

        let has_wit_export = quote! {
            impl ::golem_rust::HasWitExport for #enum_ident {
                const IDENT: &'static str = #enum_name;
                const WIT: ::golem_rust::WitExport = ::golem_rust::WitExport::Enum(#enum_meta);
            }
        };

        let has_wit_meta = quote! {
            impl ::golem_rust::HasWitMeta for #enum_ident {
                const REF: ::golem_rust::WitMeta = ::golem_rust::WitMeta::Enum(#enum_meta);
            }
        };

        let distributed_slice = make_distributed_slice(&enum_ident);

        Ok(quote! {
            #tokens
            #has_wit_export
            #has_wit_meta
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
                    .map(|field| type_wit_ref(&field.ty))
                    .collect::<Result<Vec<_>, _>>()?;

                let fields_quoted = fields.iter().map(|field| {
                    quote! { &#field }
                });

                Ok(quote! {
                    ::golem_rust::VariantOption {
                        name: ::golem_rust::Ident(#variant_name),
                        fields: &[ #(#fields_quoted),* ],
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, syn::Error>>()?;

        let variant_meta = quote! {
            ::golem_rust::VariantMeta {
                name: ::golem_rust::Ident(#enum_name),
                fields: &[ #(#variant_options),* ],
            }
        };

        let has_wit_export = quote! {
            impl ::golem_rust::HasWitExport for #enum_ident {
                const IDENT: &'static str = #enum_name;
                const WIT: ::golem_rust::WitExport = ::golem_rust::WitExport::Variant(#variant_meta);
            }
        };

        let has_wit_ref = quote! {
            impl ::golem_rust::HasWitMeta for #enum_ident {
                const REF: ::golem_rust::WitMeta = ::golem_rust::WitMeta::Variant(#variant_meta);
            }
        };

        let distributed_slice = make_distributed_slice(&enum_ident);

        Ok(quote! {
            #tokens
            #has_wit_export
            #has_wit_ref
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
        static #slice_ident: fn() -> ::golem_rust::WitExport = || #ident::WIT;
    }
}

// Result<A, B> -> Result::<A, B>::REF
// Option<T> -> Option::<T>::REF
// Vec<T> -> Vec::<T>::REF
// Result<Option<T>, E> -> Result::<Option::<T>, E>::REF
// (T, U) -> (T, U)::REF
// std::result::Result<T, E> -> std::result::Result::<T, E>::REF
fn type_wit_ref(ty: &syn::Type) -> syn::Result<TokenStream> {
    fn go(ty: &syn::Type) -> syn::Result<TokenStream> {
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
                            syn::GenericArgument::Type(ty) => go(ty),
                            s => Err(syn::Error::new(
                                s.span(),
                                format!("Unsupported generic argument: {:#?}", s),
                            )),
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    Ok(quote! {
                        #path::<#(#generic_types),*>
                    })
                } else {
                    Ok(quote! {
                        #path
                    })
                }
            }
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                let generic_types = elems.iter().map(go).collect::<Result<Vec<_>, _>>()?;

                Ok(quote! {
                    (#(#generic_types),*)
                })
            }
            unsupported @ syn::Type::Reference(_) => Err(syn::Error::new(
                unsupported.span(),
                "Unsupported type: References are not allowed. All data must be owned",
            )),
            unsupported => Err(syn::Error::new(unsupported.span(), "Unsupported type")),
        }
    }

    let result = go(ty)?;

    Ok(quote! {
        #result::REF
    })
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

    let output_type = (match &ast.sig.output {
        ReturnType::Default => syn::Result::Err(syn::Error::new(
            ast.sig.output.span(),
            "Function needs to have explicit return type",
        )),
        ReturnType::Type(_, box_type) => type_wit_ref(box_type),
    })?;

    let all_input_args = ast
        .sig
        .inputs
        .iter()
        .map(|tpe| match tpe {
            FnArg::Receiver(_) => syn::Result::Err(syn::Error::new(
                tpe.span(),
                "Function needs to have explicit return type",
            )),
            FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(i) => {
                    let args_name = i.ident.to_string();
                    type_wit_ref(&pat_type.ty).map(|ts| {
                        quote! {
                                (#args_name, &#ts)
                        }
                    })
                }
                _ => syn::Result::Err(syn::Error::new(pat_type.span(), "Unexpected pattern")),
            },
        })
        .collect::<::syn::Result<Vec<_>>>()?;

    let fun_args = quote! {
        &[#(#all_input_args),*]
    };

    let into_wit_impl = quote! {
        impl HasWitExport for #struct_name {
            const IDENT: &'static str = #function_name_string;

            const WIT: WitExport = WitExport::Function(FunctionMeta {
                name: Ident(#function_name_string),
                args: #fun_args,
                result: &#output_type,
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
        let result = type_wit_ref(&ty).unwrap();
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
        test_make_type_wit_const(parse_quote!(String), quote!(String::REF));
    }

    #[test]
    fn test_generic_type() {
        test_make_type_wit_const(parse_quote!(Option<String>), quote!(Option::<String>::REF));
    }

    #[test]
    fn test_nested_generic_type() {
        test_make_type_wit_const(
            parse_quote!(Result<Option<String>, String>),
            quote!(Result::<Option::<String>, String>::REF),
        );
    }

    #[test]
    fn test_tuple_type() {
        test_make_type_wit_const(parse_quote!((String, u32)), quote!((String, u32)::REF));
    }

    #[test]
    fn test_nested_tuple_type() {
        test_make_type_wit_const(
            parse_quote!(Result<(String, u32), String>),
            quote!(Result::<(String, u32), String>::REF),
        );
    }

    #[test]
    fn test_full_path() {
        test_make_type_wit_const(
            parse_quote!(std::result::Result<(std::option::Option<String>, u32), String>),
            quote!(std::result::Result::<(std::option::Option::<String>, u32), String>::REF),
        );
    }
}
