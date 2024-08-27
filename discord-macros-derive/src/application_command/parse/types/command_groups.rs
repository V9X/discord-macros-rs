use ahash::AHashSet;
use darling::{ast::NestedMeta, Error, FromMeta, Result};
use syn::Meta;

use crate::utils::meta_struct::meta_struct;

use super::{Description, Name, SubcommandPaths};

#[derive(Debug)]
pub struct GroupData {
    name: Name,
    description: Description,
    subcommands: SubcommandPaths,
}

#[derive(Debug)]
pub struct CommandGroups(pub Vec<GroupData>);

impl FromMeta for CommandGroups {
    fn from_meta(item: &Meta) -> Result<Self> {
        meta_struct! {
            struct GroupDataParser {
                description: Description,
                subcommands: SubcommandPaths,
            }
        }

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

            let name = acc.handle(Name::from_path(meta.path()));
            let data = acc.handle(GroupDataParser::from_meta(&meta));

            let Some((name, data)) = name.zip(data) else {
                continue;
            };

            value.push(GroupData {
                name,
                description: data.description,
                subcommands: data.subcommands,
            });

            let name = &unsafe { &*value.as_ptr().add(value.len() - 1) }.name;

            if !names.insert(name) {
                acc.push(
                    Error::custom(format_args!("Duplicate group name: `{}`", name.0))
                        .with_span(meta.path()),
                )
            };
        }

        if value.len() > 25 {
            acc.push(Error::custom(format_args!(
                "Expected no more than `25` groups, got: `{}`",
                value.len()
            )))
        }

        acc.finish()?;

        Ok(Self(value))
    }
}
