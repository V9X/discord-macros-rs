use bitflags::bitflags;
use serde::Serialize;

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-context-types)
#[derive(Debug, Default, PartialEq)]
pub struct InteractionContext(u8);

bitflags! {
    impl InteractionContext: u8 {
        /// Interaction can be used within servers
        const GUILD = 1 << 0;
        /// Interaction can be used within DMs with the app's bot user
        const BOT_DM = 1 << 1;
        /// Interaction can be used within Group DMs and DMs other than the app's bot user
        const PRIVATE_CHANNEL = 1 << 2;
    }
}

impl Serialize for InteractionContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags: Vec<u8> = Vec::new();

        for flag in self.iter() {
            flags.push(flag.0.trailing_zeros() as _);
        }

        flags.serialize(serializer)
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/application#application-object-application-integration-types)
#[derive(Debug, Default, PartialEq)]
pub struct InstallationContext(u8);

bitflags! {
    impl InstallationContext: u8 {
        /// App is installable to servers
        const GUILD_INSTALL = 1 << 0;
        /// App is installable to users
        const USER_INSTALL = 1 << 1;
    }
}

impl Serialize for InstallationContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags: Vec<u8> = Vec::new();

        for flag in self.iter() {
            flags.push(flag.0.trailing_zeros() as _);
        }

        flags.serialize(serializer)
    }
}
