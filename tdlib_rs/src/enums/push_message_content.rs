#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PushMessageContent {
    /// A general message with hidden content
    #[serde(rename(serialize = "pushMessageContentHidden", deserialize = "pushMessageContentHidden"))]
    Hidden(crate::types::PushMessageContentHidden),
    /// An animation message (GIF-style).
    #[serde(rename(serialize = "pushMessageContentAnimation", deserialize = "pushMessageContentAnimation"))]
    Animation(crate::types::PushMessageContentAnimation),
    /// An audio message
    #[serde(rename(serialize = "pushMessageContentAudio", deserialize = "pushMessageContentAudio"))]
    Audio(crate::types::PushMessageContentAudio),
    /// A message with a user contact
    #[serde(rename(serialize = "pushMessageContentContact", deserialize = "pushMessageContentContact"))]
    Contact(crate::types::PushMessageContentContact),
    /// A contact has registered with Telegram
    #[serde(rename(serialize = "pushMessageContentContactRegistered", deserialize = "pushMessageContentContactRegistered"))]
    ContactRegistered(crate::types::PushMessageContentContactRegistered),
    /// A document message (a general file)
    #[serde(rename(serialize = "pushMessageContentDocument", deserialize = "pushMessageContentDocument"))]
    Document(crate::types::PushMessageContentDocument),
    /// A message with a game
    #[serde(rename(serialize = "pushMessageContentGame", deserialize = "pushMessageContentGame"))]
    Game(crate::types::PushMessageContentGame),
    /// A new high score was achieved in a game
    #[serde(rename(serialize = "pushMessageContentGameScore", deserialize = "pushMessageContentGameScore"))]
    GameScore(crate::types::PushMessageContentGameScore),
    /// A message with an invoice from a bot
    #[serde(rename(serialize = "pushMessageContentInvoice", deserialize = "pushMessageContentInvoice"))]
    Invoice(crate::types::PushMessageContentInvoice),
    /// A message with a location
    #[serde(rename(serialize = "pushMessageContentLocation", deserialize = "pushMessageContentLocation"))]
    Location(crate::types::PushMessageContentLocation),
    /// A message with paid media
    #[serde(rename(serialize = "pushMessageContentPaidMedia", deserialize = "pushMessageContentPaidMedia"))]
    PaidMedia(crate::types::PushMessageContentPaidMedia),
    /// A photo message
    #[serde(rename(serialize = "pushMessageContentPhoto", deserialize = "pushMessageContentPhoto"))]
    Photo(crate::types::PushMessageContentPhoto),
    /// A message with a poll
    #[serde(rename(serialize = "pushMessageContentPoll", deserialize = "pushMessageContentPoll"))]
    Poll(crate::types::PushMessageContentPoll),
    /// A message with a Telegram Premium gift code created for the user
    #[serde(rename(serialize = "pushMessageContentPremiumGiftCode", deserialize = "pushMessageContentPremiumGiftCode"))]
    PremiumGiftCode(crate::types::PushMessageContentPremiumGiftCode),
    /// A message with a giveaway
    #[serde(rename(serialize = "pushMessageContentGiveaway", deserialize = "pushMessageContentGiveaway"))]
    Giveaway(crate::types::PushMessageContentGiveaway),
    /// A message with a gift
    #[serde(rename(serialize = "pushMessageContentGift", deserialize = "pushMessageContentGift"))]
    Gift(crate::types::PushMessageContentGift),
    /// A message with an upgraded gift
    #[serde(rename(serialize = "pushMessageContentUpgradedGift", deserialize = "pushMessageContentUpgradedGift"))]
    UpgradedGift(crate::types::PushMessageContentUpgradedGift),
    /// A screenshot of a message in the chat has been taken
    #[serde(rename(serialize = "pushMessageContentScreenshotTaken", deserialize = "pushMessageContentScreenshotTaken"))]
    ScreenshotTaken,
    /// A message with a sticker
    #[serde(rename(serialize = "pushMessageContentSticker", deserialize = "pushMessageContentSticker"))]
    Sticker(crate::types::PushMessageContentSticker),
    /// A message with a story
    #[serde(rename(serialize = "pushMessageContentStory", deserialize = "pushMessageContentStory"))]
    Story(crate::types::PushMessageContentStory),
    /// A text message
    #[serde(rename(serialize = "pushMessageContentText", deserialize = "pushMessageContentText"))]
    Text(crate::types::PushMessageContentText),
    /// A message with a checklist
    #[serde(rename(serialize = "pushMessageContentChecklist", deserialize = "pushMessageContentChecklist"))]
    Checklist(crate::types::PushMessageContentChecklist),
    /// A video message
    #[serde(rename(serialize = "pushMessageContentVideo", deserialize = "pushMessageContentVideo"))]
    Video(crate::types::PushMessageContentVideo),
    /// A video note message
    #[serde(rename(serialize = "pushMessageContentVideoNote", deserialize = "pushMessageContentVideoNote"))]
    VideoNote(crate::types::PushMessageContentVideoNote),
    /// A voice note message
    #[serde(rename(serialize = "pushMessageContentVoiceNote", deserialize = "pushMessageContentVoiceNote"))]
    VoiceNote(crate::types::PushMessageContentVoiceNote),
    /// A newly created basic group
    #[serde(rename(serialize = "pushMessageContentBasicGroupChatCreate", deserialize = "pushMessageContentBasicGroupChatCreate"))]
    BasicGroupChatCreate,
    /// A video chat or live stream was started
    #[serde(rename(serialize = "pushMessageContentVideoChatStarted", deserialize = "pushMessageContentVideoChatStarted"))]
    VideoChatStarted,
    /// A video chat or live stream has ended
    #[serde(rename(serialize = "pushMessageContentVideoChatEnded", deserialize = "pushMessageContentVideoChatEnded"))]
    VideoChatEnded,
    /// An invitation of participants to a video chat or live stream
    #[serde(rename(serialize = "pushMessageContentInviteVideoChatParticipants", deserialize = "pushMessageContentInviteVideoChatParticipants"))]
    InviteVideoChatParticipants(crate::types::PushMessageContentInviteVideoChatParticipants),
    /// New chat members were invited to a group
    #[serde(rename(serialize = "pushMessageContentChatAddMembers", deserialize = "pushMessageContentChatAddMembers"))]
    ChatAddMembers(crate::types::PushMessageContentChatAddMembers),
    /// A chat photo was edited
    #[serde(rename(serialize = "pushMessageContentChatChangePhoto", deserialize = "pushMessageContentChatChangePhoto"))]
    ChatChangePhoto,
    /// A chat title was edited
    #[serde(rename(serialize = "pushMessageContentChatChangeTitle", deserialize = "pushMessageContentChatChangeTitle"))]
    ChatChangeTitle(crate::types::PushMessageContentChatChangeTitle),
    /// A chat background was edited
    #[serde(rename(serialize = "pushMessageContentChatSetBackground", deserialize = "pushMessageContentChatSetBackground"))]
    ChatSetBackground(crate::types::PushMessageContentChatSetBackground),
    /// A chat theme was edited
    #[serde(rename(serialize = "pushMessageContentChatSetTheme", deserialize = "pushMessageContentChatSetTheme"))]
    ChatSetTheme(crate::types::PushMessageContentChatSetTheme),
    /// A chat member was deleted
    #[serde(rename(serialize = "pushMessageContentChatDeleteMember", deserialize = "pushMessageContentChatDeleteMember"))]
    ChatDeleteMember(crate::types::PushMessageContentChatDeleteMember),
    /// A new member joined the chat via an invite link
    #[serde(rename(serialize = "pushMessageContentChatJoinByLink", deserialize = "pushMessageContentChatJoinByLink"))]
    ChatJoinByLink,
    /// A new member was accepted to the chat by an administrator
    #[serde(rename(serialize = "pushMessageContentChatJoinByRequest", deserialize = "pushMessageContentChatJoinByRequest"))]
    ChatJoinByRequest,
    /// A new recurring payment was made by the current user
    #[serde(rename(serialize = "pushMessageContentRecurringPayment", deserialize = "pushMessageContentRecurringPayment"))]
    RecurringPayment(crate::types::PushMessageContentRecurringPayment),
    /// A profile photo was suggested to the user
    #[serde(rename(serialize = "pushMessageContentSuggestProfilePhoto", deserialize = "pushMessageContentSuggestProfilePhoto"))]
    SuggestProfilePhoto,
    /// A birthdate was suggested to be set
    #[serde(rename(serialize = "pushMessageContentSuggestBirthdate", deserialize = "pushMessageContentSuggestBirthdate"))]
    SuggestBirthdate,
    /// A user in the chat came within proximity alert range from the current user
    #[serde(rename(serialize = "pushMessageContentProximityAlertTriggered", deserialize = "pushMessageContentProximityAlertTriggered"))]
    ProximityAlertTriggered(crate::types::PushMessageContentProximityAlertTriggered),
    /// Some tasks were added to a checklist
    #[serde(rename(serialize = "pushMessageContentChecklistTasksAdded", deserialize = "pushMessageContentChecklistTasksAdded"))]
    ChecklistTasksAdded(crate::types::PushMessageContentChecklistTasksAdded),
    /// Some tasks from a checklist were marked as done or not done
    #[serde(rename(serialize = "pushMessageContentChecklistTasksDone", deserialize = "pushMessageContentChecklistTasksDone"))]
    ChecklistTasksDone(crate::types::PushMessageContentChecklistTasksDone),
    /// A forwarded messages
    #[serde(rename(serialize = "pushMessageContentMessageForwards", deserialize = "pushMessageContentMessageForwards"))]
    MessageForwards(crate::types::PushMessageContentMessageForwards),
    /// A media album
    #[serde(rename(serialize = "pushMessageContentMediaAlbum", deserialize = "pushMessageContentMediaAlbum"))]
    MediaAlbum(crate::types::PushMessageContentMediaAlbum),
}
