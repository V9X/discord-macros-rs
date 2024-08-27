use darling::FromMeta;
use proc_macro2::Span;
use syn::{spanned::Spanned, Meta};

/// Same as `darling::util::SpannedValue`, but allows to take ownership of T (pub fields)
#[derive(Debug)]
pub struct SpannedValue<T> {
    pub value: T,
    pub span: Span,
}

impl<T: Default> Default for SpannedValue<T> {
    fn default() -> Self {
        SpannedValue {
            value: T::default(),
            span: Span::call_site(),
        }
    }
}

impl<T: FromMeta> FromMeta for SpannedValue<T> {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        let value = T::from_meta(item).map_err(|e| e.with_span(item))?;

        let span = match item {
            Meta::Path(v) => v.span(),
            Meta::List(v) => v.tokens.span(),
            Meta::NameValue(v) => v.value.span(),
        };

        Ok(Self { value, span })
    }
}
