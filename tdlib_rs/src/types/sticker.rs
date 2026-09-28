//!
//! TDLib `sticker` domain types.
//!
//! Types, enums, and functions for stickers, custom emoji sets, and animated dice.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The sticker is an image in WEBP format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFormatWebp {
}

/// The sticker is an animation in TGS format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFormatTgs {
}

/// The sticker is a video in WEBM format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFormatWebm {
}

/// The sticker is a regular sticker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerTypeRegular {
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerTypeMask {
}

/// The sticker is a custom emoji to be used inside message text and caption
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerTypeCustomEmoji {
}

/// The sticker is a regular sticker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeRegular {
    /// Premium animation of the sticker; may be null. If present, only Telegram Premium users can use the sticker
    pub premium_animation: Option<crate::types::File>,
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeMask {
    /// Position where the mask is placed; may be null
    pub mask_position: Option<crate::types::MaskPosition>,
}

/// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeCustomEmoji {
    /// Identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
    /// True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    pub needs_repainting: bool,
}

/// Describes a sticker
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Sticker {
    /// Unique sticker identifier within the set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the sticker set to which the sticker belongs; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub set_id: i64,
    /// Sticker width; as defined by the sender
    pub width: i32,
    /// Sticker height; as defined by the sender
    pub height: i32,
    /// Emoji corresponding to the sticker; may be empty if unknown
    pub emoji: String,
    /// Sticker format
    pub format: crate::enums::StickerFormat,
    /// Sticker's full type
    pub full_type: crate::enums::StickerFullType,
    /// Sticker thumbnail in WEBP or JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// File containing the sticker
    pub sticker: crate::types::File,
}

/// Describes an animated or custom representation of an emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnimatedEmoji {
    /// Sticker for the emoji; may be null if yet unknown for a custom emoji. If the sticker is a custom emoji, then it can have arbitrary format
    pub sticker: Option<crate::types::Sticker>,
    /// Expected width of the sticker, which can be used if the sticker is null
    pub sticker_width: i32,
    /// Expected height of the sticker, which can be used if the sticker is null
    pub sticker_height: i32,
    /// Emoji modifier fitzpatrick type; 0-6; 0 if none
    pub fitzpatrick_type: i32,
    /// File containing the sound to be played when the sticker is clicked; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
    pub sound: Option<crate::types::File>,
}

/// Describes state of the stake dice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StakeDiceState {
    /// Hash of the state to use for sending the next dice; may be empty if the stake dice can't be sent by the current user
    pub state_hash: String,
    /// The amount of TON Grams staked in the previous roll; in the smallest units of the currency
    pub stake_gram_amount: i64,
    /// The amounts of Grams that are suggested to be staked; in the smallest units of the currency
    pub suggested_stake_gram_amounts: Vec<i64>,
    /// The number of rolled sixes towards the streak; 0-2
    pub current_streak: i32,
    /// The number of Grams received by the user for each 1000 Grams staked if the dice outcome is 1-6 correspondingly; may be empty if the stake dice can't be sent by the current user
    pub prize_per_mille: Vec<i32>,
    /// The number of Grams received by the user for each 1000 Grams staked if the dice outcome is 6 three times in a row with the same stake
    pub streak_prize_per_mille: i32,
}

/// Information about the sticker, which was used to create the chat photo
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoStickerTypeRegularOrMask {
    /// Sticker set identifier
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
    /// Identifier of the sticker in the set
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}

/// Information about the custom emoji, which was used to create the chat photo
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoStickerTypeCustomEmoji {
    /// Identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoSticker {
    /// Type of the sticker
    #[serde(rename = "type")]
    pub r#type: crate::enums::ChatPhotoStickerType,
    /// The fill to be used as background for the sticker; rotation angle in backgroundFillGradient isn't supported
    pub background_fill: crate::enums::BackgroundFill,
}

/// Animated variant of a chat photo in MPEG4 format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnimatedChatPhoto {
    /// Animation width and height
    pub length: i32,
    /// Information about the animation file
    pub file: crate::types::File,
    /// Timestamp of the frame, used as a static chat photo
    pub main_frame_timestamp: f64,
}

/// A sticker on a custom background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputChatPhotoSticker {
    /// Information about the sticker
    pub sticker: crate::types::ChatPhotoSticker,
}

/// The transaction is a payment for stake dice throw
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeStakeDiceStake {
}

/// The transaction is a payment for successful stake dice throw
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeStakeDicePayout {
}

