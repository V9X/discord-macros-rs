use bitflags::bitflags;
use serde::Serialize;

/// [Discord docs](https://discord.com/developers/docs/topics/permissions)
#[derive(Debug, Default)]
pub struct Permissions(u64);

bitflags! {
    impl Permissions: u64 {
        /// Allows creation of instant invites
        const CREATE_INSTANT_INVITE = 1 << 0;
        /// Allows kicking members
        const KICK_MEMBERS = 1 << 1;
        /// Allows banning members
        const BAN_MEMBERS = 1 << 2;
        /// Allows all permissions and bypasses channel permission overwrites
        const ADMINISTRATOR = 1 << 3;
        /// Allows management and editing of channels
        const MANAGE_CHANNELS = 1 << 4;
        /// Allows management and editing of the guild
        const MANAGE_GUILD = 1 << 5;
        /// Allows for the addition of reactions to messages
        const ADD_REACTIONS = 1 << 6;
        /// Allows for viewing of audit logs
        const VIEW_AUDIT_LOG = 1 << 7;
        /// Allows for using priority speaker in a voice channel
        const PRIORITY_SPEAKER = 1 << 8;
        /// Allows the user to go live
        const STREAM = 1 << 9;
        /// Allows guild members to view a channel, which includes reading messages in text channels and joining voice channels
        const VIEW_CHANNEL = 1 << 10;
        /// Allows for sending messages in a channel and creating threads in a forum (does not allow sending messages in threads)
        const SEND_MESSAGES = 1 << 11;
        /// Allows for sending of <code>/tts</code> messages
        const SEND_TTS_MESSAGES = 1 << 12;
        /// Allows for deletion of other users messages
        const MANAGE_MESSAGES = 1 << 13;
        /// Links sent by users with this permission will be auto-embedded
        const EMBED_LINKS = 1 << 14;
        /// Allows for uploading images and files
        const ATTACH_FILES = 1 << 15;
        /// Allows for reading of message history
        const READ_MESSAGE_HISTORY = 1 << 16;
        /// Allows for using the <code>@everyone</code> tag to notify all users in a channel,
        /// and the <code>@here</code> tag to notify all online users in a channel
        const MENTION_EVERYONE = 1 << 17;
        /// Allows the usage of custom emojis from other servers
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        /// Allows for viewing guild insights
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        /// Allows for joining of a voice channel
        const CONNECT = 1 << 20;
        /// Allows for speaking in a voice channel
        const SPEAK = 1 << 21;
        /// Allows for muting members in a voice channel
        const MUTE_MEMBERS = 1 << 22;
        /// Allows for deafening of members in a voice channel
        const DEAFEN_MEMBERS = 1 << 23;
        /// Allows for moving of members between voice channels
        const MOVE_MEMBERS = 1 << 24;
        /// Allows for using voice-activity-detection in a voice channel
        const USE_VAD = 1 << 25;
        /// Allows for modification of own nickname
        const CHANGE_NICKNAME = 1 << 26;
        /// Allows for modification of other users nicknames
        const MANAGE_NICKNAMES = 1 << 27;
        /// Allows management and editing of roles
        const MANAGE_ROLES = 1 << 28;
        /// Allows management and editing of webhooks
        const MANAGE_WEBHOOKS = 1 << 29;
        /// Allows for editing and deleting emojis, stickers, and soundboard sounds created by all users
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        /// Allows members to use application commands, including slash commands and context menu commands.
        const USE_APPLICATION_COMMANDS = 1 << 31;
        /// Allows for requesting to speak in stage channels.
        /// (This permission is under active development and may be changed or removed.)
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows for editing and deleting scheduled events created by all users
        const MANAGE_EVENTS = 1 << 33;
        /// Allows for deleting and archiving threads, and viewing all private threads
        const MANAGE_THREADS = 1 << 34;
        /// Allows for creating public and announcement threads
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows for creating private threads
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows the usage of custom stickers from other servers
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows for sending messages in threads
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows for using Activities (applications with the <code>EMBEDDED</code> flag) in a voice channel
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows for timing out users to prevent them from sending or reacting to messages in chat and threads,
        /// and from speaking in voice and stage channels
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows for viewing role subscription insights
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows for using soundboard in a voice channel
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows for creating emojis, stickers, and soundboard sounds, and editing and deleting those created by the current user.
        /// Not yet available to developers.
        const CREATE_GUILD_EXPRESSIONS = 1 << 43;
        /// Allows for creating scheduled events, and editing and deleting those created by the current user.
        /// Not yet available to developers.
        const CREATE_EVENTS = 1 << 44;
        /// Allows the usage of custom soundboard sounds from other servers
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages
        const SEND_VOICE_MESSAGES = 1 << 46;
        /// Allows sending polls
        const SEND_POLLS = 1 << 49;
        /// Allows user-installed apps to send public responses.
        /// When disabled, users will still be allowed to use their apps but the responses will be ephemeral.
        /// This only applies to apps not also installed to the server.
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}
