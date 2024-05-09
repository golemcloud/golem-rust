use darling::{ast, util::IdentString, FromDeriveInput};
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub mod structure {
    use darling::FromField;

    use super::*;

    pub fn expand(
        tokens: &proc_macro2::TokenStream,
        input: &syn::DeriveInput,
    ) -> syn::Result<proc_macro2::TokenStream> {
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
                let field_type = &field.ty;

                let type_wit_const = make_type_wit_const(field_type, true);

                type_wit_const.map(|ty| (field_name, ty))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // convert the fields to quote.
        let fields = fields.iter().map(|(name, ty)| {
            quote! {
                (#name, #ty)
            }
        });

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
        // pub ty: syn::Type,
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

    pub fn expand(
        tokens: &proc_macro2::TokenStream,
        input: &syn::DeriveInput,
    ) -> syn::Result<proc_macro2::TokenStream> {
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

fn make_type_wit_const(ty: &syn::Type, is_root: bool) -> syn::Result<proc_macro2::TokenStream> {
    let span = ty.span();

    match ty {
        syn::Type::Path(syn::TypePath { path, qself: None }) => {
            let ty = &path.segments.last().unwrap().ident;
            let generic_args = match &path.segments.last().unwrap().arguments {
                syn::PathArguments::AngleBracketed(args) => args.args.iter().collect::<Vec<_>>(),
                _ => Vec::new(),
            };

            if generic_args.is_empty() {
                if is_root {
                    Ok(quote! {
                        #ty::WIT
                    })
                } else {
                    Ok(quote! {
                        #ty
                    })
                }
            } else {
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
            }
        }
        unsupported => Err(syn::Error::new(
            unsupported.span(),
            format!("Unsupported type: {:#?}", unsupported),
        )),
    }
}

pub fn implement_global_function(ast: &mut syn::ItemFn) -> syn::Result<TokenStream> {
    todo!()
    // let function_name = ast.sig.ident.clone();

    // eprintln!("FUNCTION NAME \n{:#?}", function_name.to_string());

    // //let trait_name = ast.clone();

    // //ast.items.f

    // let struct_ast = quote! {};

    // Ok(quote!(
    //     #ast
    // ))
}