/// A custom emoji set as emoji status
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusTypeCustomEmoji {
    /// Identifier of the custom emoji in stickerFormatTgs format
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// An upgraded gift set as emoji status
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusTypeUpgradedGift {
    /// Identifier of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub upgraded_gift_id: i64,
    /// The title of the upgraded gift
    pub gift_title: String,
    /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift
    pub gift_name: String,
    /// Custom emoji identifier of the model of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub model_custom_emoji_id: i64,
    /// Custom emoji identifier of the symbol of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub symbol_custom_emoji_id: i64,
    /// Colors of the backdrop of the upgraded gift
    pub backdrop_colors: crate::types::UpgradedGiftBackdropColors,
}

/// Describes an emoji to be shown instead of the Telegram Premium badge
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatus {
    /// Type of the emoji status
    #[serde(rename = "type")]
    pub r#type: crate::enums::EmojiStatusType,
    /// Point in time (Unix timestamp) when the status will expire; 0 if never
    pub expiration_date: i32,
}

/// Contains a list of emoji statuses
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatuses {
    /// The list of emoji statuses identifiers
    pub emoji_statuses: Vec<crate::types::EmojiStatus>,
}

/// Contains a list of custom emoji identifiers for emoji statuses
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusCustomEmojis {
    /// The list of custom emoji identifiers
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub custom_emoji_ids: Vec<i64>,
}

/// A reaction with an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionTypeEmoji {
    /// Text representation of the reaction
    pub emoji: String,
}

/// A reaction with a custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionTypeCustomEmoji {
    /// Unique identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// An effect from an emoji reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffectTypeEmojiReaction {
    /// Select animation for the effect in TGS format
    pub select_animation: crate::types::Sticker,
    /// Effect animation for the effect in TGS format
    pub effect_animation: crate::types::Sticker,
}

/// An effect from a premium sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffectTypePremiumSticker {
    /// The premium sticker. The effect can be found at sticker.full_type.premium_animation
    pub sticker: crate::types::Sticker,
}

/// A custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
    /// Alternative text for the custom emoji
    pub alternative_text: String,
}

/// The link is a link to a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeSticker {
    /// The sticker. It can be an arbitrary WEBP image and can have dimensions bigger than 512
    pub sticker: crate::types::Sticker,
}

/// The link is a link to a sticker set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStickerSet {
    /// Up to 4 stickers from the sticker set
    pub stickers: Vec<crate::types::Sticker>,
}

/// A sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollMediaSticker {
    /// The sticker
    pub sticker: crate::types::Sticker,
}

/// A sticker message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSticker {
    /// The sticker description
    pub sticker: crate::types::Sticker,
    /// True, if premium animation of the sticker must be played
    pub is_premium: bool,
}

/// A message with an animated emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageAnimatedEmoji {
    /// The animated emoji
    pub animated_emoji: crate::types::AnimatedEmoji,
    /// The corresponding emoji
    pub emoji: String,
}

/// A dice message. The dice value is randomly generated by the server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageDice {
    /// The animated stickers with the initial dice animation; may be null if unknown. The update updateMessageContent will be sent when the sticker became known
    pub initial_state: Option<crate::enums::DiceStickers>,
    /// The animated stickers with the final dice animation; may be null if unknown. The update updateMessageContent will be sent when the sticker became known
    pub final_state: Option<crate::enums::DiceStickers>,
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// The dice value. If the value is 0, then the dice don't have final state yet
    pub value: i32,
    /// Number of frame after which a success animation like a shower of confetti needs to be shown on updateMessageSendSucceeded
    pub success_animation_frame_number: i32,
}

/// A stake dice message. The dice value is randomly generated by the server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageStakeDice {
    /// The animated stickers with the initial dice animation; may be null if unknown. The update updateMessageContent will be sent when the sticker became known
    pub initial_state: Option<crate::enums::DiceStickers>,
    /// The animated stickers with the final dice animation; may be null if unknown. The update updateMessageContent will be sent when the sticker became known
    pub final_state: Option<crate::enums::DiceStickers>,
    /// The dice value. If the value is 0, then the dice don't have final state yet
    pub value: i32,
    /// The TON Gram amount that was staked; in the smallest units of the currency
    pub stake_gram_amount: i64,
    /// The TON Gram amount that was gained from the roll; in the smallest units of the currency; -1 if the dice don't have final state yet
    pub prize_gram_amount: i64,
}

