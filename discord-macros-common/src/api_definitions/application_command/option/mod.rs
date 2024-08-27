use ahash::HashMap;
use serde::Serialize;
use serde_json::{Number, Value};
mod channel_types;
mod option_type;
pub use channel_types::ChannelTypes;
pub use option_type::OptionType;

#[derive(Debug, Serialize)]
pub struct CommandOptionChoices {
    pub name: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub name_localized: HashMap<String, String>,
    pub value: Value,
}

/// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure)
#[derive(Debug, Serialize, Default)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub kind: OptionType,
    pub name: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub name_localized: HashMap<String, String>,
    pub description: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub description_localized: HashMap<String, String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub required: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<CommandOptionChoices>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,
    #[serde(skip_serializing_if = "ChannelTypes::is_empty")]
    pub channel_types: ChannelTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub autocomplete: bool,
}
