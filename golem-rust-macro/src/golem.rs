use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;

pub fn implement_struct(ast: &mut syn::ItemStruct) -> syn::Result<TokenStream> {

    let struct_name = ast.ident.clone();

    eprintln!("STRUCT NAME ${:#?}", struct_name);

    let wittable = quote!(

      //  use ::golem_rust::Wittable;

        impl ::golem_rust::Wittable for #struct_name {
            fn to_wit() -> ::golem_rust::WitContent {
                ::golem_rust::WitContent::Str2
            }
            
        }
    );

    Ok(quote!(
        #ast 
        #wittable
    ))
}

pub fn implement_wittable(ast: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    Ok(ast.into_token_stream())
}

pub fn create_wit(ast: &mut syn::ItemTrait) -> syn::Result<TokenStream> {
    Ok(ast.into_token_stream())
}