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

/*
struct GetAddress {}

impl IntoWitMetadata for GetAddress {

    fn ident() -> &'static str {
        "get_address"
    }

    fn as_wit() -> WitMeta {
       WitMeta::Function(
        FunctionMeta {
            name: "get_address".to_owned(),
            args: vec![],
            result: Box::new(Address::as_wit())
        })
    }

    #[distributed_slice(ALL_WIT_TYPES_FOR_GOLEM)]
    static FUN_WIT: fn() -> WitMeta = || GetAddress::as_wit();

 }

 */

pub fn implement_global_function(ast: &mut syn::ItemFn) -> syn::Result<TokenStream> {
    let function_name = ast.sig.ident.clone();

    eprintln!("FUNCTION NAME \n{:#?}", function_name.to_string());

    //let trait_name = ast.clone();

    //ast.items.f

    let struct_ast = quote! {};

    Ok(quote!(
        #ast
    ))
}
