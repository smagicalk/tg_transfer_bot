#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGift {
    /// Unique identifier of the gift
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Unique identifier of the regular gift from which the gift was upgraded; may be 0 for short period of time for old gifts from database
    #[serde_as(as = "DisplayFromStr")]
    pub regular_gift_id: i64,
    /// Identifier of the chat that published the gift; 0 if none
    pub publisher_chat_id: i64,
    /// The title of the upgraded gift
    pub title: String,
    /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift or sendResoldGift
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same gift
    pub number: i32,
    /// Total number of gifts that were upgraded from the same gift
    pub total_upgraded_count: i32,
    /// The maximum number of gifts that can be upgraded from the same gift
    pub max_upgraded_count: i32,
    /// True, if the gift was used to craft another gift
    pub is_burned: bool,
    /// True, if the gift was craft from another gifts
    pub is_crafted: bool,
    /// True, if the original gift could have been bought only by Telegram Premium subscribers
    pub is_premium: bool,
    /// True, if the gift can be used to set a theme in a chat
    pub is_theme_available: bool,
    /// Identifier of the chat for which the gift is used to set a theme; 0 if none or the gift isn't owned by the current user
    pub used_theme_chat_id: i64,
    /// Identifier of the user or the chat to which the upgraded gift was assigned from blockchain; may be null if none or unknown
    pub host_id: Option<crate::enums::MessageSender>,
    /// Identifier of the user or the chat that owns the upgraded gift; may be null if none or unknown
    pub owner_id: Option<crate::enums::MessageSender>,
    /// Address of the gift NFT owner in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
    pub owner_address: String,
    /// Name of the owner for the case when owner identifier and address aren't known
    pub owner_name: String,
    /// Address of the gift NFT in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
    pub gift_address: String,
    /// Model of the upgraded gift
    pub model: crate::types::UpgradedGiftModel,
    /// Symbol of the upgraded gift
    pub symbol: crate::types::UpgradedGiftSymbol,
    /// Backdrop of the upgraded gift
    pub backdrop: crate::types::UpgradedGiftBackdrop,
    /// Information about the originally sent gift; may be null if unknown
    pub original_details: Option<crate::types::UpgradedGiftOriginalDetails>,
    /// Colors that can be set for user's name, background of empty chat photo, replies to messages and link previews; may be null if none or unknown
    pub colors: Option<crate::types::UpgradedGiftColors>,
    /// Resale parameters of the gift; may be null if resale isn't possible
    pub resale_parameters: Option<crate::types::GiftResaleParameters>,
    /// True, if an offer to purchase the gift can be sent using sendGiftPurchaseOffer
    pub can_send_purchase_offer: bool,
    /// Probability that the gift adds to the chance of successful crafting of a new gift; 0 if the gift can't be used for crafting
    pub craft_probability_per_mille: i32,
    /// ISO 4217 currency code of the currency in which value of the gift is represented; may be empty if unavailable
    pub value_currency: String,
    /// Estimated value of the gift; in the smallest units of the currency; 0 if unavailable
    pub value_amount: i64,
    /// Estimated value of the gift in USD; in USD cents; 0 if unavailable
    pub value_usd_amount: i64,
}
