use discord_macros_common::api_definitions::application_command::{
    Command, CommandType, InstallationContext, InteractionContext, Permissions,
};

use darling::{ast::Data, Error, FromDeriveInput, FromMeta, Result};
use proc_macro2::TokenTree;
use syn::Meta;

use crate::utils::variant;

use super::{
    fields::Field,
    types::{parse_bitflags, CommandGroups, Description, MixedName, Name, SubcommandPaths},
};

// TODO: localized names / descriptions
#[derive(Debug, Default)]
pub struct AttributesParser {
    pub command: Command,
    pub code_gen: Option<CodeGen>,
}

#[derive(Debug)]
pub enum CodeGen {
    Groups(CommandGroups),
    Subcommands(SubcommandPaths),
    Fields(Vec<Field>),
}

impl FromDeriveInput for AttributesParser {
    fn from_derive_input(input: &syn::DeriveInput) -> Result<Self> {
        let mut kind = CommandType::default();

        'ml: for ml in input
            .attrs
            .iter()
            .filter_map(|a| variant!(&a.meta, Meta::List).filter(|v| v.path.is_ident("cmd")))
        {
            let mut tokens = ml.tokens.clone().into_iter().peekable();
            while let Some(t) = tokens.next() {
                if matches!(t, TokenTree::Ident(v) if v == "kind")
                    && tokens
                        .next()
                        .is_some_and(|v| matches!(v, TokenTree::Punct(v) if v.as_char() == '='))
                {
                    if let Some(v) = tokens
                        .next()
                        .and_then(|v| {
                            variant!(v, TokenTree::Literal)
                                .map(|v| v.to_string())
                                .filter(|v| v.len() > 2)
                        })
                        .and_then(|v| CommandType::from_string(&v[1..v.len() - 1]).ok())
                    {
                        kind = v;
                        break 'ml;
                    }
                }
            }
        }

        Ok(match kind {
            CommandType::ChatInput => {
                let v: ChatInputParser = ChatInputParser::from_derive_input(input)?;
                let fields = v.data.take_struct().unwrap().fields;
                let mut command: Command = v.common.into();
                command.name = v.name.0;
                command.description = Some(v.description.0);

                Self {
                    command,
                    code_gen: v
                        .subcommands
                        .map(CodeGen::Subcommands)
                        .or(v.groups.map(CodeGen::Groups))
                        .or((!fields.is_empty()).then_some(CodeGen::Fields(fields))),
                }
            }
            CommandType::Message | CommandType::User => {
                let v = UserMessageParser::from_derive_input(input)?;
                let mut command: Command = v.common.into();
                command.name = v.name.0;

                Self {
                    command,
                    ..Default::default()
                }
            }
        })
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(cmd),
    supports(struct_named, struct_unit),
    and_then = "Self::validate"
)]
struct ChatInputParser {
    #[darling(flatten)]
    common: CommonAttributes,
    name: Name,
    description: Description,
    groups: Option<CommandGroups>,
    subcommands: Option<SubcommandPaths>,
    data: darling::ast::Data<(), Field>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(cmd), supports(struct_unit))]
struct UserMessageParser {
    #[darling(flatten)]
    common: CommonAttributes,
    name: MixedName,
}

#[derive(Debug, FromMeta)]
struct CommonAttributes {
    #[darling(default)]
    kind: CommandType,
    #[darling(default)]
    nsfw: bool,
    #[darling(default, and_then = "parse_bitflags")]
    permissions: Permissions,
    #[darling(default, and_then = "parse_bitflags")]
    installation: InstallationContext,
    #[darling(default, and_then = "parse_bitflags")]
    contexts: InteractionContext,
}

impl From<CommonAttributes> for Command {
    fn from(value: CommonAttributes) -> Self {
        Command {
            kind: value.kind,
            nsfw: value.nsfw,
            default_member_permissions: value.permissions,
            integration_types: value.installation,
            contexts: value.contexts,
            ..Default::default()
        }
    }
}

impl ChatInputParser {
    fn validate(self) -> Result<Self> {
        let mut acc = Error::accumulator();

        if self.groups.is_some() as u8
            + self.subcommands.is_some() as u8
            + variant!(&self.data, Data::Struct).is_some_and(|v| !v.is_empty()) as u8
            > 1
        {
            acc.push(Error::custom(
                "Command can contain only one of: `options`, `subcommands`, `groups`",
            ));
        }

        variant!(&self.data, Data::Struct).inspect(|v| {
            if v.len() > 25 {
                acc.push(Error::custom(format_args!(
                    "Expected no more than `25` options, got: `{}`",
                    v.len()
                )));
            }

            v.fields.iter().fold(false, |s, v| {
                if s && v.command_option.required {
                    acc.push(
                        Error::custom("Required fields must appear before optional fields")
                            .with_span(&v.ident),
                    )
                };

                s || !v.command_option.required
            });
        });

        acc.finish()?;

        Ok(self)
    }
}
