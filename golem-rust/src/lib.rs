mod der_macro;
mod wit_gen;

use proc_macro::TokenStream;
use syn::*;

/**
 * Usage:
 *      
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
 * 
 * 

  TODO
  - proper error handling
  - do not generate by every compilation

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

    wit_gen::generate_witfile(&mut input, "../generated.wit".to_owned())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}