//!
//! TDLib `sticker` domain enums.
//!
//! Types, enums, and functions for stickers, custom emoji sets, and animated dice.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes format of a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerFormat {
    /// The sticker is an image in WEBP format
    #[serde(rename(serialize = "stickerFormatWebp", deserialize = "stickerFormatWebp"))]
    Webp,
    /// The sticker is an animation in TGS format
    #[serde(rename(serialize = "stickerFormatTgs", deserialize = "stickerFormatTgs"))]
    Tgs,
    /// The sticker is a video in WEBM format
    #[serde(rename(serialize = "stickerFormatWebm", deserialize = "stickerFormatWebm"))]
    Webm,
}

/// Describes type of sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerType {
    /// The sticker is a regular sticker
    #[serde(rename(serialize = "stickerTypeRegular", deserialize = "stickerTypeRegular"))]
    Regular,
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename(serialize = "stickerTypeMask", deserialize = "stickerTypeMask"))]
    Mask,
    /// The sticker is a custom emoji to be used inside message text and caption
    #[serde(rename(serialize = "stickerTypeCustomEmoji", deserialize = "stickerTypeCustomEmoji"))]
    CustomEmoji,
}

/// Contains full information about sticker type
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerFullType {
    /// The sticker is a regular sticker
    #[serde(rename(serialize = "stickerFullTypeRegular", deserialize = "stickerFullTypeRegular"))]
    Regular(Box<crate::types::StickerFullTypeRegular>),
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename(serialize = "stickerFullTypeMask", deserialize = "stickerFullTypeMask"))]
    Mask(Box<crate::types::StickerFullTypeMask>),
    /// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
    #[serde(rename(serialize = "stickerFullTypeCustomEmoji", deserialize = "stickerFullTypeCustomEmoji"))]
    CustomEmoji(Box<crate::types::StickerFullTypeCustomEmoji>),
}

impl StickerFullType {
    /// Convenience constructor to create a [`StickerFullType::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::StickerFullTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`StickerFullType::Mask`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mask(val: crate::types::StickerFullTypeMask) -> Self {
        Self::Mask(Box::new(val))
    }

    /// Convenience constructor to create a [`StickerFullType::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::StickerFullTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

}

/// Converts a [`crate::types::StickerFullTypeRegular`] into [`StickerFullType`].
impl From<crate::types::StickerFullTypeRegular> for StickerFullType {
    fn from(val: crate::types::StickerFullTypeRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::StickerFullTypeMask`] into [`StickerFullType`].
impl From<crate::types::StickerFullTypeMask> for StickerFullType {
    fn from(val: crate::types::StickerFullTypeMask) -> Self {
        Self::Mask(Box::new(val))
    }
}

/// Converts a [`crate::types::StickerFullTypeCustomEmoji`] into [`StickerFullType`].
impl From<crate::types::StickerFullTypeCustomEmoji> for StickerFullType {
    fn from(val: crate::types::StickerFullTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// TDLib `Sticker` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Sticker {
    /// Describes a sticker
    #[serde(rename(serialize = "sticker", deserialize = "sticker"))]
    Sticker(Box<crate::types::Sticker>),
}

impl Sticker {
    /// Convenience constructor to create a [`Sticker::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::Sticker) -> Self {
        Self::Sticker(Box::new(val))
    }

}

/// Converts a [`crate::types::Sticker`] into [`Sticker`].
impl From<crate::types::Sticker> for Sticker {
    fn from(val: crate::types::Sticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// TDLib `AnimatedEmoji` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AnimatedEmoji {
    /// Describes an animated or custom representation of an emoji
    #[serde(rename(serialize = "animatedEmoji", deserialize = "animatedEmoji"))]
    AnimatedEmoji(Box<crate::types::AnimatedEmoji>),
}

impl AnimatedEmoji {
    /// Convenience constructor to create a [`AnimatedEmoji::AnimatedEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animated_emoji(val: crate::types::AnimatedEmoji) -> Self {
        Self::AnimatedEmoji(Box::new(val))
    }

}

/// Converts a [`crate::types::AnimatedEmoji`] into [`AnimatedEmoji`].
impl From<crate::types::AnimatedEmoji> for AnimatedEmoji {
    fn from(val: crate::types::AnimatedEmoji) -> Self {
        Self::AnimatedEmoji(Box::new(val))
    }
}

