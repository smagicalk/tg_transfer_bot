#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumLimitType {
    /// The maximum number of joined supergroups and channels
    #[serde(rename(serialize = "premiumLimitTypeSupergroupCount", deserialize = "premiumLimitTypeSupergroupCount"))]
    SupergroupCount,
    /// The maximum number of pinned chats in the main chat list
    #[serde(rename(serialize = "premiumLimitTypePinnedChatCount", deserialize = "premiumLimitTypePinnedChatCount"))]
    PinnedChatCount,
    /// The maximum number of created public chats
    #[serde(rename(serialize = "premiumLimitTypeCreatedPublicChatCount", deserialize = "premiumLimitTypeCreatedPublicChatCount"))]
    CreatedPublicChatCount,
    /// The maximum number of saved animations
    #[serde(rename(serialize = "premiumLimitTypeSavedAnimationCount", deserialize = "premiumLimitTypeSavedAnimationCount"))]
    SavedAnimationCount,
    /// The maximum number of favorite stickers
    #[serde(rename(serialize = "premiumLimitTypeFavoriteStickerCount", deserialize = "premiumLimitTypeFavoriteStickerCount"))]
    FavoriteStickerCount,
    /// The maximum number of chat folders
    #[serde(rename(serialize = "premiumLimitTypeChatFolderCount", deserialize = "premiumLimitTypeChatFolderCount"))]
    ChatFolderCount,
    /// The maximum number of pinned and always included, or always excluded chats in a chat folder
    #[serde(rename(serialize = "premiumLimitTypeChatFolderChosenChatCount", deserialize = "premiumLimitTypeChatFolderChosenChatCount"))]
    ChatFolderChosenChatCount,
    /// The maximum number of pinned chats in the archive chat list
    #[serde(rename(serialize = "premiumLimitTypePinnedArchivedChatCount", deserialize = "premiumLimitTypePinnedArchivedChatCount"))]
    PinnedArchivedChatCount,
    /// The maximum number of pinned Saved Messages topics
    #[serde(rename(serialize = "premiumLimitTypePinnedSavedMessagesTopicCount", deserialize = "premiumLimitTypePinnedSavedMessagesTopicCount"))]
    PinnedSavedMessagesTopicCount,
    /// The maximum length of sent media caption
    #[serde(rename(serialize = "premiumLimitTypeCaptionLength", deserialize = "premiumLimitTypeCaptionLength"))]
    CaptionLength,
    /// The maximum length of the user's bio
    #[serde(rename(serialize = "premiumLimitTypeBioLength", deserialize = "premiumLimitTypeBioLength"))]
    BioLength,
    /// The maximum number of invite links for a chat folder
    #[serde(rename(serialize = "premiumLimitTypeChatFolderInviteLinkCount", deserialize = "premiumLimitTypeChatFolderInviteLinkCount"))]
    ChatFolderInviteLinkCount,
    /// The maximum number of added shareable chat folders
    #[serde(rename(serialize = "premiumLimitTypeShareableChatFolderCount", deserialize = "premiumLimitTypeShareableChatFolderCount"))]
    ShareableChatFolderCount,
    /// The maximum number of active stories
    #[serde(rename(serialize = "premiumLimitTypeActiveStoryCount", deserialize = "premiumLimitTypeActiveStoryCount"))]
    ActiveStoryCount,
    /// The maximum number of stories posted per week
    #[serde(rename(serialize = "premiumLimitTypeWeeklyPostedStoryCount", deserialize = "premiumLimitTypeWeeklyPostedStoryCount"))]
    WeeklyPostedStoryCount,
    /// The maximum number of stories posted per month
    #[serde(rename(serialize = "premiumLimitTypeMonthlyPostedStoryCount", deserialize = "premiumLimitTypeMonthlyPostedStoryCount"))]
    MonthlyPostedStoryCount,
    /// The maximum length of captions of posted stories
    #[serde(rename(serialize = "premiumLimitTypeStoryCaptionLength", deserialize = "premiumLimitTypeStoryCaptionLength"))]
    StoryCaptionLength,
    /// The maximum number of suggested reaction areas on a story
    #[serde(rename(serialize = "premiumLimitTypeStorySuggestedReactionAreaCount", deserialize = "premiumLimitTypeStorySuggestedReactionAreaCount"))]
    StorySuggestedReactionAreaCount,
    /// The maximum number of received similar chats
    #[serde(rename(serialize = "premiumLimitTypeSimilarChatCount", deserialize = "premiumLimitTypeSimilarChatCount"))]
    SimilarChatCount,
}
