mod der_macro;
mod wit_gen;

use proc_macro::TokenStream;
use syn::*;

/**
   #[derive(WIT)]
   #[wit(WitPerson)]
   pub struct Person {

       pub name: String,

       #[rename("age2")]
       pub age: i32
   }
*/
#[proc_macro_derive(WIT, attributes(wit, rename))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    der_macro::expand_wit(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/**
   #[golem_rust::create_wit_file]
   mod golem_component {

       enum IpAddr {
           V4(String),
           V6(String),
       }

       pub struct BidderId {
           pub bidder_id: String,
           pub verified: bool
       }

       trait AuctionService {

           fn create_bidder(full_name: String, address: String, age: u16) -> BidderId;
       }
   }
*/
#[proc_macro_attribute]
pub fn create_wit_file(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_moved = item.clone();

    let mut input = parse_macro_input!(item_moved as ItemMod);

    let file_name_result = syn::parse2::<syn::Lit>(_attr.into())
        .map_or_else(|_| {
            Ok("generated.wit".to_owned())
        },
        |literal| {
            match literal {
                syn::Lit::Str(lit) => {
                    let mut n = lit.value();
                    if n.ends_with(".wit") {
                        Ok(n)
                    } else {
                        n.push_str(".wit");
                        Ok(n)
                    }
                },
                _ =>  Err(syn::Error::new(literal.span(), "If you want to specify name of the generated file, please input is as a String, otherwise do not input any attributes. \n Generated file will be 'generated.wit'")),
            }
        });

    file_name_result
        .and_then(|file_name| wit_gen::generate_witfile(&mut input, file_name).to_owned())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
