use ahash::AHashSet;
use darling::{util::path_to_string, Error, FromMeta, Result};
use syn::{spanned::Spanned, Expr, Meta, Path};

use crate::extensions::darling_err::{DarlingErrorExt, ExprType};

#[derive(Debug)]
pub struct SubcommandPaths(pub Vec<Path>);

impl FromMeta for SubcommandPaths {
    fn from_meta(item: &Meta) -> Result<Self> {
        const NAME: &str = "subcommands";
        const LIMIT_SIZE: usize = 25;

        let name_value = item.require_name_value()?;
        let Expr::Array(expr) = &name_value.value else {
            return Err(Error::unexpected_expr_w(&name_value.value, ExprType::Array)
                .with_span(&name_value.value.span()));
        };

        if expr.elems.is_empty() {
            return Err(Error::not_empty().with_span(&expr));
        }

        let mut acc = Error::accumulator();

        if expr.elems.len() > LIMIT_SIZE {
            acc.push(Error::length_limit(NAME, expr.elems.len(), LIMIT_SIZE).with_span(&expr));
        }

        let mut value = Vec::with_capacity(expr.elems.len());
        let mut paths = AHashSet::with_capacity(expr.elems.len());

        for expr in &expr.elems {
            if let Some(path) = acc.handle(
                Path::from_expr(expr)
                    .map_err(|_| Error::unexpected_expr_w(expr, ExprType::Path).with_span(expr)),
            ) {
                value.push(path);
                let path = unsafe { &*value.as_ptr().add(value.len() - 1) };

                if !paths.insert(path) {
                    acc.push(
                        Error::custom(format_args!(
                            "Duplicate subcommand `{}`",
                            path_to_string(path)
                        ))
                        .with_span(expr),
                    )
                };
            }
        }

        if value.len() > 25 {
            acc.push(Error::custom(format_args!(
                "Expected no more than `25` subcommands, got: `{}`",
                value.len()
            )))
        }

        acc.finish()?;

        Ok(Self(value))
    }
}
