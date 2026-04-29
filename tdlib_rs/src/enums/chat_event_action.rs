#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEventAction {
    /// A message was edited
    #[serde(rename(serialize = "chatEventMessageEdited", deserialize = "chatEventMessageEdited"))]
    ChatEventMessageEdited(crate::types::ChatEventMessageEdited),
    /// A message was deleted
    #[serde(rename(serialize = "chatEventMessageDeleted", deserialize = "chatEventMessageDeleted"))]
    ChatEventMessageDeleted(crate::types::ChatEventMessageDeleted),
    /// A message was pinned
    #[serde(rename(serialize = "chatEventMessagePinned", deserialize = "chatEventMessagePinned"))]
    ChatEventMessagePinned(crate::types::ChatEventMessagePinned),
    /// A message was unpinned
    #[serde(rename(serialize = "chatEventMessageUnpinned", deserialize = "chatEventMessageUnpinned"))]
    ChatEventMessageUnpinned(crate::types::ChatEventMessageUnpinned),
    /// A poll in a message was stopped
    #[serde(rename(serialize = "chatEventPollStopped", deserialize = "chatEventPollStopped"))]
    ChatEventPollStopped(crate::types::ChatEventPollStopped),
    /// A new member joined the chat
    #[serde(rename(serialize = "chatEventMemberJoined", deserialize = "chatEventMemberJoined"))]
    ChatEventMemberJoined,
    /// A new member joined the chat via an invite link
    #[serde(rename(serialize = "chatEventMemberJoinedByInviteLink", deserialize = "chatEventMemberJoinedByInviteLink"))]
    ChatEventMemberJoinedByInviteLink(crate::types::ChatEventMemberJoinedByInviteLink),
    /// A new member was accepted to the chat by an administrator
    #[serde(rename(serialize = "chatEventMemberJoinedByRequest", deserialize = "chatEventMemberJoinedByRequest"))]
    ChatEventMemberJoinedByRequest(crate::types::ChatEventMemberJoinedByRequest),
    /// A new chat member was invited
    #[serde(rename(serialize = "chatEventMemberInvited", deserialize = "chatEventMemberInvited"))]
    ChatEventMemberInvited(crate::types::ChatEventMemberInvited),
    /// A member left the chat
    #[serde(rename(serialize = "chatEventMemberLeft", deserialize = "chatEventMemberLeft"))]
    ChatEventMemberLeft,
    /// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
    #[serde(rename(serialize = "chatEventMemberPromoted", deserialize = "chatEventMemberPromoted"))]
    ChatEventMemberPromoted(crate::types::ChatEventMemberPromoted),
    /// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
    #[serde(rename(serialize = "chatEventMemberRestricted", deserialize = "chatEventMemberRestricted"))]
    ChatEventMemberRestricted(crate::types::ChatEventMemberRestricted),
    /// A chat member tag has been changed
    #[serde(rename(serialize = "chatEventMemberTagChanged", deserialize = "chatEventMemberTagChanged"))]
    ChatEventMemberTagChanged(crate::types::ChatEventMemberTagChanged),
    /// A chat member extended their subscription to the chat
    #[serde(rename(serialize = "chatEventMemberSubscriptionExtended", deserialize = "chatEventMemberSubscriptionExtended"))]
    ChatEventMemberSubscriptionExtended(crate::types::ChatEventMemberSubscriptionExtended),
    /// The chat available reactions were changed
    #[serde(rename(serialize = "chatEventAvailableReactionsChanged", deserialize = "chatEventAvailableReactionsChanged"))]
    ChatEventAvailableReactionsChanged(crate::types::ChatEventAvailableReactionsChanged),
    /// The chat background was changed
    #[serde(rename(serialize = "chatEventBackgroundChanged", deserialize = "chatEventBackgroundChanged"))]
    ChatEventBackgroundChanged(crate::types::ChatEventBackgroundChanged),
    /// The chat description was changed
    #[serde(rename(serialize = "chatEventDescriptionChanged", deserialize = "chatEventDescriptionChanged"))]
    ChatEventDescriptionChanged(crate::types::ChatEventDescriptionChanged),
    /// The chat emoji status was changed
    #[serde(rename(serialize = "chatEventEmojiStatusChanged", deserialize = "chatEventEmojiStatusChanged"))]
    ChatEventEmojiStatusChanged(crate::types::ChatEventEmojiStatusChanged),
    /// The linked chat of a supergroup was changed
    #[serde(rename(serialize = "chatEventLinkedChatChanged", deserialize = "chatEventLinkedChatChanged"))]
    ChatEventLinkedChatChanged(crate::types::ChatEventLinkedChatChanged),
    /// The supergroup location was changed
    #[serde(rename(serialize = "chatEventLocationChanged", deserialize = "chatEventLocationChanged"))]
    ChatEventLocationChanged(crate::types::ChatEventLocationChanged),
    /// The message auto-delete timer was changed
    #[serde(rename(serialize = "chatEventMessageAutoDeleteTimeChanged", deserialize = "chatEventMessageAutoDeleteTimeChanged"))]
    ChatEventMessageAutoDeleteTimeChanged(crate::types::ChatEventMessageAutoDeleteTimeChanged),
    /// The chat permissions were changed
    #[serde(rename(serialize = "chatEventPermissionsChanged", deserialize = "chatEventPermissionsChanged"))]
    ChatEventPermissionsChanged(crate::types::ChatEventPermissionsChanged),
    /// The chat photo was changed
    #[serde(rename(serialize = "chatEventPhotoChanged", deserialize = "chatEventPhotoChanged"))]
    ChatEventPhotoChanged(crate::types::ChatEventPhotoChanged),
    /// The slow_mode_delay setting of a supergroup was changed
    #[serde(rename(serialize = "chatEventSlowModeDelayChanged", deserialize = "chatEventSlowModeDelayChanged"))]
    ChatEventSlowModeDelayChanged(crate::types::ChatEventSlowModeDelayChanged),
    /// The supergroup sticker set was changed
    #[serde(rename(serialize = "chatEventStickerSetChanged", deserialize = "chatEventStickerSetChanged"))]
    ChatEventStickerSetChanged(crate::types::ChatEventStickerSetChanged),
    /// The supergroup sticker set with allowed custom emoji was changed
    #[serde(rename(serialize = "chatEventCustomEmojiStickerSetChanged", deserialize = "chatEventCustomEmojiStickerSetChanged"))]
    ChatEventCustomEmojiStickerSetChanged(crate::types::ChatEventCustomEmojiStickerSetChanged),
    /// The chat title was changed
    #[serde(rename(serialize = "chatEventTitleChanged", deserialize = "chatEventTitleChanged"))]
    ChatEventTitleChanged(crate::types::ChatEventTitleChanged),
    /// The chat editable username was changed
    #[serde(rename(serialize = "chatEventUsernameChanged", deserialize = "chatEventUsernameChanged"))]
    ChatEventUsernameChanged(crate::types::ChatEventUsernameChanged),
    /// The chat active usernames were changed
    #[serde(rename(serialize = "chatEventActiveUsernamesChanged", deserialize = "chatEventActiveUsernamesChanged"))]
    ChatEventActiveUsernamesChanged(crate::types::ChatEventActiveUsernamesChanged),
    /// The chat accent color or background custom emoji were changed
    #[serde(rename(serialize = "chatEventAccentColorChanged", deserialize = "chatEventAccentColorChanged"))]
    ChatEventAccentColorChanged(crate::types::ChatEventAccentColorChanged),
    /// The chat's profile accent color or profile background custom emoji were changed
    #[serde(rename(serialize = "chatEventProfileAccentColorChanged", deserialize = "chatEventProfileAccentColorChanged"))]
    ChatEventProfileAccentColorChanged(crate::types::ChatEventProfileAccentColorChanged),
    /// The has_protected_content setting of a channel was toggled
    #[serde(rename(serialize = "chatEventHasProtectedContentToggled", deserialize = "chatEventHasProtectedContentToggled"))]
    ChatEventHasProtectedContentToggled(crate::types::ChatEventHasProtectedContentToggled),
    /// The can_invite_users permission of a supergroup chat was toggled
    #[serde(rename(serialize = "chatEventInvitesToggled", deserialize = "chatEventInvitesToggled"))]
    ChatEventInvitesToggled(crate::types::ChatEventInvitesToggled),
    /// The is_all_history_available setting of a supergroup was toggled
    #[serde(rename(serialize = "chatEventIsAllHistoryAvailableToggled", deserialize = "chatEventIsAllHistoryAvailableToggled"))]
    ChatEventIsAllHistoryAvailableToggled(crate::types::ChatEventIsAllHistoryAvailableToggled),
    /// The has_aggressive_anti_spam_enabled setting of a supergroup was toggled
    #[serde(rename(serialize = "chatEventHasAggressiveAntiSpamEnabledToggled", deserialize = "chatEventHasAggressiveAntiSpamEnabledToggled"))]
    ChatEventHasAggressiveAntiSpamEnabledToggled(crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled),
    /// The sign_messages setting of a channel was toggled
    #[serde(rename(serialize = "chatEventSignMessagesToggled", deserialize = "chatEventSignMessagesToggled"))]
    ChatEventSignMessagesToggled(crate::types::ChatEventSignMessagesToggled),
    /// The show_message_sender setting of a channel was toggled
    #[serde(rename(serialize = "chatEventShowMessageSenderToggled", deserialize = "chatEventShowMessageSenderToggled"))]
    ChatEventShowMessageSenderToggled(crate::types::ChatEventShowMessageSenderToggled),
    /// The has_automatic_translation setting of a channel was toggled
    #[serde(rename(serialize = "chatEventAutomaticTranslationToggled", deserialize = "chatEventAutomaticTranslationToggled"))]
    ChatEventAutomaticTranslationToggled(crate::types::ChatEventAutomaticTranslationToggled),
    /// A chat invite link was edited
    #[serde(rename(serialize = "chatEventInviteLinkEdited", deserialize = "chatEventInviteLinkEdited"))]
    ChatEventInviteLinkEdited(crate::types::ChatEventInviteLinkEdited),
    /// A chat invite link was revoked
    #[serde(rename(serialize = "chatEventInviteLinkRevoked", deserialize = "chatEventInviteLinkRevoked"))]
    ChatEventInviteLinkRevoked(crate::types::ChatEventInviteLinkRevoked),
    /// A revoked chat invite link was deleted
    #[serde(rename(serialize = "chatEventInviteLinkDeleted", deserialize = "chatEventInviteLinkDeleted"))]
    ChatEventInviteLinkDeleted(crate::types::ChatEventInviteLinkDeleted),
    /// A video chat was created
    #[serde(rename(serialize = "chatEventVideoChatCreated", deserialize = "chatEventVideoChatCreated"))]
    ChatEventVideoChatCreated(crate::types::ChatEventVideoChatCreated),
    /// A video chat was ended
    #[serde(rename(serialize = "chatEventVideoChatEnded", deserialize = "chatEventVideoChatEnded"))]
    ChatEventVideoChatEnded(crate::types::ChatEventVideoChatEnded),
    /// The mute_new_participants setting of a video chat was toggled
    #[serde(rename(serialize = "chatEventVideoChatMuteNewParticipantsToggled", deserialize = "chatEventVideoChatMuteNewParticipantsToggled"))]
    ChatEventVideoChatMuteNewParticipantsToggled(crate::types::ChatEventVideoChatMuteNewParticipantsToggled),
    /// A video chat participant was muted or unmuted
    #[serde(rename(serialize = "chatEventVideoChatParticipantIsMutedToggled", deserialize = "chatEventVideoChatParticipantIsMutedToggled"))]
    ChatEventVideoChatParticipantIsMutedToggled(crate::types::ChatEventVideoChatParticipantIsMutedToggled),
    /// A video chat participant volume level was changed
    #[serde(rename(serialize = "chatEventVideoChatParticipantVolumeLevelChanged", deserialize = "chatEventVideoChatParticipantVolumeLevelChanged"))]
    ChatEventVideoChatParticipantVolumeLevelChanged(crate::types::ChatEventVideoChatParticipantVolumeLevelChanged),
    /// The is_forum setting of a channel was toggled
    #[serde(rename(serialize = "chatEventIsForumToggled", deserialize = "chatEventIsForumToggled"))]
    ChatEventIsForumToggled(crate::types::ChatEventIsForumToggled),
    /// A new forum topic was created
    #[serde(rename(serialize = "chatEventForumTopicCreated", deserialize = "chatEventForumTopicCreated"))]
    ChatEventForumTopicCreated(crate::types::ChatEventForumTopicCreated),
    /// A forum topic was edited
    #[serde(rename(serialize = "chatEventForumTopicEdited", deserialize = "chatEventForumTopicEdited"))]
    ChatEventForumTopicEdited(crate::types::ChatEventForumTopicEdited),
    /// A forum topic was closed or reopened
    #[serde(rename(serialize = "chatEventForumTopicToggleIsClosed", deserialize = "chatEventForumTopicToggleIsClosed"))]
    ChatEventForumTopicToggleIsClosed(crate::types::ChatEventForumTopicToggleIsClosed),
    /// The General forum topic was hidden or unhidden
    #[serde(rename(serialize = "chatEventForumTopicToggleIsHidden", deserialize = "chatEventForumTopicToggleIsHidden"))]
    ChatEventForumTopicToggleIsHidden(crate::types::ChatEventForumTopicToggleIsHidden),
    /// A forum topic was deleted
    #[serde(rename(serialize = "chatEventForumTopicDeleted", deserialize = "chatEventForumTopicDeleted"))]
    ChatEventForumTopicDeleted(crate::types::ChatEventForumTopicDeleted),
    /// A pinned forum topic was changed
    #[serde(rename(serialize = "chatEventForumTopicPinned", deserialize = "chatEventForumTopicPinned"))]
    ChatEventForumTopicPinned(crate::types::ChatEventForumTopicPinned),
}
