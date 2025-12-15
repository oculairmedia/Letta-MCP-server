//! Flatten struct derive macro
//!
//! This module provides a derive macro that generates flattened tool implementations
//! from struct definitions while preserving field documentation in the schema.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Generate flattened tool support for a struct
///
/// This macro generates:
/// 1. A constructor that builds the struct from flat parameters
/// 2. A schema generator that produces a flat schema with field descriptions
///
/// # Example
///
/// ```ignore
/// #[derive(FlattenTool, JsonSchema)]
/// struct AgentRequest {
///     /// Operation to perform
///     operation: String,
///     /// Agent ID
///     agent_id: Option<String>,
/// }
/// ```
pub fn derive_flatten_tool(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    // Extract fields from the struct
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "FlattenTool only works with structs that have named fields"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "FlattenTool can only be derived for structs"
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate parameter list with doc comments
    let mut param_list = TokenStream2::new();
    let mut field_assignments = TokenStream2::new();
    let mut first = true;

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        // NOTE: We can't add doc comments to function parameters in Rust
        // Function parameters don't support /// doc comments
        // The struct field doc comments are preserved in the struct definition itself

        if !first {
            param_list.extend(quote! { , });
        }
        first = false;

        // Just add the parameter without doc comments
        param_list.extend(quote! {
            #field_name: #field_ty
        });

        field_assignments.extend(quote! {
            #field_name,
        });
    }

    // Generate the flattened constructor method
    let method_name = syn::Ident::new(
        &format!("{}_from_flat", struct_name.to_string().to_lowercase()),
        struct_name.span(),
    );

    // Generate schema method name
    let schema_method_name = syn::Ident::new(
        &format!("{}_flat_schema", struct_name.to_string().to_lowercase()),
        struct_name.span(),
    );

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Construct struct from flattened parameters
            #vis fn #method_name(#param_list) -> Self {
                Self {
                    #field_assignments
                }
            }

            /// Generate flat JSON schema with field descriptions
            #[cfg(feature = "schemars")]
            #vis fn #schema_method_name() -> ::serde_json::Value {
                use schemars::{JsonSchema, schema_for};

                // Get the struct's schema (which includes field descriptions)
                let root_schema = schema_for!(Self);

                // Convert to JSON Value for easier manipulation
                let schema_value = ::serde_json::to_value(&root_schema).unwrap();

                // Extract the schema object
                if let Some(schema_obj) = schema_value.as_object() {
                    if let Some(schema_def) = schema_obj.get("schema") {
                        // Return the schema definition directly, which should be flat
                        return schema_def.clone();
                    }
                }

                // Fallback: return the whole schema
                schema_value
            }
        }
    };

    TokenStream::from(expanded)
}