/// TDLib `StakeDiceState` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StakeDiceState {
    /// Describes state of the stake dice
    #[serde(rename(serialize = "stakeDiceState", deserialize = "stakeDiceState"))]
    StakeDiceState(Box<crate::types::StakeDiceState>),
}

impl StakeDiceState {
    /// Convenience constructor to create a [`StakeDiceState::StakeDiceState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stake_dice_state(val: crate::types::StakeDiceState) -> Self {
        Self::StakeDiceState(Box::new(val))
    }

}

/// Converts a [`crate::types::StakeDiceState`] into [`StakeDiceState`].
impl From<crate::types::StakeDiceState> for StakeDiceState {
    fn from(val: crate::types::StakeDiceState) -> Self {
        Self::StakeDiceState(Box::new(val))
    }
}

/// Describes type of sticker, which was used to create a chat photo
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotoStickerType {
    /// Information about the sticker, which was used to create the chat photo
    #[serde(rename(serialize = "chatPhotoStickerTypeRegularOrMask", deserialize = "chatPhotoStickerTypeRegularOrMask"))]
    RegularOrMask(Box<crate::types::ChatPhotoStickerTypeRegularOrMask>),
    /// Information about the custom emoji, which was used to create the chat photo
    #[serde(rename(serialize = "chatPhotoStickerTypeCustomEmoji", deserialize = "chatPhotoStickerTypeCustomEmoji"))]
    CustomEmoji(Box<crate::types::ChatPhotoStickerTypeCustomEmoji>),
}

impl ChatPhotoStickerType {
    /// Convenience constructor to create a [`ChatPhotoStickerType::RegularOrMask`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular_or_mask(val: crate::types::ChatPhotoStickerTypeRegularOrMask) -> Self {
        Self::RegularOrMask(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatPhotoStickerType::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::ChatPhotoStickerTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatPhotoStickerTypeRegularOrMask`] into [`ChatPhotoStickerType`].
impl From<crate::types::ChatPhotoStickerTypeRegularOrMask> for ChatPhotoStickerType {
    fn from(val: crate::types::ChatPhotoStickerTypeRegularOrMask) -> Self {
        Self::RegularOrMask(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatPhotoStickerTypeCustomEmoji`] into [`ChatPhotoStickerType`].
impl From<crate::types::ChatPhotoStickerTypeCustomEmoji> for ChatPhotoStickerType {
    fn from(val: crate::types::ChatPhotoStickerTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// TDLib `ChatPhotoSticker` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotoSticker {
    /// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
    #[serde(rename(serialize = "chatPhotoSticker", deserialize = "chatPhotoSticker"))]
    ChatPhotoSticker(Box<crate::types::ChatPhotoSticker>),
}

impl ChatPhotoSticker {
    /// Convenience constructor to create a [`ChatPhotoSticker::ChatPhotoSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_photo_sticker(val: crate::types::ChatPhotoSticker) -> Self {
        Self::ChatPhotoSticker(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatPhotoSticker`] into [`ChatPhotoSticker`].
impl From<crate::types::ChatPhotoSticker> for ChatPhotoSticker {
    fn from(val: crate::types::ChatPhotoSticker) -> Self {
        Self::ChatPhotoSticker(Box::new(val))
    }
}

/// TDLib `AnimatedChatPhoto` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AnimatedChatPhoto {
    /// Animated variant of a chat photo in MPEG4 format
    #[serde(rename(serialize = "animatedChatPhoto", deserialize = "animatedChatPhoto"))]
    AnimatedChatPhoto(Box<crate::types::AnimatedChatPhoto>),
}

impl AnimatedChatPhoto {
    /// Convenience constructor to create a [`AnimatedChatPhoto::AnimatedChatPhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animated_chat_photo(val: crate::types::AnimatedChatPhoto) -> Self {
        Self::AnimatedChatPhoto(Box::new(val))
    }

}

/// Converts a [`crate::types::AnimatedChatPhoto`] into [`AnimatedChatPhoto`].
impl From<crate::types::AnimatedChatPhoto> for AnimatedChatPhoto {
    fn from(val: crate::types::AnimatedChatPhoto) -> Self {
        Self::AnimatedChatPhoto(Box::new(val))
    }
}

/// Describes type of emoji status
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatusType {
    /// A custom emoji set as emoji status
    #[serde(rename(serialize = "emojiStatusTypeCustomEmoji", deserialize = "emojiStatusTypeCustomEmoji"))]
    CustomEmoji(Box<crate::types::EmojiStatusTypeCustomEmoji>),
    /// An upgraded gift set as emoji status
    #[serde(rename(serialize = "emojiStatusTypeUpgradedGift", deserialize = "emojiStatusTypeUpgradedGift"))]
    UpgradedGift(Box<crate::types::EmojiStatusTypeUpgradedGift>),
}

impl EmojiStatusType {
    /// Convenience constructor to create a [`EmojiStatusType::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::EmojiStatusTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

