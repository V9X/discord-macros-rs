pub mod meta_struct;
mod spanned_value;
pub use spanned_value::*;

/// Extracts value from an enum
macro_rules! variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(v) => Some(v),
            _ => None,
        }
    };
    ($value:expr, $variant:path{$($var:ident),+}) => {
        match $value {
            $variant{$($var,)+ ..} => Some(($($var),+)),
            _ => None,
        }
    };
}

pub(crate) use variant;
