use proc_macro2::TokenStream;
use quote::quote;

pub fn implement_struct(ast: &mut syn::ItemStruct) -> syn::Result<TokenStream> {

    let struct_name = ast.ident.clone();

    // let wittable = quote!(

    //     impl ::golem_rust::Wittable for #struct_name {
    //         fn to_wit() -> ::golem_rust::WitContent {
    //             ::golem_rust::WitContent::Product("
    //             record house {
    //                 address: string,
    //             }".to_owned())
    //         }
            
    //     }
    // );

    Ok(quote!(
        #ast 
       // #wittable
    ))
}

pub fn implement_trait(ast: &mut syn::ItemTrait) -> syn::Result<TokenStream> {

    eprintln!("TRAIT {:#?}", ast);

    let trait_name = ast.ident.clone();

    //ast.items.f

    Ok(quote!(
        #ast
    ))
}

