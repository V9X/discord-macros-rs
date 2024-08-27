use serde::Serialize;

/// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type)
#[derive(Debug, Clone, Copy, Serialize, Default)]
#[serde(into = "u8")]
pub enum OptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    #[default]
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

impl From<OptionType> for u8 {
    fn from(value: OptionType) -> Self {
        value as u8
    }
}
