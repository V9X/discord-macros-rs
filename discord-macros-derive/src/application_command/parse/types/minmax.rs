use darling::{Error, FromMeta, Result};

use crate::utils::{meta_struct::meta_struct, SpannedValue};

#[derive(Debug)]
pub struct MinMax<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T: FromMeta + PartialOrd> FromMeta for MinMax<T> {
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        meta_struct! {
            struct MinMaxParser<T: PartialOrd> {
                min: Option<SpannedValue<T>>,
                max: Option<SpannedValue<T>>,
            }
        };

        let min_max: MinMaxParser<T> = MinMaxParser::from_meta(item)?;

        if let Some((min, max)) = min_max.min.as_ref().zip(min_max.max.as_ref()) {
            if min.value > max.value {
                Err(Error::custom("`min` cannot be greater than `max`"))?;
            }
        }

        Ok(MinMax {
            min: min_max.min.map(|v| v.value),
            max: min_max.max.map(|v| v.value),
        })
    }
}