/// A custom emoji. The text behind a custom emoji must be an emoji. Only premium users can use premium custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeCustomEmoji {
    /// Unique identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// A sticker to be sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputSticker {
    /// Sticker to be sent
    pub sticker: crate::enums::InputFile,
    /// Sticker thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// Sticker width
    pub width: i32,
    /// Sticker height
    pub height: i32,
}

/// A sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPollMediaSticker {
    /// Sticker to be sent
    pub sticker: crate::types::InputSticker,
}

/// A sticker message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageSticker {
    /// Sticker to be sent
    pub sticker: crate::types::InputSticker,
    /// Emoji used to choose the sticker
    pub emoji: String,
}

/// A dice message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageDice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Pass true to delete message draft in the chat
    pub clear_draft: bool,
}

/// A stake dice message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageStakeDice {
    /// Hash of the stake dice state. The state hash can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
    pub state_hash: String,
    /// The TON Gram amount that will be staked; in the smallest units of the currency. Must be in the range
    /// getOption("stake_dice_stake_amount_min")-getOption("stake_dice_stake_amount_max")
    pub stake_gram_amount: i64,
    /// Pass true to delete message draft in the chat
    pub clear_draft: bool,
}

/// The user is picking a sticker to send
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionChoosingSticker {
}

/// Represents an emoji with its keyword
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiKeyword {
    /// The emoji
    pub emoji: String,
    /// The keyword
    pub keyword: String,
}

/// Represents a list of emojis with their keywords
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiKeywords {
    /// List of emojis with their keywords
    pub emoji_keywords: Vec<crate::types::EmojiKeyword>,
}

/// Represents a list of stickers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Stickers {
    /// List of stickers
    pub stickers: Vec<crate::types::Sticker>,
}

/// Represents a list of emojis
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Emojis {
    /// List of emojis
    pub emojis: Vec<String>,
}

/// Represents a sticker set
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StickerSet {
    /// Identifier of the sticker set
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Title of the sticker set
    pub title: String,
    /// Name of the sticker set
    pub name: String,
    /// Sticker set thumbnail in WEBP, TGS, or WEBM format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Sticker set thumbnail's outline; may be null if unknown
    pub thumbnail_outline: Option<crate::types::Outline>,
    /// True, if the sticker set is owned by the current user
    pub is_owned: bool,
    /// True, if the sticker set has been installed by the current user
    pub is_installed: bool,
    /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
    pub is_archived: bool,
    /// True, if the sticker set is official
    pub is_official: bool,
    /// Type of the stickers in the set
    pub sticker_type: crate::enums::StickerType,
    /// True, if stickers in the sticker set are custom emoji that must be repainted; for custom emoji sticker sets only
    pub needs_repainting: bool,
    /// True, if stickers in the sticker set are custom emoji that can be used as chat emoji status; for custom emoji sticker sets only
    pub is_allowed_as_chat_emoji_status: bool,
    /// True for already viewed trending sticker sets
    pub is_viewed: bool,
    /// List of stickers in this set
    pub stickers: Vec<crate::types::Sticker>,
    /// A list of emojis corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    pub emojis: Vec<crate::types::Emojis>,
}

/// Represents short information about a sticker set
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StickerSetInfo {
    /// Identifier of the sticker set
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Title of the sticker set
    pub title: String,
    /// Name of the sticker set
    pub name: String,
    /// Sticker set thumbnail in WEBP, TGS, or WEBM format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Sticker set thumbnail's outline; may be null if unknown
    pub thumbnail_outline: Option<crate::types::Outline>,
    /// True, if the sticker set is owned by the current user
    pub is_owned: bool,
    /// True, if the sticker set has been installed by the current user
    pub is_installed: bool,
    /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
    pub is_archived: bool,
    /// True, if the sticker set is official
    pub is_official: bool,
    /// Type of the stickers in the set
    pub sticker_type: crate::enums::StickerType,
    /// True, if stickers in the sticker set are custom emoji that must be repainted; for custom emoji sticker sets only
    pub needs_repainting: bool,
    /// True, if stickers in the sticker set are custom emoji that can be used as chat emoji status; for custom emoji sticker sets only
    pub is_allowed_as_chat_emoji_status: bool,
    /// True for already viewed trending sticker sets
    pub is_viewed: bool,
    /// Total number of stickers in the set
    pub size: i32,
    /// Up to the first 5 stickers from the set, depending on the context. If the application needs more stickers the full sticker set needs to be requested
    pub covers: Vec<crate::types::Sticker>,
}

