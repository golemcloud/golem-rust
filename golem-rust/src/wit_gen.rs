use proc_macro2::{Span, TokenStream};
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

    let module_content: syn::Result<Vec<_>> = items
        .into_iter()
        .map(|item| {
            match item {
                Item::Struct(i) => {
                    let ident = i.ident.to_string();

                    let record_title = pascal_case_to_kebab_case(ident);

                    check_unsupported_identifiers(record_title.clone(), i.ident.span())?;
                        
                    let fields: syn::Result<Vec<_>> = i
                    .fields
                    .into_iter()
                    .map(|f| {
                        let field_name = f
                            .ident
                            .unwrap()
                            .to_string()
                            .to_lowercase()
                            .replace("_", "-");

                        resolve_type(f.ty).map(|tpe| format!("{}: {}", field_name, tpe))
                    })
                    .collect();
                    let joined = fields?.join(", \n\t\t");

                    if joined.is_empty() {
                        Ok(format!("    record {} {{}}", record_title))
                    } else {
                        Ok(format!(
                        "
    record {} {{
        {},
    }}",
                        record_title, joined
                        ))
                    }
                }
                Item::Trait(i) => {
                    // ignored - we probably don't care about a trait name
                    let _ = pascal_case_to_kebab_case(i.ident.to_string());

                    let contents: syn::Result<Vec<_>> = i
                        .items
                        .into_iter()
                        .map(|trait_item| {
                            let y = match trait_item {
                                TraitItem::Fn(tif) => {
                                    let signature = tif.sig.clone();

                                    let fun_title = signature
                                        .ident
                                        .to_string()
                                        .to_lowercase()
                                        .replace("_", "-");

                                    let ret_tpe = extract_return_type(signature.output);

                                    let params: syn::Result<Vec<_>> = signature
                                        .inputs
                                        .into_iter()
                                        .map(|arg| match arg {
                                            FnArg::Typed(pat_type) => pat_type_to_param(pat_type),
                                            FnArg::Receiver(_) => {
                                                Err(syn::Error::new(arg.span(), "'sefl' not supported. If you think this should be supported, please open an issue https://github.com/golemcloud/golem-rust/issues"))
                                            }
                                        })
                                        .collect();

                                    let pars = params?;
                                    let ret_tp = ret_tpe?;

                                    Ok((fun_title, pars.join(", "), ret_tp))
                                }
                                _ => Err(syn::Error::new(trait_item.span(), "Unknown element inside trait. If you think this should be supported, please open an issue https://github.com/golemcloud/golem-rust/issues")),
                            };

                            y.map(|(fun_title, params, ret_tpe)| {
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
                        })
                        .collect();

                    Ok(contents?.join("\n"))
                }
                // Do we need to distinguish between WIT enum and variant ?
                Item::Enum(item_enum) => {
                    let variant_title = pascal_case_to_kebab_case(item_enum.ident.to_string());

                    let variant_body: syn::Result<Vec<_>> = item_enum
                        .variants
                        .into_iter()
                        .map(|variant| {
                            let variant_name = pascal_case_to_kebab_case(variant.ident.to_string());

                            match variant.fields {
                                Fields::Named(named_fields) => {
                                    let tpes: syn::Result<Vec<_>> = named_fields
                                        .named
                                        .into_iter()
                                        .map(|f| resolve_type(f.ty))
                                        .collect();

                                    tpes.map(|t| format!("{}({})", variant_name, t.join(", ")))
                                }
                                Fields::Unit => Ok(variant_name),
                                Fields::Unnamed(fields) => {
                                    let tpes: syn::Result<Vec<_>> = fields
                                        .unnamed
                                        .into_iter()
                                        .map(|f| resolve_type(f.ty))
                                        .collect();

                                    tpes.map(|t| format!("{}({})", variant_name, t.join(", ")))
                                }
                            }
                        })
                        .collect();

                    let var_body = variant_body?.join(", \n \t\t");

                    Ok(format!(
                        "
    variant {} {{
        {}
    }}
                ",
                        variant_title, var_body
                    ))
                },
                Item::Use(use_item) => Err(syn::Error::new(
                    use_item.span(),
                    format!("'use' imports are not supportedin create_wit_file macro."),
                )),
                Item::Const(const_item) => Err(syn::Error::new(
                    const_item.span(),
                    format!("Constants are not supported in create_wit_file macro."),
                )),
                a => Err(syn::Error::new(
                    ast.ident.span(),
                    format!("Unknown item in module - {:#?}", a),
                )),
            }
        })
        .collect();

    module_content
        .and_then(|content| {
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
                    content.join("\n")
                )
                .trim()
                .as_bytes(),
            )
            .map_err(|e| syn::Error::new(ast.span(), format!("Error while writing to file {}", e)))
        })
        .map(|_| {
            let result = quote!(#ast);
            // don't do anything with ast
            result
        })
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

fn extract_return_type(return_type: ReturnType) -> syn::Result<String> {
    match return_type {
        ReturnType::Default => Ok("".to_owned()),
        ReturnType::Type(_, tpe) => resolve_type(*tpe),
    }
}

// full_name: String to full-name: string for trait functions
fn pat_type_to_param(pat_type: PatType) -> syn::Result<String> {
    let pat = pat_type.clone().pat;

    let param_name = match *pat {
        Pat::Ident(i) => Ok(i.ident.to_string().to_lowercase().replace("_", "-")),
        _ => Err(syn::Error::new(pat_type.span(), "Unexpected param name. If you think this should be supported, please open an issue https://github.com/golemcloud/golem-rust/issues")),
    };

    let mut name = param_name?;

    let param_tpe = resolve_type(*pat_type.ty);
    let tpe = param_tpe?;

    name.push_str(": ");
    name.push_str(&tpe);
    Ok(name)
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
        x => pascal_case_to_kebab_case(x.to_owned()),
    }
}

