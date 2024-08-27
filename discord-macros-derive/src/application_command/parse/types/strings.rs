use super::utils::validate_string;
use syn::spanned::Spanned;

macro_rules! wrapper {
    ($name:ident, $len:literal, $mixed:literal, $whitespace:literal) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub struct $name(pub String);
        impl darling::FromMeta for $name {
            fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
                let string = String::from_meta(item)?;
                validate_string(&string, &item.span(), $len, $mixed, $whitespace)?;

                Ok(Self(string))
            }
        }
        impl $name {
            pub fn from_path(path: &syn::Path) -> darling::Result<Self> {
                let string = path.require_ident()?.to_string();
                validate_string(&string, &path.span(), $len, $mixed, $whitespace)?;

                Ok(Self(string))
            }
        }
    };
}

wrapper!(Description, 100, true, true);
wrapper!(Name, 32, false, false);
wrapper!(MixedName, 32, true, false);
wrapper!(ChoiceName, 100, false, false);
