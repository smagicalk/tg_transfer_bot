#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeature {
    /// Increased limits
    #[serde(rename(
        serialize = "premiumFeatureIncreasedLimits",
        deserialize = "premiumFeatureIncreasedLimits"
    ))]
    IncreasedLimits,
    /// Increased maximum upload file size
    #[serde(rename(
        serialize = "premiumFeatureIncreasedUploadFileSize",
        deserialize = "premiumFeatureIncreasedUploadFileSize"
    ))]
    IncreasedUploadFileSize,
    /// Improved download speed
    #[serde(rename(
        serialize = "premiumFeatureImprovedDownloadSpeed",
        deserialize = "premiumFeatureImprovedDownloadSpeed"
    ))]
    ImprovedDownloadSpeed,
    /// The ability to convert voice notes to text
    #[serde(rename(
        serialize = "premiumFeatureVoiceRecognition",
        deserialize = "premiumFeatureVoiceRecognition"
    ))]
    VoiceRecognition,
    /// Disabled ads
    #[serde(rename(
        serialize = "premiumFeatureDisabledAds",
        deserialize = "premiumFeatureDisabledAds"
    ))]
    DisabledAds,
    /// Allowed to use more reactions
    #[serde(rename(
        serialize = "premiumFeatureUniqueReactions",
        deserialize = "premiumFeatureUniqueReactions"
    ))]
    UniqueReactions,
    /// Allowed to use premium stickers with unique effects
    #[serde(rename(
        serialize = "premiumFeatureUniqueStickers",
        deserialize = "premiumFeatureUniqueStickers"
    ))]
    UniqueStickers,
    /// Allowed to use custom emoji stickers in message texts and captions
    #[serde(rename(
        serialize = "premiumFeatureCustomEmoji",
        deserialize = "premiumFeatureCustomEmoji"
    ))]
    CustomEmoji,
    /// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
    #[serde(rename(
        serialize = "premiumFeatureAdvancedChatManagement",
        deserialize = "premiumFeatureAdvancedChatManagement"
    ))]
    AdvancedChatManagement,
    /// A badge in the user's profile
    #[serde(rename(
        serialize = "premiumFeatureProfileBadge",
        deserialize = "premiumFeatureProfileBadge"
    ))]
    ProfileBadge,
    /// The ability to show an emoji status along with the user's name
    #[serde(rename(
        serialize = "premiumFeatureEmojiStatus",
        deserialize = "premiumFeatureEmojiStatus"
    ))]
    EmojiStatus,
    /// Profile photo animation on message and chat screens
    #[serde(rename(
        serialize = "premiumFeatureAnimatedProfilePhoto",
        deserialize = "premiumFeatureAnimatedProfilePhoto"
    ))]
    AnimatedProfilePhoto,
    /// The ability to set a custom emoji as a forum topic icon
    #[serde(rename(
        serialize = "premiumFeatureForumTopicIcon",
        deserialize = "premiumFeatureForumTopicIcon"
    ))]
    ForumTopicIcon,
    /// Allowed to set a premium application icons
    #[serde(rename(
        serialize = "premiumFeatureAppIcons",
        deserialize = "premiumFeatureAppIcons"
    ))]
    AppIcons,
    /// Allowed to translate chat messages real-time
    #[serde(rename(
        serialize = "premiumFeatureRealTimeChatTranslation",
        deserialize = "premiumFeatureRealTimeChatTranslation"
    ))]
    RealTimeChatTranslation,
    /// Allowed to use many additional features for stories
    #[serde(rename(
        serialize = "premiumFeatureUpgradedStories",
        deserialize = "premiumFeatureUpgradedStories"
    ))]
    UpgradedStories,
    /// The ability to boost chats
    #[serde(rename(
        serialize = "premiumFeatureChatBoost",
        deserialize = "premiumFeatureChatBoost"
    ))]
    ChatBoost,
    /// The ability to choose accent color for replies and user profile
    #[serde(rename(
        serialize = "premiumFeatureAccentColor",
        deserialize = "premiumFeatureAccentColor"
    ))]
    AccentColor,
    /// The ability to set private chat background for both users
    #[serde(rename(
        serialize = "premiumFeatureBackgroundForBoth",
        deserialize = "premiumFeatureBackgroundForBoth"
    ))]
    BackgroundForBoth,
    /// The ability to use tags in Saved Messages
    #[serde(rename(
        serialize = "premiumFeatureSavedMessagesTags",
        deserialize = "premiumFeatureSavedMessagesTags"
    ))]
    SavedMessagesTags,
    /// The ability to disallow incoming voice and video note messages in private chats using setUserPrivacySettingRules with userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages
    /// and to restrict incoming messages from non-contacts using setNewChatPrivacySettings
    #[serde(rename(
        serialize = "premiumFeatureMessagePrivacy",
        deserialize = "premiumFeatureMessagePrivacy"
    ))]
    MessagePrivacy,
    /// The ability to view last seen and read times of other users even if they can't view last seen or read time for the current user
    #[serde(rename(
        serialize = "premiumFeatureLastSeenTimes",
        deserialize = "premiumFeatureLastSeenTimes"
    ))]
    LastSeenTimes,
    /// The ability to use Business features
    #[serde(rename(
        serialize = "premiumFeatureBusiness",
        deserialize = "premiumFeatureBusiness"
    ))]
    Business,
    /// The ability to use all available message effects
    #[serde(rename(
        serialize = "premiumFeatureMessageEffects",
        deserialize = "premiumFeatureMessageEffects"
    ))]
    MessageEffects,
    /// The ability to create and use checklist messages
    #[serde(rename(
        serialize = "premiumFeatureChecklists",
        deserialize = "premiumFeatureChecklists"
    ))]
    Checklists,
    /// The ability to require a payment for incoming messages in new chats
    #[serde(rename(
        serialize = "premiumFeaturePaidMessages",
        deserialize = "premiumFeaturePaidMessages"
    ))]
    PaidMessages,
    /// The ability to enable content protection in private chats
    #[serde(rename(
        serialize = "premiumFeatureProtectPrivateChatContent",
        deserialize = "premiumFeatureProtectPrivateChatContent"
    ))]
    ProtectPrivateChatContent,
}
