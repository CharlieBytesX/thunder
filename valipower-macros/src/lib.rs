extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, MetaList, MetaNameValue, Token, Type,
    parse_macro_input, punctuated::Punctuated,
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
                        todo!()
                    }
                    Meta::NameValue(name_value)=>{
                        match  &name_value.path{
                            path if path.is_ident("min")=>{
                                field_checks.push(parse_comparison_rule(field_name, &f.ty, &name_value));
                            }
                            path if path.is_ident("max")=>{
                                field_checks.push(parse_comparison_rule(field_name, &f.ty, &name_value));
                            }
                            _ => {
                                panic!("This case for: {} and type: {:#?} is not handled",field_name, name_value
                                    )
                            }
                        }
                    }
                    _ => { /* Ignore unknown rules */ }
                }
            }
            //////////////////////////////////////////////////////
        }
        quote! { #(#field_checks)* }
    });

    let expanded = quote! {
        impl valipower::Validate for #name {
            fn validate(&self) -> Result<(), valipower::ValidationErrors> {
                let mut errors = valipower::ValidationErrors::new();

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

fn parse_comparison_rule(
    field_name: &Ident,
    field_type: &Type,
    nv: &MetaNameValue, // nv stands for NameValue, e.g., `min = 10`
) -> proc_macro2::TokenStream {
    // 1. 🧐 First, check if the field's type is a number.
    // This is crucial for providing clear compile-time errors.
    let supported_numeric_types = [
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
        "f32", "f64",
    ];
    let supported_string_types = ["String"];

    let type_path = match field_type {
        Type::Path(type_path) => {
            if !supported_numeric_types
                .iter()
                .any(|t| type_path.path.is_ident(t))
                && !supported_string_types
                    .iter()
                    .any(|t| type_path.path.is_ident(t))
            {
                panic!(
                    "The `min` and `max` validators can only be applied to numeric or String fields."
                );
            }
            type_path
        }
        _ => {
            panic!("The `min` and `max` validators are not supported for this complex field type.");
        }
    };

    let value = &nv.value;

    if type_path.path.is_ident("String") {
        if nv.path.is_ident("min") {
            return quote! {
                    if self.#field_name.len() < #value {
                    errors.add(
                        stringify!(#field_name),
                        "min",
                        &format!("Length must be at least {}.", #value)
                    );
                }
            };
        }
        if nv.path.is_ident("max") {
            return quote! {
                    if self.#field_name.len() > #value {
                    errors.add(
                        stringify!(#field_name),
                        "min",
                        &format!("Length must be no more than {}.", #value)
                    );
                }
            };
        }
        if nv.path.is_ident("equals") {
            return quote! {
                    if self.#field_name.len() == #value {
                    errors.add(
                        stringify!(#field_name),
                        "min",
                        &format!("Length must be exactly {}.", #value)
                    );
                }
            };
        }
    }

    panic!("unhandled")
}
