use super::SpannedValue;
use crate::extensions::darling_err::AccumulatorExt;
use darling::{error::Accumulator, Error, FromMeta, Result};
use proc_macro2::Span;
use syn::{spanned::Spanned, Meta};

// The idea behind this macro is to introduce better error handling for nested meta arguments
macro_rules! meta_struct {
    (
        $vis:vis struct $name:ident
        $(<$($gv:tt $(: $fgv:tt $(+ $sgv:tt)*)?),+>)?
        {$($field:ident: $value:ty $(=> $validator:expr)?,)+}
    ) => {
        $vis struct $name $(<$($gv),+>)? {
            $($field: $value),+
        }

        impl $(<$($gv: FromMeta $(+ $fgv $(+ $sgv)*)?),+>)? ::darling::FromMeta for $name $(<$($gv),+>)? {
            fn from_meta(item: &::syn::Meta) -> ::darling::Result<Self> {
                let meta_list = item.require_list()?;

                let nested_meta_list =
                    ::darling::ast::NestedMeta::parse_meta_list(meta_list.tokens.clone())?;

                let mut acc = Error::accumulator();

                $(
                    let mut $field: $crate::utils::meta_struct::FieldExtractor<$value> =
                        $crate::utils::meta_struct::FieldExtractor::new(stringify!($field), ::syn::spanned::Spanned::span(item));
                )+

                // TODO: autocorrection for names

                for nested_meta in nested_meta_list {
                    let meta = match nested_meta {
                        ::darling::ast::NestedMeta::Lit(l) => {
                            acc.push(<::darling::Error as crate::extensions::darling_err::DarlingErrorExt>::unsupported_lit_format(&l).with_span(&l));
                            continue;
                        }
                        ::darling::ast::NestedMeta::Meta(m) => m,
                    };

                    let Some(ident) = acc.handle(meta.path().require_ident().map_err(Into::into)) else {
                        continue;
                    };

                    match ident.to_string().as_str() {
                        $(
                            stringify!($field) => $field.push(&meta),
                        )+
                        unknown => {
                            acc.push(Error::unknown_field(unknown).with_span(&::syn::spanned::Spanned::span(&meta)));
                            continue;
                        }
                    }
                }

                $(
                    $($field.validate($validator);)?
                    let $field = acc.handle($field.into());
                )+

                if true $(&& $field.is_some())+ {
                    return acc.finish_with($name {
                        $($field: $field.unwrap(),)+
                    });
                }

                Err(acc.finish().unwrap_err())
            }
        }
    };
}

pub(crate) use meta_struct;

pub struct FieldExtractor<'a, T: FromMeta> {
    name: &'a str,
    acc: Accumulator,
    outer_span: Span,
    value: Option<Result<SpannedValue<T>>>,
}

impl<'a, T: FromMeta> FieldExtractor<'a, T> {
    pub fn new(name: &'a str, outer_span: Span) -> Self {
        Self {
            acc: Error::accumulator(),
            name,
            outer_span,
            value: None,
        }
    }

    pub fn push(&mut self, item: &Meta) {
        if self.value.is_some() {
            self.acc
                .push(Error::duplicate_field(self.name).with_span(&item.span()));
            return;
        }

        self.value = Some(SpannedValue::<T>::from_meta(item));
    }

    pub fn validate<F>(&mut self, f: F)
    where
        F: FnOnce(&T) -> Result<()>,
    {
        if let Some(Ok(v)) = &self.value {
            self.acc
                .handle(f(&v.value).map_err(|e| e.with_span(&v.span)));
        }
    }
}

impl<'a, T: FromMeta> From<FieldExtractor<'a, T>> for Result<Option<T>> {
    fn from(value: FieldExtractor<T>) -> Self {
        value
            .acc
            .finish_with_result(value.value.map(|v| v.map(|v| v.value)).transpose())
    }
}

impl<'a, T: FromMeta> From<FieldExtractor<'a, T>> for Result<T> {
    fn from(value: FieldExtractor<T>) -> Self {
        let Some(v) = value.value else {
            return value
                .acc
                .finish_with_err(Error::missing_field(value.name).with_span(&value.outer_span));
        };

        value.acc.finish_with_result(v.map(|v| v.value))
    }
}
