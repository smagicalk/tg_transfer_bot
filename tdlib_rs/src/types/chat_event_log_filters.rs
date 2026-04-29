#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a set of filters used to obtain a chat event log
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLogFilters {
    /// True, if message edits need to be returned
    pub message_edits: bool,
    /// True, if message deletions need to be returned
    pub message_deletions: bool,
    /// True, if pin/unpin events need to be returned
    pub message_pins: bool,
    /// True, if members joining events need to be returned
    pub member_joins: bool,
    /// True, if members leaving events need to be returned
    pub member_leaves: bool,
    /// True, if invited member events need to be returned
    pub member_invites: bool,
    /// True, if member promotion/demotion events need to be returned
    pub member_promotions: bool,
    /// True, if member restricted/unrestricted/banned/unbanned events need to be returned
    pub member_restrictions: bool,
    /// True, if member tag and custom title change events need to be returned
    pub member_tag_changes: bool,
    /// True, if changes in chat information need to be returned
    pub info_changes: bool,
    /// True, if changes in chat settings need to be returned
    pub setting_changes: bool,
    /// True, if changes to invite links need to be returned
    pub invite_link_changes: bool,
    /// True, if video chat actions need to be returned
    pub video_chat_changes: bool,
    /// True, if forum-related actions need to be returned
    pub forum_changes: bool,
    /// True, if subscription extensions need to be returned
    pub subscription_extensions: bool,
}
