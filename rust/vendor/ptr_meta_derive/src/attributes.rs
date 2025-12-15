use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta, parse_quote, AttrStyle, Attribute, Error, Path,
    Token,
};

fn try_set_attribute<T: ToTokens>(
    attribute: &mut Option<T>,
    value: T,
    name: &'static str,
) -> Result<(), Error> {
    if attribute.is_none() {
        *attribute = Some(value);
        Ok(())
    } else {
        Err(Error::new_spanned(
            value,
            format!("{name} already specified"),
        ))
    }
}

#[derive(Default)]
pub struct Attributes {
    crate_path: Option<Path>,
}

impl Attributes {
    pub fn parse_meta(
        &mut self,
        meta: ParseNestedMeta<'_>,
    ) -> Result<(), Error> {
        if meta.path.is_ident("crate") {
            if meta.input.parse::<Token![=]>().is_ok() {
                let path = meta.input.parse::<Path>()?;
                try_set_attribute(&mut self.crate_path, path, "crate")
            } else if meta.input.is_empty() || meta.input.peek(Token![,]) {
                try_set_attribute(
                    &mut self.crate_path,
                    parse_quote! { crate },
                    "crate",
                )
            } else {
                Err(meta.error("expected `crate` or `crate = ...`"))
            }
        } else {
            Err(meta.error("unrecognized ptr_meta argument"))
        }
    }

    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut result = Self::default();

        for attr in attrs.iter() {
            if !matches!(attr.style, AttrStyle::Outer) {
                continue;
            }

            if attr.path().is_ident("ptr_meta") {
                attr.parse_nested_meta(|nested| result.parse_meta(nested))?;
            }
        }

        Ok(result)
    }

    pub fn crate_path(&self) -> Path {
        self.crate_path
            .clone()
            .unwrap_or_else(|| parse_quote! { ::ptr_meta })
    }
}
