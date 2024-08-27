use application_command::expand_command_derive;
use syn::{parse_macro_input, DeriveInput, Error};

mod application_command;
mod extensions;
mod utils;

#[proc_macro_derive(ApplicationCommand, attributes(cmd))]
pub fn command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_command_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
