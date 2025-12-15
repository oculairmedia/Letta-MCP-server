mod attributes;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    meta, parse_macro_input, parse_quote, Data, DeriveInput, Error, ItemTrait,
};

use self::attributes::Attributes;

/// Derives `Pointee` for the labeled struct which has a trailing DST.
///
/// # Attributes
///
/// Additional arguments can be specified using attributes.
///
/// `#[ptr_meta(...)]` accepts the following arguments:
///
/// - `crate = ...`: Chooses an alternative crate path to import ptr_meta from.
#[proc_macro_derive(Pointee, attributes(ptr_meta))]
pub fn derive_pointee(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    match derive_pointee_impl(derive_input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_pointee_impl(mut input: DeriveInput) -> Result<TokenStream, Error> {
    let attributes = Attributes::parse(&input.attrs)?;
    let ident = &input.ident;
    let crate_path = attributes.crate_path();

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        Data::Enum(_) => {
            return Err(Error::new(
                ident.span(),
                "enums always have a provided `Pointee` impl because they \
                 cannot be dynamically-sized",
            ))
        }
        Data::Union(_) => {
            return Err(Error::new(
                ident.span(),
                "unions always have an provided `Pointee` impl because they \
                 cannot be dynamically-sized",
            ))
        }
    };

    let Some(last_field) = fields.iter().next_back() else {
        return Err(Error::new(
            ident.span(),
            "fieldless structs always have a provided `Poitnee` impl because
            they cannot be dynamically-sized",
        ));
    };
    let last_field_ty = &last_field.ty;

    let where_clause = input.generics.make_where_clause();
    where_clause
        .predicates
        .push(parse_quote! { #last_field_ty: #crate_path::Pointee });

    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    Ok(quote! {
        unsafe impl #impl_generics #crate_path::Pointee for #ident #ty_generics
        #where_clause
        {
            type Metadata = <#last_field_ty as #crate_path::Pointee>::Metadata;
        }
    })
}

/// Generates a `Pointee` implementation for trait object of the labeled trait.
///
/// # Arguments
///
/// `#[pointee(...)]` takes the following arguments:
///
/// - `crate = ...`: Chooses an alternative crate path to import ptr_meta from.
#[proc_macro_attribute]
pub fn pointee(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attributes = Attributes::default();
    let meta_parser = meta::parser(|nested| attributes.parse_meta(nested));

    parse_macro_input!(attr with meta_parser);
    let item = parse_macro_input!(item as ItemTrait);

    match pointee_impl(attributes, item) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn pointee_impl(
    attributes: Attributes,
    item: ItemTrait,
) -> Result<TokenStream, Error> {
    let ident = &item.ident;
    let crate_path = attributes.crate_path();

    let (impl_generics, ty_generics, where_clause) =
        item.generics.split_for_impl();

    Ok(quote! {
        #item

        unsafe impl #impl_generics #crate_path::Pointee for
            (dyn #ident #ty_generics + '_)
        #where_clause
        {
            type Metadata = #crate_path::DynMetadata<Self>;
        }
    })
}