    /// Convenience constructor to create a [`EmojiStatusType::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::EmojiStatusTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiStatusTypeCustomEmoji`] into [`EmojiStatusType`].
impl From<crate::types::EmojiStatusTypeCustomEmoji> for EmojiStatusType {
    fn from(val: crate::types::EmojiStatusTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// Converts a [`crate::types::EmojiStatusTypeUpgradedGift`] into [`EmojiStatusType`].
impl From<crate::types::EmojiStatusTypeUpgradedGift> for EmojiStatusType {
    fn from(val: crate::types::EmojiStatusTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// TDLib `EmojiStatus` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatus {
    /// Describes an emoji to be shown instead of the Telegram Premium badge
    #[serde(rename(serialize = "emojiStatus", deserialize = "emojiStatus"))]
    EmojiStatus(Box<crate::types::EmojiStatus>),
}

impl EmojiStatus {
    /// Convenience constructor to create a [`EmojiStatus::EmojiStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_status(val: crate::types::EmojiStatus) -> Self {
        Self::EmojiStatus(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiStatus`] into [`EmojiStatus`].
impl From<crate::types::EmojiStatus> for EmojiStatus {
    fn from(val: crate::types::EmojiStatus) -> Self {
        Self::EmojiStatus(Box::new(val))
    }
}

/// TDLib `EmojiStatuses` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatuses {
    /// Contains a list of emoji statuses
    #[serde(rename(serialize = "emojiStatuses", deserialize = "emojiStatuses"))]
    EmojiStatuses(Box<crate::types::EmojiStatuses>),
}

impl EmojiStatuses {
    /// Convenience constructor to create a [`EmojiStatuses::EmojiStatuses`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_statuses(val: crate::types::EmojiStatuses) -> Self {
        Self::EmojiStatuses(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiStatuses`] into [`EmojiStatuses`].
impl From<crate::types::EmojiStatuses> for EmojiStatuses {
    fn from(val: crate::types::EmojiStatuses) -> Self {
        Self::EmojiStatuses(Box::new(val))
    }
}

/// TDLib `EmojiStatusCustomEmojis` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatusCustomEmojis {
    /// Contains a list of custom emoji identifiers for emoji statuses
    #[serde(rename(serialize = "emojiStatusCustomEmojis", deserialize = "emojiStatusCustomEmojis"))]
    EmojiStatusCustomEmojis(Box<crate::types::EmojiStatusCustomEmojis>),
}

impl EmojiStatusCustomEmojis {
    /// Convenience constructor to create a [`EmojiStatusCustomEmojis::EmojiStatusCustomEmojis`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_status_custom_emojis(val: crate::types::EmojiStatusCustomEmojis) -> Self {
        Self::EmojiStatusCustomEmojis(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiStatusCustomEmojis`] into [`EmojiStatusCustomEmojis`].
impl From<crate::types::EmojiStatusCustomEmojis> for EmojiStatusCustomEmojis {
    fn from(val: crate::types::EmojiStatusCustomEmojis) -> Self {
        Self::EmojiStatusCustomEmojis(Box::new(val))
    }
}

/// TDLib `InputSticker` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputSticker {
    /// A sticker to be sent
    #[serde(rename(serialize = "inputSticker", deserialize = "inputSticker"))]
    InputSticker(Box<crate::types::InputSticker>),
}

impl InputSticker {
    /// Convenience constructor to create a [`InputSticker::InputSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_sticker(val: crate::types::InputSticker) -> Self {
        Self::InputSticker(Box::new(val))
    }

}

/// Converts a [`crate::types::InputSticker`] into [`InputSticker`].
impl From<crate::types::InputSticker> for InputSticker {
    fn from(val: crate::types::InputSticker) -> Self {
        Self::InputSticker(Box::new(val))
    }
}

/// TDLib `EmojiKeyword` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiKeyword {
    /// Represents an emoji with its keyword
    #[serde(rename(serialize = "emojiKeyword", deserialize = "emojiKeyword"))]
    EmojiKeyword(Box<crate::types::EmojiKeyword>),
}

impl EmojiKeyword {
    /// Convenience constructor to create a [`EmojiKeyword::EmojiKeyword`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_keyword(val: crate::types::EmojiKeyword) -> Self {
        Self::EmojiKeyword(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiKeyword`] into [`EmojiKeyword`].
impl From<crate::types::EmojiKeyword> for EmojiKeyword {
    fn from(val: crate::types::EmojiKeyword) -> Self {
        Self::EmojiKeyword(Box::new(val))
    }
}

/// TDLib `EmojiKeywords` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiKeywords {
    /// Represents a list of emojis with their keywords
    #[serde(rename(serialize = "emojiKeywords", deserialize = "emojiKeywords"))]
    EmojiKeywords(Box<crate::types::EmojiKeywords>),
}

impl EmojiKeywords {
    /// Convenience constructor to create a [`EmojiKeywords::EmojiKeywords`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_keywords(val: crate::types::EmojiKeywords) -> Self {
        Self::EmojiKeywords(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiKeywords`] into [`EmojiKeywords`].
impl From<crate::types::EmojiKeywords> for EmojiKeywords {
    fn from(val: crate::types::EmojiKeywords) -> Self {
        Self::EmojiKeywords(Box::new(val))
    }
}

/// TDLib `Stickers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Stickers {
    /// Represents a list of stickers
    #[serde(rename(serialize = "stickers", deserialize = "stickers"))]
    Stickers(Box<crate::types::Stickers>),
}

impl Stickers {
    /// Convenience constructor to create a [`Stickers::Stickers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stickers(val: crate::types::Stickers) -> Self {
        Self::Stickers(Box::new(val))
    }

}

/// Converts a [`crate::types::Stickers`] into [`Stickers`].
impl From<crate::types::Stickers> for Stickers {
    fn from(val: crate::types::Stickers) -> Self {
        Self::Stickers(Box::new(val))
    }
}

/// TDLib `Emojis` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Emojis {
    /// Represents a list of emojis
    #[serde(rename(serialize = "emojis", deserialize = "emojis"))]
    Emojis(Box<crate::types::Emojis>),
}

impl Emojis {
    /// Convenience constructor to create a [`Emojis::Emojis`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emojis(val: crate::types::Emojis) -> Self {
        Self::Emojis(Box::new(val))
    }

}

/// Converts a [`crate::types::Emojis`] into [`Emojis`].
impl From<crate::types::Emojis> for Emojis {
    fn from(val: crate::types::Emojis) -> Self {
        Self::Emojis(Box::new(val))
    }
}

/// TDLib `StickerSet` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSet {
    /// Represents a sticker set
    #[serde(rename(serialize = "stickerSet", deserialize = "stickerSet"))]
    StickerSet(Box<crate::types::StickerSet>),
}

impl StickerSet {
    /// Convenience constructor to create a [`StickerSet::StickerSet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set(val: crate::types::StickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }

}

/// Converts a [`crate::types::StickerSet`] into [`StickerSet`].
impl From<crate::types::StickerSet> for StickerSet {
    fn from(val: crate::types::StickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }
}

/// TDLib `StickerSetInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSetInfo {
    /// Represents short information about a sticker set
    #[serde(rename(serialize = "stickerSetInfo", deserialize = "stickerSetInfo"))]
    StickerSetInfo(Box<crate::types::StickerSetInfo>),
}

impl StickerSetInfo {
    /// Convenience constructor to create a [`StickerSetInfo::StickerSetInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set_info(val: crate::types::StickerSetInfo) -> Self {
        Self::StickerSetInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::StickerSetInfo`] into [`StickerSetInfo`].
impl From<crate::types::StickerSetInfo> for StickerSetInfo {
    fn from(val: crate::types::StickerSetInfo) -> Self {
        Self::StickerSetInfo(Box::new(val))
    }
}

/// TDLib `StickerSets` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerSets {
    /// Represents a list of sticker sets
    #[serde(rename(serialize = "stickerSets", deserialize = "stickerSets"))]
    StickerSets(Box<crate::types::StickerSets>),
}

impl StickerSets {
    /// Convenience constructor to create a [`StickerSets::StickerSets`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_sets(val: crate::types::StickerSets) -> Self {
        Self::StickerSets(Box::new(val))
    }

}

/// Converts a [`crate::types::StickerSets`] into [`StickerSets`].
impl From<crate::types::StickerSets> for StickerSets {
    fn from(val: crate::types::StickerSets) -> Self {
        Self::StickerSets(Box::new(val))
    }
}

/// TDLib `TrendingStickerSets` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TrendingStickerSets {
    /// Represents a list of trending sticker sets
    #[serde(rename(serialize = "trendingStickerSets", deserialize = "trendingStickerSets"))]
    TrendingStickerSets(Box<crate::types::TrendingStickerSets>),
}

impl TrendingStickerSets {
    /// Convenience constructor to create a [`TrendingStickerSets::TrendingStickerSets`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn trending_sticker_sets(val: crate::types::TrendingStickerSets) -> Self {
        Self::TrendingStickerSets(Box::new(val))
    }

}

/// Converts a [`crate::types::TrendingStickerSets`] into [`TrendingStickerSets`].
impl From<crate::types::TrendingStickerSets> for TrendingStickerSets {
    fn from(val: crate::types::TrendingStickerSets) -> Self {
        Self::TrendingStickerSets(Box::new(val))
    }
}

/// Describes source of stickers for an emoji category
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategorySource {
    /// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers,
    /// or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
    #[serde(rename(serialize = "emojiCategorySourceSearch", deserialize = "emojiCategorySourceSearch"))]
    Search(Box<crate::types::EmojiCategorySourceSearch>),
    /// The category contains premium stickers that must be found by getPremiumStickers
    #[serde(rename(serialize = "emojiCategorySourcePremium", deserialize = "emojiCategorySourcePremium"))]
    Premium,
}

impl EmojiCategorySource {
    /// Convenience constructor to create a [`EmojiCategorySource::Search`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn search(val: crate::types::EmojiCategorySourceSearch) -> Self {
        Self::Search(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiCategorySourceSearch`] into [`EmojiCategorySource`].
impl From<crate::types::EmojiCategorySourceSearch> for EmojiCategorySource {
    fn from(val: crate::types::EmojiCategorySourceSearch) -> Self {
        Self::Search(Box::new(val))
    }
}

/// TDLib `EmojiCategory` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategory {
    /// Describes an emoji category
    #[serde(rename(serialize = "emojiCategory", deserialize = "emojiCategory"))]
    EmojiCategory(Box<crate::types::EmojiCategory>),
}

impl EmojiCategory {
    /// Convenience constructor to create a [`EmojiCategory::EmojiCategory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_category(val: crate::types::EmojiCategory) -> Self {
        Self::EmojiCategory(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiCategory`] into [`EmojiCategory`].
impl From<crate::types::EmojiCategory> for EmojiCategory {
    fn from(val: crate::types::EmojiCategory) -> Self {
        Self::EmojiCategory(Box::new(val))
    }
}

/// TDLib `EmojiCategories` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategories {
    /// Represents a list of emoji categories
    #[serde(rename(serialize = "emojiCategories", deserialize = "emojiCategories"))]
    EmojiCategories(Box<crate::types::EmojiCategories>),
}

impl EmojiCategories {
    /// Convenience constructor to create a [`EmojiCategories::EmojiCategories`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_categories(val: crate::types::EmojiCategories) -> Self {
        Self::EmojiCategories(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiCategories`] into [`EmojiCategories`].
impl From<crate::types::EmojiCategories> for EmojiCategories {
    fn from(val: crate::types::EmojiCategories) -> Self {
        Self::EmojiCategories(Box::new(val))
    }
}

/// Describes type of emoji category
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategoryType {
    /// The category must be used by default (e.g., for custom emoji or animation search)
    #[serde(rename(serialize = "emojiCategoryTypeDefault", deserialize = "emojiCategoryTypeDefault"))]
    Default,
    /// The category must be used by default for regular sticker selection. It may contain greeting emoji category and premium stickers
    #[serde(rename(serialize = "emojiCategoryTypeRegularStickers", deserialize = "emojiCategoryTypeRegularStickers"))]
    RegularStickers,
    /// The category must be used for emoji status selection
    #[serde(rename(serialize = "emojiCategoryTypeEmojiStatus", deserialize = "emojiCategoryTypeEmojiStatus"))]
    EmojiStatus,
    /// The category must be used for chat photo emoji selection
    #[serde(rename(serialize = "emojiCategoryTypeChatPhoto", deserialize = "emojiCategoryTypeChatPhoto"))]
    ChatPhoto,
}

/// TDLib `EmojiReaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiReaction {
    /// Contains information about an emoji reaction
    #[serde(rename(serialize = "emojiReaction", deserialize = "emojiReaction"))]
    EmojiReaction(Box<crate::types::EmojiReaction>),
}

impl EmojiReaction {
    /// Convenience constructor to create a [`EmojiReaction::EmojiReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_reaction(val: crate::types::EmojiReaction) -> Self {
        Self::EmojiReaction(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiReaction`] into [`EmojiReaction`].
impl From<crate::types::EmojiReaction> for EmojiReaction {
    fn from(val: crate::types::EmojiReaction) -> Self {
        Self::EmojiReaction(Box::new(val))
    }
}

/// Contains animated stickers which must be used for dice animation rendering
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiceStickers {
    /// A regular animated sticker
    #[serde(rename(serialize = "diceStickersRegular", deserialize = "diceStickersRegular"))]
    Regular(Box<crate::types::DiceStickersRegular>),
    /// Animated stickers to be combined into a slot machine
    #[serde(rename(serialize = "diceStickersSlotMachine", deserialize = "diceStickersSlotMachine"))]
    SlotMachine(Box<crate::types::DiceStickersSlotMachine>),
}

impl DiceStickers {
    /// Convenience constructor to create a [`DiceStickers::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::DiceStickersRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`DiceStickers::SlotMachine`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn slot_machine(val: crate::types::DiceStickersSlotMachine) -> Self {
        Self::SlotMachine(Box::new(val))
    }

}

/// Converts a [`crate::types::DiceStickersRegular`] into [`DiceStickers`].
impl From<crate::types::DiceStickersRegular> for DiceStickers {
    fn from(val: crate::types::DiceStickersRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::DiceStickersSlotMachine`] into [`DiceStickers`].
impl From<crate::types::DiceStickersSlotMachine> for DiceStickers {
    fn from(val: crate::types::DiceStickersSlotMachine) -> Self {
        Self::SlotMachine(Box::new(val))
    }
}

/// TDLib `EmojiChatTheme` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiChatTheme {
    /// Describes a chat theme based on an emoji
    #[serde(rename(serialize = "emojiChatTheme", deserialize = "emojiChatTheme"))]
    EmojiChatTheme(Box<crate::types::EmojiChatTheme>),
}

impl EmojiChatTheme {
    /// Convenience constructor to create a [`EmojiChatTheme::EmojiChatTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_chat_theme(val: crate::types::EmojiChatTheme) -> Self {
        Self::EmojiChatTheme(Box::new(val))
    }

}

/// Converts a [`crate::types::EmojiChatTheme`] into [`EmojiChatTheme`].
impl From<crate::types::EmojiChatTheme> for EmojiChatTheme {
    fn from(val: crate::types::EmojiChatTheme) -> Self {
        Self::EmojiChatTheme(Box::new(val))
    }
}

/// Represents result of checking whether a name can be used for a new sticker set
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CheckStickerSetNameResult {
    /// The name can be set
    #[serde(rename(serialize = "checkStickerSetNameResultOk", deserialize = "checkStickerSetNameResultOk"))]
    Ok,
    /// The name is invalid
    #[serde(rename(serialize = "checkStickerSetNameResultNameInvalid", deserialize = "checkStickerSetNameResultNameInvalid"))]
    NameInvalid,
    /// The name is occupied
    #[serde(rename(serialize = "checkStickerSetNameResultNameOccupied", deserialize = "checkStickerSetNameResultNameOccupied"))]
    NameOccupied,
}

/// TDLib `NewSticker` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NewSticker {
    /// A sticker to be added to a sticker set
    #[serde(rename(serialize = "newSticker", deserialize = "newSticker"))]
    NewSticker(Box<crate::types::NewSticker>),
}

impl NewSticker {
    /// Convenience constructor to create a [`NewSticker::NewSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_sticker(val: crate::types::NewSticker) -> Self {
        Self::NewSticker(Box::new(val))
    }

}

/// Converts a [`crate::types::NewSticker`] into [`NewSticker`].
impl From<crate::types::NewSticker> for NewSticker {
    fn from(val: crate::types::NewSticker) -> Self {
        Self::NewSticker(Box::new(val))
    }
}

