use ahash::HashMap;
use serde::Serialize;

mod contexts;
mod option;
mod permissions;
pub use contexts::*;
pub use option::*;
pub use permissions::*;

/// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types)
#[derive(Debug, Default, Clone, Copy, Serialize)]
#[cfg_attr(feature = "derive", derive(darling::FromMeta))]
#[serde(into = "u8")]
pub enum CommandType {
    #[default]
    ChatInput = 1,
    User = 2,
    Message = 3,
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        value as u8
    }
}

impl CommandType {
    pub fn is_default(&self) -> bool {
        matches!(&self, Self::ChatInput)
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#create-global-application-command-json-params)
#[derive(Debug, Serialize, Default)]
pub struct Command {
    pub name: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub name_localizations: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub description_localizations: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,
    #[serde(skip_serializing_if = "Permissions::is_empty")]
    pub default_member_permissions: Permissions,
    #[serde(skip_serializing_if = "InstallationContext::is_empty")]
    pub integration_types: InstallationContext,
    #[serde(skip_serializing_if = "InteractionContext::is_empty")]
    pub contexts: InteractionContext,
    #[serde(rename = "type", skip_serializing_if = "CommandType::is_default")]
    pub kind: CommandType,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub nsfw: bool,
}
