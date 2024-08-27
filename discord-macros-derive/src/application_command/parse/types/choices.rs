use ahash::AHashSet;
use darling::{ast::NestedMeta, Error, FromMeta, Result};

use crate::application_command::parse::types::ChoiceName;

#[derive(Debug)]
pub struct Choices<T>(pub Vec<(String, T)>);

impl<T: FromMeta> FromMeta for Choices<T> {
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        let list: &syn::MetaList = item.require_list()?;
        let nested_meta_list = NestedMeta::parse_meta_list(list.tokens.clone())?;

        let mut acc = Error::accumulator();
        let mut value = Vec::with_capacity(nested_meta_list.len());
        let mut names = AHashSet::with_capacity(nested_meta_list.len());

        for nested_meta in nested_meta_list {
            let meta = match nested_meta {
                NestedMeta::Lit(v) => {
                    acc.push(Error::unsupported_format("Literal").with_span(&v));
                    continue;
                }
                NestedMeta::Meta(v) => v,
            };

            let name = acc.handle(ChoiceName::from_path(meta.path()));
            let data = acc.handle(T::from_meta(&meta));

            let Some((name, data)) = name.zip(data) else {
                continue;
            };

            value.push((name.0, data));

            let name = &unsafe { &*value.as_ptr().add(value.len() - 1) }.0;

            if !names.insert(name) {
                acc.push(
                    Error::custom(format_args!("Duplicate choice name: `{}`", name))
                        .with_span(meta.path()),
                )
            };
        }

        if value.len() > 25 {
            acc.push(Error::custom(format_args!(
                "Expected no more than `25` choices, got: `{}`",
                value.len()
            )))
        }

        acc.finish_with(Self(value))
    }
}
