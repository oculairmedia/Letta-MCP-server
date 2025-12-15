//! Utility functions for TurboMCP procedural macros

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generate schema implementation by delegating to the schema system
pub fn generate_schema_impl(input: DeriveInput, method_name: &str) -> TokenStream {
    let name = &input.ident;
    let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
    
    TokenStream::from(quote! {
        impl #name {
            pub fn #method_ident() -> serde_json::Value {
                // Delegate to the schema generation system
                turbomcp::schema::json_schema_for::<Self>()
            }
        }
    })
}

/// Generate tool input schema implementation
pub fn generate_tool_input_schema(input: DeriveInput) -> TokenStream {
    generate_schema_impl(input, "schema")
}

/// Generate prompt args schema implementation
pub fn generate_prompt_args_schema(input: DeriveInput) -> TokenStream {
    generate_schema_impl(input, "prompt_schema")
}

