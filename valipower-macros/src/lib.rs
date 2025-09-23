// In my-validator-derive/src/lib.rs

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Validate, attributes(validate))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        syn::Data::Enum(data_enum) => todo!(),
        syn::Data::Union(data_union) => todo!(),
    };

    let field_validators = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        quote! {}
    });

    let expaned = quote! {
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
