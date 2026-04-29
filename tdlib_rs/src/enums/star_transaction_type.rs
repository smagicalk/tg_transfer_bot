#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransactionType {
    /// The transaction is a deposit of Telegram Stars from the Premium bot; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePremiumBotDeposit", deserialize = "starTransactionTypePremiumBotDeposit"))]
    PremiumBotDeposit,
    /// The transaction is a deposit of Telegram Stars from App Store; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeAppStoreDeposit", deserialize = "starTransactionTypeAppStoreDeposit"))]
    AppStoreDeposit,
    /// The transaction is a deposit of Telegram Stars from Google Play; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGooglePlayDeposit", deserialize = "starTransactionTypeGooglePlayDeposit"))]
    GooglePlayDeposit,
    /// The transaction is a deposit of Telegram Stars from Fragment; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypeFragmentDeposit", deserialize = "starTransactionTypeFragmentDeposit"))]
    FragmentDeposit,
    /// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUserDeposit", deserialize = "starTransactionTypeUserDeposit"))]
    UserDeposit(crate::types::StarTransactionTypeUserDeposit),
    /// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiveawayDeposit", deserialize = "starTransactionTypeGiveawayDeposit"))]
    GiveawayDeposit(crate::types::StarTransactionTypeGiveawayDeposit),
    /// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
    #[serde(rename(serialize = "starTransactionTypeFragmentWithdrawal", deserialize = "starTransactionTypeFragmentWithdrawal"))]
    FragmentWithdrawal(crate::types::StarTransactionTypeFragmentWithdrawal),
    /// The transaction is a withdrawal of earned Telegram Stars to Telegram Ad platform; relevant for bots and channel chats only
    #[serde(rename(serialize = "starTransactionTypeTelegramAdsWithdrawal", deserialize = "starTransactionTypeTelegramAdsWithdrawal"))]
    TelegramAdsWithdrawal,
    /// The transaction is a payment for Telegram API usage; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeTelegramApiUsage", deserialize = "starTransactionTypeTelegramApiUsage"))]
    TelegramApiUsage(crate::types::StarTransactionTypeTelegramApiUsage),
    /// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotPaidMediaPurchase", deserialize = "starTransactionTypeBotPaidMediaPurchase"))]
    BotPaidMediaPurchase(crate::types::StarTransactionTypeBotPaidMediaPurchase),
    /// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotPaidMediaSale", deserialize = "starTransactionTypeBotPaidMediaSale"))]
    BotPaidMediaSale(crate::types::StarTransactionTypeBotPaidMediaSale),
    /// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidMediaPurchase", deserialize = "starTransactionTypeChannelPaidMediaPurchase"))]
    ChannelPaidMediaPurchase(crate::types::StarTransactionTypeChannelPaidMediaPurchase),
    /// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidMediaSale", deserialize = "starTransactionTypeChannelPaidMediaSale"))]
    ChannelPaidMediaSale(crate::types::StarTransactionTypeChannelPaidMediaSale),
    /// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotInvoicePurchase", deserialize = "starTransactionTypeBotInvoicePurchase"))]
    BotInvoicePurchase(crate::types::StarTransactionTypeBotInvoicePurchase),
    /// The transaction is a sale of a product by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotInvoiceSale", deserialize = "starTransactionTypeBotInvoiceSale"))]
    BotInvoiceSale(crate::types::StarTransactionTypeBotInvoiceSale),
    /// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotSubscriptionPurchase", deserialize = "starTransactionTypeBotSubscriptionPurchase"))]
    BotSubscriptionPurchase(crate::types::StarTransactionTypeBotSubscriptionPurchase),
    /// The transaction is a sale of a subscription by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotSubscriptionSale", deserialize = "starTransactionTypeBotSubscriptionSale"))]
    BotSubscriptionSale(crate::types::StarTransactionTypeBotSubscriptionSale),
    /// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelSubscriptionPurchase", deserialize = "starTransactionTypeChannelSubscriptionPurchase"))]
    ChannelSubscriptionPurchase(crate::types::StarTransactionTypeChannelSubscriptionPurchase),
    /// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelSubscriptionSale", deserialize = "starTransactionTypeChannelSubscriptionSale"))]
    ChannelSubscriptionSale(crate::types::StarTransactionTypeChannelSubscriptionSale),
    /// The transaction is a bid on a gift auction; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftAuctionBid", deserialize = "starTransactionTypeGiftAuctionBid"))]
    GiftAuctionBid(crate::types::StarTransactionTypeGiftAuctionBid),
    /// The transaction is a purchase of a regular gift; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypeGiftPurchase", deserialize = "starTransactionTypeGiftPurchase"))]
    GiftPurchase(crate::types::StarTransactionTypeGiftPurchase),
    /// The transaction is an offer of gift purchase; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftPurchaseOffer", deserialize = "starTransactionTypeGiftPurchaseOffer"))]
    GiftPurchaseOffer(crate::types::StarTransactionTypeGiftPurchaseOffer),
    /// The transaction is a transfer of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftTransfer", deserialize = "starTransactionTypeGiftTransfer"))]
    GiftTransfer(crate::types::StarTransactionTypeGiftTransfer),
    /// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftOriginalDetailsDrop", deserialize = "starTransactionTypeGiftOriginalDetailsDrop"))]
    GiftOriginalDetailsDrop(crate::types::StarTransactionTypeGiftOriginalDetailsDrop),
    /// The transaction is a sale of a received gift; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypeGiftSale", deserialize = "starTransactionTypeGiftSale"))]
    GiftSale(crate::types::StarTransactionTypeGiftSale),
    /// The transaction is an upgrade of a gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftUpgrade", deserialize = "starTransactionTypeGiftUpgrade"))]
    GiftUpgrade(crate::types::StarTransactionTypeGiftUpgrade),
    /// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftUpgradePurchase", deserialize = "starTransactionTypeGiftUpgradePurchase"))]
    GiftUpgradePurchase(crate::types::StarTransactionTypeGiftUpgradePurchase),
    /// The transaction is a purchase of an upgraded gift for some user or channel; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUpgradedGiftPurchase", deserialize = "starTransactionTypeUpgradedGiftPurchase"))]
    UpgradedGiftPurchase(crate::types::StarTransactionTypeUpgradedGiftPurchase),
    /// The transaction is a sale of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUpgradedGiftSale", deserialize = "starTransactionTypeUpgradedGiftSale"))]
    UpgradedGiftSale(crate::types::StarTransactionTypeUpgradedGiftSale),
    /// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidReactionSend", deserialize = "starTransactionTypeChannelPaidReactionSend"))]
    ChannelPaidReactionSend(crate::types::StarTransactionTypeChannelPaidReactionSend),
    /// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidReactionReceive", deserialize = "starTransactionTypeChannelPaidReactionReceive"))]
    ChannelPaidReactionReceive(crate::types::StarTransactionTypeChannelPaidReactionReceive),
    /// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
    #[serde(rename(serialize = "starTransactionTypeAffiliateProgramCommission", deserialize = "starTransactionTypeAffiliateProgramCommission"))]
    AffiliateProgramCommission(crate::types::StarTransactionTypeAffiliateProgramCommission),
    /// The transaction is a sending of a paid message; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidMessageSend", deserialize = "starTransactionTypePaidMessageSend"))]
    PaidMessageSend(crate::types::StarTransactionTypePaidMessageSend),
    /// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidMessageReceive", deserialize = "starTransactionTypePaidMessageReceive"))]
    PaidMessageReceive(crate::types::StarTransactionTypePaidMessageReceive),
    /// The transaction is a sending of a paid group call message; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallMessageSend", deserialize = "starTransactionTypePaidGroupCallMessageSend"))]
    PaidGroupCallMessageSend(crate::types::StarTransactionTypePaidGroupCallMessageSend),
    /// The transaction is a receiving of a paid group call message; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallMessageReceive", deserialize = "starTransactionTypePaidGroupCallMessageReceive"))]
    PaidGroupCallMessageReceive(crate::types::StarTransactionTypePaidGroupCallMessageReceive),
    /// The transaction is a sending of a paid group reaction; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallReactionSend", deserialize = "starTransactionTypePaidGroupCallReactionSend"))]
    PaidGroupCallReactionSend(crate::types::StarTransactionTypePaidGroupCallReactionSend),
    /// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallReactionReceive", deserialize = "starTransactionTypePaidGroupCallReactionReceive"))]
    PaidGroupCallReactionReceive(crate::types::StarTransactionTypePaidGroupCallReactionReceive),
    /// The transaction is a payment for a suggested post; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeSuggestedPostPaymentSend", deserialize = "starTransactionTypeSuggestedPostPaymentSend"))]
    SuggestedPostPaymentSend(crate::types::StarTransactionTypeSuggestedPostPaymentSend),
    /// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeSuggestedPostPaymentReceive", deserialize = "starTransactionTypeSuggestedPostPaymentReceive"))]
    SuggestedPostPaymentReceive(crate::types::StarTransactionTypeSuggestedPostPaymentReceive),
    /// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypePremiumPurchase", deserialize = "starTransactionTypePremiumPurchase"))]
    PremiumPurchase(crate::types::StarTransactionTypePremiumPurchase),
    /// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBusinessBotTransferSend", deserialize = "starTransactionTypeBusinessBotTransferSend"))]
    BusinessBotTransferSend(crate::types::StarTransactionTypeBusinessBotTransferSend),
    /// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBusinessBotTransferReceive", deserialize = "starTransactionTypeBusinessBotTransferReceive"))]
    BusinessBotTransferReceive(crate::types::StarTransactionTypeBusinessBotTransferReceive),
    /// The transaction is a payment for search of posts in public Telegram channels; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePublicPostSearch", deserialize = "starTransactionTypePublicPostSearch"))]
    PublicPostSearch,
    /// The transaction is a transaction of an unsupported type
    #[serde(rename(serialize = "starTransactionTypeUnsupported", deserialize = "starTransactionTypeUnsupported"))]
    Unsupported,
}
