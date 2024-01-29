use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::prelude::*;
use syn::spanned::Spanned;
use syn::*;

pub fn generate_witfile(ast: &mut syn::ItemMod, path: String) -> syn::Result<TokenStream> {
    eprintln!("{:#?}", ast.clone());

    // create file
    let mut file = File::create(path).map_err(|e| {
        syn::Error::new(
            ast.span(),
            format!("Cannot create with file at requested location {}", e),
        )
    })?;

    let package_name = ast
        .clone()
        .ident
        .to_string()
        .to_lowercase()
        .replace("_", ":");

    let items = ast.clone().content.unwrap().1;

    let interface_content: syn::Result<Vec<_>> = items
        .into_iter()
        .map(|item| {
            match item {
                Item::Struct(i) => {
                    let ident = i.ident.to_string();

                    let record_title = pascal_case_to_kebab_case(ident);

                    check_unsupported_identifiers(record_title.clone());

                    let fields = i
                        .fields
                        .into_iter()
                        .map(|f| {
                            let field_name = f
                                .ident
                                .unwrap()
                                .to_string()
                                .to_lowercase()
                                .replace("_", "-");

                            let tpe = resolve_type(f.ty);

                            format!("{}: {}", field_name, tpe)
                        })
                        .collect::<Vec<String>>()
                        .join(", \n\t\t");

                    if fields.is_empty() {
                        Ok(format!("    record {} {{}}", record_title))
                    } else {
                        Ok(format!(
                            "
    record {} {{
        {},
    }}",
                            record_title, fields
                        ))
                    }
                }
                Item::Trait(i) => {
                    // ignored - we probably don't care about a trait name
                    let _ = pascal_case_to_kebab_case(i.ident.to_string());

                    Ok(i.items
                        .into_iter()
                        .map(|trait_item| {
                            let (fun_title, params, ret_tpe) = match trait_item {
                                TraitItem::Fn(tif) => {
                                    let signature = tif.sig.clone();

                                    let fun_title = signature
                                        .ident
                                        .to_string()
                                        .to_lowercase()
                                        .replace("_", "-");

                                    let ret_tpe = extract_return_type(signature.output);

                                    let params = signature
                                        .inputs
                                        .into_iter()
                                        .map(|arg| match arg {
                                            FnArg::Typed(pat_type) => pat_type_to_param(pat_type),
                                            FnArg::Receiver(_) => {
                                                panic!("do proper error handling later")
                                            }
                                        })
                                        .collect::<Vec<String>>()
                                        .join(", ");

                                    (fun_title, params, ret_tpe)
                                }
                                _ => panic!("unsupported"),
                            };
                            if ret_tpe.is_empty() {
                                format!(
                                    "
    {}: func({})
                ",
                                    fun_title, params
                                )
                            } else {
                                format!(
                                    "
    {}: func({}) -> {}
                ",
                                    fun_title, params, ret_tpe
                                )
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n"))
                }
                // Do we need to distinguish between WIT enum and variant ?
                Item::Enum(item_enum) => {
                    let variant_title = pascal_case_to_kebab_case(item_enum.ident.to_string());

                    let variant_body = item_enum
                        .variants
                        .into_iter()
                        .map(|variant| {
                            let variant_name = pascal_case_to_kebab_case(variant.ident.to_string());

                            match variant.fields {
                                Fields::Named(named_fields) => {
                                    let tpes = named_fields
                                        .named
                                        .into_iter()
                                        .map(|f| resolve_type(f.ty))
                                        .collect::<Vec<String>>()
                                        .join(", ");

                                    format!("{}({})", variant_name, tpes)
                                }
                                Fields::Unit => variant_name,
                                Fields::Unnamed(fields) => {
                                    let tpes = fields
                                        .unnamed
                                        .into_iter()
                                        .map(|f| resolve_type(f.ty))
                                        .collect::<Vec<String>>()
                                        .join(", ");

                                    format!("{}({})", variant_name, tpes)
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", \n \t\t");

                    Ok(format!(
                        "
    variant {} {{
        {}
    }}
                ",
                        variant_title, variant_body
                    ))
                }
                a => Err(syn::Error::new(
                    ast.ident.span(),
                    format!("Unknown item in module - {:#?}", a),
                )),
            }
        })
        .collect();

    file.write_all(
        format!(
            "package {}

interface api {{
{}
}}

world golem-service {{
    export api
}}",
            package_name,
            interface_content.unwrap().join("\n")
        )
        .trim()
        .as_bytes(),
    )
    .map_err(|e| syn::Error::new(ast.span(), format!("Error while writing to file {}", e)))?;

    // don't do anything with ast
    let result = quote!(#ast);
    Ok(result)
}

// AuctionService -> auction-service
fn pascal_case_to_kebab_case(pascal_case: String) -> String {
    let mut record_title = pascal_case.chars();

    let mut first_letter = record_title.nth(0).unwrap().to_lowercase().to_string();
    let rest = record_title
        .into_iter()
        .map(|ch| {
            if ch.is_uppercase() {
                format!("-{}", ch.to_lowercase())
            } else {
                ch.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");

    first_letter.push_str(&rest);

    first_letter
}

fn extract_return_type(return_type: ReturnType) -> String {
    match return_type {
        ReturnType::Default => "".to_owned(),
        ReturnType::Type(_, tpe) => resolve_type(*tpe),
    }
}

// full_name: String to full-name: string for trait functions
fn pat_type_to_param(pat_type: PatType) -> String {
    let pat = pat_type.pat;

    let mut param_name = match *pat {
        Pat::Ident(i) => i.ident.to_string().to_lowercase().replace("_", "-"),
        _ => panic!("unsupported"),
    };

    let param_tpe = resolve_type(*pat_type.ty);

    param_name.push_str(": ");
    param_name.push_str(&param_tpe);
    param_name
}

fn convert_rust_types_to_wit_types(rust_tpe: String) -> String {
    match rust_tpe.as_str() {
        "bool" => "bool".to_owned(),
        "i8" => "s8".to_owned(),
        "i16" => "s16".to_owned(),
        "i32" => "s32".to_owned(),
        "i64" => "s64".to_owned(),
        "isize" => "s64".to_owned(),
        "u8" => "u8".to_owned(),
        "u16" => "u16".to_owned(),
        "u32" => "u32".to_owned(),
        "u64" => "u64".to_owned(),
        "usize" => "u64".to_owned(),
        "f32" => "float32".to_owned(),
        "f64" => "float64".to_owned(),
        "String" => "string".to_owned(),
        "char" => "char".to_owned(),
        x => pascal_case_to_kebab_case(x.to_owned()), //panic!("is better to pani or return result with error?") // return error here
    }
}

fn check_unsupported_identifiers(name: String) {
    match name.as_str() {
        "option" => panic!("expected an identifier or string, found keyword `option`"),
        "result" => panic!("expected an identifier or string, found keyword `result`"),
        _ => (),
    };
}

// https://component-model.bytecodealliance.org/design/wit.html?search=#built-in-types
// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn resolve_type(ty: Type) -> String {
    match ty {
        Type::Path(type_path) => {
            if type_path.path.segments.first().unwrap().ident.to_string() == "super" {
                panic!("Types need to be defined inside module.")
            }

            // we take last segment e.g. Result from std::result::Result
            let path_segment = type_path.path.segments.last().unwrap();
            if path_segment.ident.to_string() == "Box" {
                match &path_segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        let gen_arg = args.args.first().unwrap();
                        match gen_arg {
                            GenericArgument::Type(tpe) => resolve_type(tpe.clone()),
                            _ => panic!("unhandled"),
                        }
                    }
                    _ => panic!("unhandled"),
                }
            } else if let (PathArguments::AngleBracketed(args), true) = (
                &path_segment.arguments,
                path_segment.ident.to_string() == "Vec",
            ) {
                // vector has only one type param
                let gen_arg = args.args.first().unwrap();
                match gen_arg {
                    GenericArgument::Type(tpe) => {
                        let tpe_name = resolve_type(tpe.clone());

                        format!("list<{}>", tpe_name)
                    }
                    _ => panic!("unhandled"),
                }
            } else if let (PathArguments::AngleBracketed(args), true) = (
                &path_segment.arguments,
                path_segment.ident.to_string() == "Result",
            ) {
                let result_arguments = args
                    .clone()
                    .args
                    .into_iter()
                    .map(|a| match a {
                        GenericArgument::Type(tpe) => resolve_type(tpe.clone()),
                        _ => panic!("unhandled"),
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("result<{}>", result_arguments)
            } else if let (PathArguments::AngleBracketed(args), true) = (
                &path_segment.arguments,
                path_segment.ident.to_string() == "Option",
            ) {
                let gen_arg = args.args.first().unwrap();
                match gen_arg {
                    GenericArgument::Type(tpe) => {
                        let tpe_name = resolve_type(tpe.clone());

                        format!("option<{}>", tpe_name)
                    }
                    _ => panic!("unhandled"),
                }
            } else {
                convert_rust_types_to_wit_types(path_segment.ident.to_string())
            }
        }
        Type::Tuple(tuple_type) => {
            let tuples = tuple_type
                .elems
                .into_iter()
                .map(|tpe| resolve_type(tpe))
                .collect::<Vec<String>>()
                .join(", ");

            if tuples.is_empty() {
                "".to_string()
            } else {
                format!("tuple<{tuples}>")
            }
        }
        Type::Slice(type_slice) => {
            format!("list<{}>", resolve_type(*type_slice.elem))
        }
        _ => "".to_owned(),
    }
}
