extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, MetaList, Token, parse_macro_input,
    punctuated::Punctuated,
};

#[proc_macro_derive(Validate, attributes(validate))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => panic!("Validate can only be derived for structs"),
    };

    let field_validators = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().expect("Field must have a name");
        let mut field_checks = Vec::new();

        for attr in &f.attrs {
            if !attr.path().is_ident("validate") {
                continue;
            }

            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .expect("Failed to parse validation rules");

            ////////////////////////////////////////////////////////
            // IMPLEMENT VALIDATION TYPES
            for meta in nested {
                match meta {
                    Meta::Path(path) if path.is_ident("email") => {
                        field_checks.push(quote! {
                            if !self.#field_name.contains('@') {
                                errors.add(stringify!(#field_name), "email", "Must be a valid email address.");
                            }
                        });
                    }
                    Meta::List(list) if list.path.is_ident("length") => {
                        field_checks.push(parse_length_rule(field_name, &list));
                    }
                    _ => { /* Ignore unknown rules */ }
                }
            }
            //////////////////////////////////////////////////////
        }
        quote! { #(#field_checks)* }
    });

    let expanded = quote! {
        impl my_validator::Validate for #name {
            fn validate(&self) -> Result<(), my_validator::ValidationErrors> {
                let mut errors = my_validator::ValidationErrors::new();

                #(#field_validators)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_length_rule(field_name: &Ident, list: &MetaList) -> proc_macro2::TokenStream {
    let mut min_val: Option<usize> = None;
    let mut max_val: Option<usize> = None;

    let nested = list
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .unwrap();

    for meta in nested {
        if let Meta::NameValue(nv) = meta {
            if let Expr::Lit(expr_lit) = nv.value {
                if let Lit::Int(lit_int) = expr_lit.lit {
                    if nv.path.is_ident("min") {
                        min_val = Some(lit_int.base10_parse().unwrap());
                    } else if nv.path.is_ident("max") {
                        max_val = Some(lit_int.base10_parse().unwrap());
                    }
                }
            }
        }
    }

    let mut checks = vec![];
    if let Some(min) = min_val {
        checks.push(quote! {
            if self.#field_name.len() < #min {
                errors.add(stringify!(#field_name), "length", &format!("Length must be at least {}.", #min));
            }
        });
    }
    if let Some(max) = max_val {
        checks.push(quote! {
            if self.#field_name.len() > #max {
                errors.add(stringify!(#field_name), "length", &format!("Length must be no more than {}.", #max));
            }
        });
    }

    quote! { #(#checks)* }
}
