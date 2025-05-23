// Copyright 2024-2025 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro::TokenStream;

use syn::*;

use crate::transaction::golem_operation_impl;

mod expand;
mod transaction;
mod value;
mod wit_gen;

#[proc_macro_derive(IntoValue, attributes(flatten_value, unit_case))]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    value::derive_into_value(input)
}

#[proc_macro_derive(FromValueAndType, attributes(flatten_value, unit_case))]
pub fn derive_from_value_and_type(input: TokenStream) -> TokenStream {
    value::derive_from_value_and_type(input)
}

/// Derives `From<>` And `Into<>` typeclasses for wit-bindgen generated data types (e.g. `WitPerson`)
/// and custom domain data types (e.g. `Person`). So it's possible to write code like this:
/// ```ignore
///  let person = Person {
///     name: "John Doe".to_owned(),
///     age: 42,
///  };
///
///  let converted: WitPerson = person.into();
/// ```
///
/// `#[wit_type_name(WitPerson)]` is optional. Defines name of the wit-bindgen generated data type. Default is name of this data type prepanded with 'Wit'.
///
/// `#[rename_field("age2")]` is optional. Anotates field and specify field name in the other data type, in case it's different.
///
/// # Example:
/// ```
///  pub struct WitPerson {
///      pub title: String,
///      pub age: i32,
///  }
///
///
///  #[derive(golem_rust_macro::WIT_From_Into)]
///  #[wit_type_name(WitPerson)]
///  pub struct Person {
///
///      #[rename_field("title")]
///      pub name: String,
///      
///      pub age: i32
///  }
/// ```
#[proc_macro_derive(WIT_From_Into, attributes(wit_type_name, rename_field))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    expand::expand_wit(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Annotates a module with `#[golem_rust_macro::create_wit_file]` and generates WIT file in the root of your project.
/// Supports enums, structs, traits and alias types.
///
/// # Example:
/// ```
///  #[golem_rust_macro::create_wit_file("auction_app.wit")]
///  mod auction_app {
///  
///      struct BidderId {
///          bidder_id: String,
///      }
///  
///      struct AuctionId {
///          auction_id: String,
///      }
///  
///      struct Auction {
///          auction_id: Option<AuctionId>,
///          name: String,
///          description: String,
///          starting_price: f32,
///          deadline: Deadline,
///      }
///  
///      enum BidResult {
///          Failure(String),
///          Success,
///      }
///  
///      type Deadline = u64;
///  
///      trait AuctionService {
///          fn initialize(auction: Auction);
///  
///          fn bid(bidder_id: BidderId, price: f32) -> BidResult;
///  
///          fn close_auction() -> Option<BidderId>;
///  
///          fn create_bidder(name: String, address: String) -> BidderId;
///  
///          fn create_auction(
///              name: String,
///              description: String,
///              starting_price: f32,
///              deadline: u64,
///          ) -> AuctionId;
///  
///          fn get_auctions() -> Vec<Auction>;
///      }
///  }
/// ```
///
/// File `auction_app.wit` is then created with the following content.
///
/// ```ignore
/// package auction:app
///
/// interface api {
///     
///     record bidder-id {
///         bidder-id: string,
///     }
///
///     record auction-id {
///         auction-id: string,
///     }
///
///     record auction {
///         auction-id: option<auction-id>,
///         name: string,
///         description: string,
///         starting-price: float32,
///         deadline: deadline,
///     }
///
///     variant bid-result {
///         failure(string),
///         success
///     }
///                 
///     type deadline = u64
///
///     initialize: func(auction: auction)
///
///     bid: func(bidder-id: bidder-id, price: float32) -> bid-result
///
///     close-auction: func() -> option<bidder-id>
///
///     create-bidder: func(name: string, address: string) -> bidder-id
///
///     create-auction: func(name: string, description: string, starting-price: float32, deadline: u64) -> auction-id
///
///     get-auctions: func() -> list<auction>
/// }
///
/// world golem-service {
///     export api
/// }
///  ```
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

/// Defines a function as an `Operation` that can be used in transactions
#[proc_macro_attribute]
pub fn golem_operation(attr: TokenStream, item: TokenStream) -> TokenStream {
    golem_operation_impl(attr, item)
}
