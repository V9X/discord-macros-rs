use darling::{Error, FromMeta, Result};

use crate::utils::SpannedValue;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumLimiter<const L: u64, const M: u64, T: PartialOrd>(pub T);

impl<const L: u64, const M: u64, T: FromMeta + PartialOrd + Clone + Into<u64>> FromMeta
    for NumLimiter<L, M, T>
{
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        let num = SpannedValue::<T>::from_meta(item)?;
        let num_val = num.value.clone().into();

        if num_val < L {
            Err(
                Error::custom(format_args!("value cannot be lower than {L}")).with_span(&num.span),
            )?;
        }

        if M != 0 && num_val > M {
            Err(
                Error::custom(format_args!("value cannot be greater than {M}"))
                    .with_span(&num.span),
            )?;
        }

        Ok(Self(num.value))
    }
}