/// Represents a list of sticker sets
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerSets {
    /// Approximate total number of sticker sets found
    pub total_count: i32,
    /// List of sticker sets
    pub sets: Vec<crate::types::StickerSetInfo>,
}

/// Represents a list of trending sticker sets
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TrendingStickerSets {
    /// Approximate total number of trending sticker sets
    pub total_count: i32,
    /// List of trending sticker sets
    pub sets: Vec<crate::types::StickerSetInfo>,
    /// True, if the list contains sticker sets with premium stickers
    pub is_premium: bool,
}

/// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers,
/// or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategorySourceSearch {
    /// List of emojis to search for
    pub emojis: Vec<String>,
}

/// The category contains premium stickers that must be found by getPremiumStickers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategorySourcePremium {
}

/// Describes an emoji category
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategory {
    /// Name of the category
    pub name: String,
    /// Custom emoji sticker, which represents icon of the category
    pub icon: crate::types::Sticker,
    /// Source of stickers for the emoji category
    pub source: crate::enums::EmojiCategorySource,
    /// True, if the category must be shown first when choosing a sticker for the start page
    pub is_greeting: bool,
}

/// Represents a list of emoji categories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategories {
    /// List of categories
    pub categories: Vec<crate::types::EmojiCategory>,
}

/// The category must be used by default (e.g., for custom emoji or animation search)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategoryTypeDefault {
}

/// The category must be used by default for regular sticker selection. It may contain greeting emoji category and premium stickers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategoryTypeRegularStickers {
}

/// The category must be used for emoji status selection
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategoryTypeEmojiStatus {
}

/// The category must be used for chat photo emoji selection
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategoryTypeChatPhoto {
}

/// Contains information about an emoji reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiReaction {
    /// Text representation of the reaction
    pub emoji: String,
    /// Reaction title
    pub title: String,
    /// True, if the reaction can be added to new messages and enabled in chats
    pub is_active: bool,
    /// Static icon for the reaction
    pub static_icon: crate::types::Sticker,
    /// Appear animation for the reaction
    pub appear_animation: crate::types::Sticker,
    /// Select animation for the reaction
    pub select_animation: crate::types::Sticker,
    /// Activate animation for the reaction
    pub activate_animation: crate::types::Sticker,
    /// Effect animation for the reaction
    pub effect_animation: crate::types::Sticker,
    /// Around animation for the reaction; may be null
    pub around_animation: Option<crate::types::Sticker>,
    /// Center animation for the reaction; may be null
    pub center_animation: Option<crate::types::Sticker>,
}

/// A regular animated sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiceStickersRegular {
    /// The animated sticker with the dice animation
    pub sticker: crate::types::Sticker,
}

/// Animated stickers to be combined into a slot machine
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiceStickersSlotMachine {
    /// The animated sticker with the slot machine background. The background animation must start playing after all reel animations finish
    pub background: crate::types::Sticker,
    /// The animated sticker with the lever animation. The lever animation must play once in the initial dice state
    pub lever: crate::types::Sticker,
    /// The animated sticker with the left reel
    pub left_reel: crate::types::Sticker,
    /// The animated sticker with the center reel
    pub center_reel: crate::types::Sticker,
    /// The animated sticker with the right reel
    pub right_reel: crate::types::Sticker,
}

/// Represents a link to a WEBP, TGS, or WEBM sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultSticker {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the sticker thumbnail, if it exists
    pub thumbnail_url: String,
    /// The URL of the WEBP, TGS, or WEBM sticker (sticker file size must not exceed 5MB)
    pub sticker_url: String,
    /// Width of the sticker
    pub sticker_width: i32,
    /// Height of the sticker
    pub sticker_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageSticker, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultSticker {
    /// Unique identifier of the query result
    pub id: String,
    /// Sticker
    pub sticker: crate::types::Sticker,
}

/// The chat emoji status was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventEmojiStatusChanged {
    /// Previous emoji status; may be null if none
    pub old_emoji_status: Option<crate::types::EmojiStatus>,
    /// New emoji status; may be null if none
    pub new_emoji_status: Option<crate::types::EmojiStatus>,
}

/// The supergroup sticker set was changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventStickerSetChanged {
    /// Previous identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_sticker_set_id: i64,
    /// New identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_sticker_set_id: i64,
}

/// The supergroup sticker set with allowed custom emoji was changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventCustomEmojiStickerSetChanged {
    /// Previous identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_sticker_set_id: i64,
    /// New identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_sticker_set_id: i64,
}

