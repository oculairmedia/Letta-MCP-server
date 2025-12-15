//! Flatten tool helper for expanding struct parameters into individual parameters
//!
//! This module provides support for flattening struct-based tool parameters into
//! individual primitive parameters while preserving field doc comments as parameter descriptions.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_quote};

/// Information about a flattened struct field
pub struct FlattenedField {
    pub name: String,
    pub ty: Type,
    pub doc: Option<String>,
    pub is_optional: bool,
}

/// Extract fields from a struct definition
pub fn extract_struct_fields(input: &DeriveInput) -> Result<Vec<FlattenedField>, syn::Error> {
    let mut fields = Vec::new();

    match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    for field in &fields_named.named {
                        let field_name = field.ident.as_ref()
                            .ok_or_else(|| syn::Error::new_spanned(field, "Field must have a name"))?
                            .to_string();

                        // Extract doc comments from attributes
                        let doc = extract_doc_comments(&field.attrs);

                        // Check if field is Option<T>
                        let is_optional = is_option_type(&field.ty);

                        fields.push(FlattenedField {
                            name: field_name,
                            ty: field.ty.clone(),
                            doc,
                            is_optional,
                        });
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        input,
                        "Only structs with named fields can be flattened"
                    ));
                }
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Only structs can be flattened"
            ));
        }
    }

    Ok(fields)
}

/// Extract doc comments from attributes
fn extract_doc_comments(attrs: &[syn::Attribute]) -> Option<String> {
    let mut docs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        let doc_str = lit_str.value();
                        // Trim leading space that rustdoc adds
                        let trimmed = doc_str.trim_start();
                        docs.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    if docs.is_empty() {
        None
    } else {
        Some(docs.join(" "))
    }
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Option"
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Generate code to construct struct from flat parameters
pub fn generate_struct_construction(
    struct_name: &syn::Ident,
    fields: &[FlattenedField],
) -> TokenStream2 {
    let field_names: Vec<syn::Ident> = fields
        .iter()
        .map(|f| syn::Ident::new(&f.name, proc_macro2::Span::call_site()))
        .collect();

    quote! {
        #struct_name {
            #(#field_names),*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_struct_fields() {
        let input: DeriveInput = parse_quote! {
            struct TestRequest {
                /// Operation to perform
                operation: String,
                /// Optional agent ID
                agent_id: Option<String>,
            }
        };

        let fields = extract_struct_fields(&input).unwrap();
        assert_eq!(fields.len(), 2);

        assert_eq!(fields[0].name, "operation");
        assert_eq!(fields[0].doc.as_deref(), Some("Operation to perform"));
        assert!(!fields[0].is_optional);

        assert_eq!(fields[1].name, "agent_id");
        assert_eq!(fields[1].doc.as_deref(), Some("Optional agent ID"));
        assert!(fields[1].is_optional);
    }
}
