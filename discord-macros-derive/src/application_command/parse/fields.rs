use std::fmt::Debug;

use crate::{
    application_command::parse::types::validate_string,
    utils::{variant, SpannedValue},
};

use super::types::{parse_bitflags_spn, Choices, Description, MinMax, Name, NumLimiter};

use const_format::{concatcp, formatcp};
use darling::{Error, FromField, FromMeta, Result};
use discord_macros_common::api_definitions::application_command::{
    ChannelTypes, CommandOption, CommandOptionChoices, OptionType,
};

use serde_json::Value;

#[derive(Debug)]
pub struct Field {
    pub command_option: CommandOption,
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

impl FromField for Field {
    // Type validation here is not complete, there is still a possibility that the parsed type is different from the one we expect,
    // in order to be absolutely sure, we need to generate code that will throw a compilation error if the type does not match.
    fn from_field(field: &syn::Field) -> Result<Self> {
        const OPTION_DISPLAY: &str = "Option<_>";
        const OPTION: &str = "Option";
        const STRING: &str = "String";
        const U64: &str = "u64";
        const F64: &str = "f64";
        const BOOL: &str = "bool";
        const NON_ZERO_U64: &str = "NonZeroU64";

        const ALLOWED_TYPES_WO_OPT: &str =
            formatcp!("`{STRING}`, `{U64}`, `{F64}`, `{BOOL}`, `{NON_ZERO_U64}`");
        const ALLOWED_TYPES: &str = formatcp!("`{OPTION_DISPLAY}`, {ALLOWED_TYPES_WO_OPT}`");
        const UNSUPPORTED_MESSAGE: &str = "Unsupported value, expected one of: ";

        const UNSUPPORTED_OPT_VALUE_ERROR: &str = concatcp!(UNSUPPORTED_MESSAGE, ALLOWED_TYPES);
        const UNSUPPORTED_VALUE_ERROR: &str = concatcp!(UNSUPPORTED_MESSAGE, ALLOWED_TYPES_WO_OPT);

        let ident = field
            .ident
            .as_ref()
            .expect("Field used in struct newtype/tuple`");

        let syn::Type::Path(type_path) = &field.ty else {
            return Err(Error::custom(UNSUPPORTED_OPT_VALUE_ERROR).with_span(&field.ty));
        };

        let mut path_segment = type_path.path.segments.last().unwrap();
        let mut required = true;

        if path_segment.ident == OPTION {
            let syn::PathArguments::AngleBracketed(b) = &path_segment.arguments else {
                return Err(Error::custom(UNSUPPORTED_OPT_VALUE_ERROR).with_span(&field.ty));
            };

            if b.args.len() != 1 {
                return Err(Error::custom(UNSUPPORTED_OPT_VALUE_ERROR).with_span(&field.ty));
            }

            let syn::GenericArgument::Type(t) = &b.args[0] else {
                return Err(Error::custom(UNSUPPORTED_OPT_VALUE_ERROR).with_span(&field.ty));
            };

            let syn::Type::Path(type_path) = t else {
                return Err(Error::custom(UNSUPPORTED_VALUE_ERROR).with_span(t));
            };

            path_segment = type_path.path.segments.last().unwrap();
            required = false;
        }

        let mut command_option: CommandOption = match path_segment.ident.to_string().as_str() {
            STRING => FieldString::from_field(field)?.into(),
            U64 => FieldNumber::<u64>::from_field(field)?.into(),
            F64 => FieldNumber::<u64>::from_field(field)?.into(),
            NON_ZERO_U64 => FieldId::from_field(field)?.into(),
            BOOL => FieldBool::from_field(field)?.into(),
            _ => return Err(Error::custom(UNSUPPORTED_VALUE_ERROR).with_span(path_segment)),
        };

        command_option.required = required;
        if command_option.name.is_empty() {
            let ident_string = ident.to_string();
            command_option.name = validate_string(&ident_string, &ident.span(), 32, false, false)
                .map(|_| ident_string)?;
        }

        Ok(Self {
            command_option,
            ident: ident.clone(),
            ty: field.ty.clone(),
        })
    }
}

trait SerdeJsonValue {
    fn to_value(self) -> Value;
}

macro_rules! impl_serde_json_value {
    ($type:ty) => {
        impl SerdeJsonValue for $type {
            fn to_value(self) -> Value {
                Value::from(self)
            }
        }
    };
}

impl_serde_json_value!(u64);
impl_serde_json_value!(f64);
impl_serde_json_value!(String);

#[derive(Debug, FromMeta)]
struct FieldSuperCommon {
    rename: Option<Name>,
    description: Description,
}

impl From<FieldSuperCommon> for CommandOption {
    fn from(value: FieldSuperCommon) -> Self {
        CommandOption {
            name: value.rename.map(|v| v.0).unwrap_or_default(),
            description: value.description.0,
            ..Default::default()
        }
    }
}

#[derive(Debug, FromMeta)]
#[darling(and_then = "Self::validate")]
struct FieldCommon<T: SerdeJsonValue> {
    #[darling(flatten)]
    common: FieldSuperCommon,
    choices: Option<Choices<T>>,
    #[darling(default)]
    autocomplete: SpannedValue<bool>,
}

impl<T: SerdeJsonValue> FieldCommon<T> {
    fn validate(self) -> Result<Self> {
        if self.choices.is_some() && self.autocomplete.value {
            Err(
                Error::custom("No need for `autocomplete` when `choices` are present")
                    .with_span(&self.autocomplete.span),
            )?;
        }

        Ok(self)
    }
}

impl<T: SerdeJsonValue> From<FieldCommon<T>> for CommandOption {
    fn from(value: FieldCommon<T>) -> Self {
        let mut cmd_opt: CommandOption = value.common.into();

        cmd_opt.choices = value
            .choices
            .map(|v| {
                v.0.into_iter()
                    .map(|v| CommandOptionChoices {
                        name: v.0,
                        name_localized: Default::default(),
                        value: v.1.to_value(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        cmd_opt.autocomplete = value.autocomplete.value;

        cmd_opt
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(cmd))]
struct FieldNumber<T: SerdeJsonValue + PartialOrd> {
    #[darling(flatten)]
    common: FieldCommon<T>,
    value: Option<MinMax<T>>,
}

impl<T: SerdeJsonValue + PartialOrd> From<FieldNumber<T>> for CommandOption {
    fn from(value: FieldNumber<T>) -> Self {
        let mut cmd_opt: CommandOption = value.common.into();
        cmd_opt.kind = OptionType::Number;
        if let Some(value) = value.value {
            cmd_opt.min_value = value
                .min
                .and_then(|v| variant!(v.to_value(), Value::Number));
            cmd_opt.max_value = value
                .max
                .and_then(|v| variant!(v.to_value(), Value::Number));
        }

        cmd_opt
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(cmd))]
struct FieldString {
    #[darling(flatten)]
    common: FieldCommon<String>,
    length: Option<MinMax<NumLimiter<1, 6000, u16>>>,
}

impl From<FieldString> for CommandOption {
    fn from(value: FieldString) -> Self {
        let mut cmd_opt: CommandOption = value.common.into();
        cmd_opt.kind = OptionType::String;
        if let Some(length) = value.length {
            cmd_opt.min_length = length.min.map(|v| v.0);
            cmd_opt.max_length = length.max.map(|v| v.0);
        }

        cmd_opt
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(cmd), and_then = "Self::validate")]
struct FieldId {
    #[darling(flatten)]
    common: FieldSuperCommon,
    kind: OptionIdKind,
    #[darling(default, and_then = "parse_bitflags_spn")]
    channel_types: SpannedValue<ChannelTypes>,
}

impl FieldId {
    fn validate(self) -> Result<Self> {
        if !matches!(self.kind, OptionIdKind::Channel) && !self.channel_types.value.is_empty() {
            Err(
                Error::custom("`channel_types` can only be used when the `kind` is `channel`")
                    .with_span(&self.channel_types.span),
            )?;
        }

        Ok(self)
    }
}

impl From<FieldId> for CommandOption {
    fn from(value: FieldId) -> Self {
        let mut cmd_opt: CommandOption = value.common.into();
        cmd_opt.kind = value.kind.into();
        cmd_opt.channel_types = value.channel_types.value;

        cmd_opt
    }
}

#[derive(Debug, FromMeta)]
enum OptionIdKind {
    Attachment,
    Channel,
    Mentionable,
    Role,
    User,
}

impl From<OptionIdKind> for OptionType {
    fn from(value: OptionIdKind) -> Self {
        match value {
            OptionIdKind::Attachment => Self::Attachment,
            OptionIdKind::Channel => Self::Channel,
            OptionIdKind::Mentionable => Self::Mentionable,
            OptionIdKind::Role => Self::Role,
            OptionIdKind::User => Self::User,
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(cmd))]
struct FieldBool {
    #[darling(flatten)]
    common: FieldSuperCommon,
}

impl From<FieldBool> for CommandOption {
    fn from(value: FieldBool) -> Self {
        let mut cmd_opt: CommandOption = value.common.into();
        cmd_opt.kind = OptionType::Boolean;

        cmd_opt
    }
}