fn check_unsupported_identifiers(name: String, span: Span) -> syn::Result<()> {
    match name.as_str() {
        "option" => Err(syn::Error::new(
            span,
            "expected an identifier or string, found keyword `option'",
        )),
        "result" => Err(syn::Error::new(
            span,
            "expected an identifier or string, found keyword `result`",
        )),
        _ => Ok(()),
    }
}

// https://component-model.bytecodealliance.org/design/wit.html?search=#built-in-types
// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn resolve_type(ty: Type) -> syn::Result<String> {
    match ty.clone() {
        Type::Path(type_path) => {
            if type_path.path.segments.first().unwrap().ident.to_string() == "super" {
                return Err(syn::Error::new(
                    ty.span(),
                    "Cannot reference types from outside of module.",
                ));
            }

            // we take last segment e.g. Result from std::result::Result
            let path_segment = type_path.path.segments.last().unwrap();
            if path_segment.ident.to_string() == "Box" {
                match &path_segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        let gen_arg = args.args.first().unwrap();
                        match gen_arg {
                            GenericArgument::Type(tpe) => resolve_type(tpe.clone()),
                            _ => Err(syn::Error::new(ty.span(), "Unexpected error. If you think this should work, please open an issue https://github.com/golemcloud/golem-rust/issues")),
                        }
                    }
                    _ => Err(syn::Error::new(ty.span(), "Unexpected error. If you think this should work, please open an issue https://github.com/golemcloud/golem-rust/issues")),
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

                        tpe_name.map(|t| format!("list<{}>", t))
                    }
                    _ => Err(syn::Error::new(ty.span(), "Unexpected error. If you think this should work, please open an issue https://github.com/golemcloud/golem-rust/issues")),
                }
            } else if let (PathArguments::AngleBracketed(args), true) = (
                &path_segment.arguments,
                path_segment.ident.to_string() == "Result",
            ) {
                let result_arguments: syn::Result<Vec<_>> = args
                    .clone()
                    .args
                    .into_iter()
                    .map(|a| match a {
                        GenericArgument::Type(tpe) => resolve_type(tpe.clone()),
                        _ => Err(syn::Error::new(ty.span(), "Unexpected error. If you think this should work, please open an issue https://github.com/golemcloud/golem-rust/issues")),
                    })
                    .collect();

                result_arguments.map(|c| format!("result<{}>", c.join(", ")))
            } else if let (PathArguments::AngleBracketed(args), true) = (
                &path_segment.arguments,
                path_segment.ident.to_string() == "Option",
            ) {
                let gen_arg = args.args.first().unwrap();
                match gen_arg {
                    GenericArgument::Type(tpe) => {
                        let tpe_name = resolve_type(tpe.clone());

                        tpe_name.map(|t| format!("option<{}>", t))
                    }
                    _ => Err(syn::Error::new(ty.span(), "Unexpected error. If you think this should work, please open an issue https://github.com/golemcloud/golem-rust/issues")),
                }
            } else {
                Ok(convert_rust_types_to_wit_types(
                    path_segment.ident.to_string(),
                ))
            }
        }
        Type::Tuple(tuple_type) => {
            let ts: syn::Result<Vec<_>> = tuple_type
                .elems
                .into_iter()
                .map(|tpe| resolve_type(tpe))
                .collect();

            ts.map(|c| {
                let t = c.join("\n");
                if t.is_empty() {
                    "".to_string()
                } else {
                    format!("tuple<{}>", t)
                }
            })
        }
        Type::Slice(type_slice) => resolve_type(*type_slice.elem).map(|t| format!("list<{}>", t)),
        _ => Ok("".to_owned()),
    }
}