/// The maximum number of favorite stickers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeFavoriteStickerCount {
}

/// Allowed to use premium stickers with unique effects
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureUniqueStickers {
}

/// Allowed to use custom emoji stickers in message texts and captions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureCustomEmoji {
}

/// The ability to show an emoji status along with the user's name
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureEmojiStatus {
}

/// Profile photo animation on message and chat screens
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureAnimatedProfilePhoto {
}

/// The ability to show an emoji status along with the business name
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureEmojiStatus {
}

/// Describes a chat theme based on an emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiChatTheme {
    /// Theme name
    pub name: String,
    /// Theme settings for a light chat theme
    pub light_settings: crate::types::ThemeSettings,
    /// Theme settings for a dark chat theme
    pub dark_settings: crate::types::ThemeSettings,
}

/// A chat theme based on an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatThemeEmoji {
    /// Name of the theme; full theme description is received through updateEmojiChatThemes
    pub name: String,
}

/// A theme based on an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChatThemeEmoji {
    /// Name of the theme
    pub name: String,
}

/// The name can be set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckStickerSetNameResultOk {
}

/// The name is invalid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckStickerSetNameResultNameInvalid {
}

/// The name is occupied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckStickerSetNameResultNameOccupied {
}

/// A message with a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentSticker {
    /// Message content; may be null
    pub sticker: Option<crate::types::Sticker>,
    /// Emoji corresponding to the sticker; may be empty
    pub emoji: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set.
/// If the sticker set is found and the user wants to add it, then call changeStickerSet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStickerSet {
    /// Name of the sticker set
    pub sticker_set_name: String,
    /// True, if the sticker set is expected to contain custom emoji
    pub expect_custom_emoji: bool,
}

/// The file is a sticker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FileTypeSticker {
}

/// A URL linking to a sticker set
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeStickerSet {
    /// Identifier of the sticker set
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
}

/// A sticker to be added to a sticker set
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewSticker {
    /// File with the sticker; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side.
    /// See https:core.telegram.org/animated_stickers#technical-requirements for technical requirements
    pub sticker: crate::enums::InputFile,
    /// Format of the sticker
    pub format: crate::enums::StickerFormat,
    /// String with 1-20 emoji corresponding to the sticker
    pub emojis: String,
    /// Position where the mask is placed; pass null if not specified
    pub mask_position: Option<crate::types::MaskPosition>,
    /// List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker
    pub keywords: Vec<String>,
}

/// Chat emoji status has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatEmojiStatus {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat emoji status; may be null
    pub emoji_status: Option<crate::types::EmojiStatus>,
}

/// A sticker set has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStickerSet {
    /// The sticker set
    pub sticker_set: crate::types::StickerSet,
}

/// The list of installed sticker sets was updated
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateInstalledStickerSets {
    /// Type of the affected stickers
    pub sticker_type: crate::enums::StickerType,
    /// The new list of installed ordinary sticker sets
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub sticker_set_ids: Vec<i64>,
}

/// The list of trending sticker sets was updated or some of them were viewed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateTrendingStickerSets {
    /// Type of the affected stickers
    pub sticker_type: crate::enums::StickerType,
    /// The prefix of the list of trending sticker sets with the newest trending sticker sets
    pub sticker_sets: crate::types::TrendingStickerSets,
}

/// The list of recently used stickers was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateRecentStickers {
    /// True, if the list of stickers attached to photo or video files was updated; otherwise, the list of sent stickers is updated
    pub is_attached: bool,
    /// The new list of file identifiers of recently used stickers
    pub sticker_ids: Vec<i32>,
}

/// The list of favorite stickers was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFavoriteStickers {
    /// The new list of file identifiers of favorite stickers
    pub sticker_ids: Vec<i32>,
}

/// The list of available emoji chat themes has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateEmojiChatThemes {
    /// The new list of emoji chat themes
    pub chat_themes: Vec<crate::types::EmojiChatTheme>,
}

/// The list of active emoji reactions has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveEmojiReactions {
    /// The new list of active emoji reactions
    pub emojis: Vec<String>,
}

/// The list of supported dice emojis has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateDiceEmojis {
    /// The new list of supported dice emojis
    pub emojis: Vec<String>,
}

/// The stake dice state has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStakeDiceState {
    /// The new state. The state can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
    pub state: crate::types::StakeDiceState,
}

/// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAnimatedEmojiMessageClicked {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The animated sticker to be played
    pub sticker: crate::types::Sticker,
}

