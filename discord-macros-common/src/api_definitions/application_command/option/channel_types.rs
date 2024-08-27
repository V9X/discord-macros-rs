use bitflags::bitflags;
use serde::Serialize;

/// [Discord docs](https://discord.com/developers/docs/topics/permissions)
#[derive(Debug, Default)]
pub struct ChannelTypes(u64);

bitflags! {
    impl ChannelTypes: u64 {
        /// a text channel within a server
        const GUILD_TEXT = 1 << 0;
        /// a direct message between users
        const DM = 1 << 1;
        /// a voice channel within a server
        const GUILD_VOICE = 1 << 2;
        /// a direct message between multiple users
        const GROUP_DM = 1 << 3;
        /// an organizational category that contains up to 50 channels
        const GUILD_CATEGORY = 1 << 4;
        /// a channel that users can follow and crosspost into their own server (formerly news channels)
        const GUILD_ANNOUNCEMENT = 1 << 5;
        /// a temporary sub-channel within a GUILD_ANNOUNCEMENT channel
        const ANNOUNCEMENT_THREAD = 1 << 10;
        /// a temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel
        const PUBLIC_THREAD = 1 << 11;
        /// a temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
        const PRIVATE_THREAD = 1 << 12;
        /// a voice channel for hosting events with an audience
        const GUILD_STAGE_VOICE = 1 << 13;
        /// the channel in a hub containing the listed servers
        const GUILD_DIRECTORY = 1 << 14;
        /// Channel that can only contain threads
        const GUILD_FORUM = 1 << 15;
        /// Channel that can only contain threads, similar to GUILD_FORUM channels
        const GUILD_MEDIA = 1 << 16;
    }
}

impl Serialize for ChannelTypes {
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
