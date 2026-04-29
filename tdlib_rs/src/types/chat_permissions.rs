#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes actions that a user is allowed to take in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPermissions {
    /// True, if the user can send text messages, contacts, giveaways, giveaway winners, invoices, locations, and venues
    pub can_send_basic_messages: bool,
    /// True, if the user can send music files
    pub can_send_audios: bool,
    /// True, if the user can send documents
    pub can_send_documents: bool,
    /// True, if the user can send photos
    pub can_send_photos: bool,
    /// True, if the user can send videos
    pub can_send_videos: bool,
    /// True, if the user can send video notes
    pub can_send_video_notes: bool,
    /// True, if the user can send voice notes
    pub can_send_voice_notes: bool,
    /// True, if the user can send polls and checklists
    pub can_send_polls: bool,
    /// True, if the user can send animations, games, stickers, and dice and use inline bots
    pub can_send_other_messages: bool,
    /// True, if the user may add a link preview to their messages
    pub can_add_link_previews: bool,
    /// True, if the user may change the tag of self
    pub can_edit_tag: bool,
    /// True, if the user can change the chat title, photo, and other settings
    pub can_change_info: bool,
    /// True, if the user can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user can pin messages
    pub can_pin_messages: bool,
    /// True, if the user can create topics
    pub can_create_topics: bool,
}
