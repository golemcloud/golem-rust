// Copyright 2024 Golem Cloud
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

use quote::ToTokens;
use syn::*;

use crate::transaction::golem_operation_impl;

mod expand;
mod transaction;
mod golem;

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

/// Defines a function as an `Operation` that can be used in transactions
#[proc_macro_attribute]
pub fn golem_operation(attr: TokenStream, item: TokenStream) -> TokenStream {
    golem_operation_impl(attr, item)
}

#[proc_macro_attribute]
pub fn golem(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let struct_tokens = parse::<ItemStruct>(item.clone()).and_then(|mut t| {
        
        golem::implement_struct(&mut t)
    });

    let global_function = parse::<ItemFn>(item.clone()).and_then(|mut t| {

        golem::implement_global_function(&mut t)
    });

    let type_tokens = parse::<ItemType>(item.clone()).and_then(|mut t| {
        eprintln!("ÌTEM TYPE \n{:#?}", t.clone());

        Ok(t.to_token_stream())
    });

    let enum_tokens = parse::<ItemEnum>(item.clone()).and_then(|mut t| {
        eprintln!("ENUM TYPE \n{:#?}", t.clone());

        Ok(t.to_token_stream())
    });

    let struct_impl = parse::<ItemImpl>(item.clone()).and_then(|mut t| {
        eprintln!("STRUCT IMPL TYPE \n{:#?}", t.clone());

        Ok(t.to_token_stream())
    });
    
    global_function
        .or_else(|_| type_tokens)
        .or_else(|_| struct_tokens)
        .or_else(|_| enum_tokens)
        .or_else(|_| struct_impl)
        .or_else(|_| Err(Error::new(
            proc_macro2::Span::call_site(),
            "Use golem annotation only to annotate: top level functions, structs, enums and type aliases."
        )))
        .unwrap_or_else(syn::Error::into_compile_error).into()
}