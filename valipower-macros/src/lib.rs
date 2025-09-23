// In my-validator-derive/src/lib.rs

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_derive(Validate, attributes(validate))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let field_validators = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let mut field_checks = Vec::new();

        for attribute in &f.attrs {
            if !attribute.path().is_ident("valipower") {
                continue;
            }
            // This parses the comma-separated list inside `validate(...)`
            let nested = attribute
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();

            for meta in nested {
                match meta {
                // Case 1: A simple path validator like `#[validate(email)]`
                    Meta::Path(path) if path.is_ident("email") => {
                        let check = quote! {
                            // `self.#field_name` refers to the field's value
                            if !self.#field_name.contains('@') {
                                 errors.add(stringify!(#field_name), "email", "Must be a valid email address.");
                            }
                        };
                        field_checks.push(check);
                    }

                    // Case 2: A list validator like `#[validate(length(min=3))]`
                    Meta::List(list) if list.path.is_ident("length") => {
                        // This is where you'd parse the arguments `min=3, max=20`
                        // For brevity, we will implement this next
                    }
                    
                    // You would add more `Meta` arms for `range`, `alphanumeric`, etc.

                    _ => {} // Ignore unknown validation rules

                }
            }
        }

        quote! {}
    });

    let expanded = quote! {
        impl valipower::Validate for #name{

            fn validate(&self)-> Result<(),valipower::ValidationErrors>{
                let mut errors = valipower::ValidationErrors::new();

                #(#field_validators)*
                if (errors.is_empty()){
                    Ok(())
                }else{
                    Err(errors)
                }
            }
        }

    };

    // // Here, you would parse the attributes on each field.
    // // This is a complex task involving iterating through `input.data`,
    // // finding fields, and parsing their `attrs`.
    // // For this example, we'll hardcode a check for a field named `username`.
    //
    // // This is a simplified generation logic. A real implementation
    // // would dynamically build this based on parsed attributes.
    // let validation_logic = quote! {
    //     // Let's pretend we parsed an attribute for a field named `username`
    //     if self.username.len() < 3 {
    //         // In a real scenario, you'd populate your `ValidationErrors` struct here
    //         println!("Username is too short!");
    //         // return Err(...);
    //     }
    // };
    //
    // let expanded = quote! {
    //     // Generate the implementation of the `Validate` trait
    //     impl Validate for #name {
    //         fn validate(&self) -> Result<(), ValidationErrors> {
    //             // Insert the validation logic we generated
    //             #validation_logic
    //
    //             // If all checks pass
    //             Ok(())
    //         }
    //     }
    // };

    // Hand the generated code back to the compiler
    TokenStream::from(expanded)
}
