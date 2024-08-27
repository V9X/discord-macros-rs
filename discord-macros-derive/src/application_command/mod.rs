use darling::FromDeriveInput;
use parse::global_attributes::AttributesParser;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

mod parse;

pub fn expand_command_derive(input: DeriveInput) -> Result<TokenStream> {
    let input = AttributesParser::from_derive_input(&input)?;
    // TODO: expand

    Ok(quote! {})
}
