#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedAction {
    /// Suggests the user to enable archive_and_mute_new_chats_from_unknown_users setting in archiveChatListSettings
    #[serde(rename(serialize = "suggestedActionEnableArchiveAndMuteNewChats", deserialize = "suggestedActionEnableArchiveAndMuteNewChats"))]
    EnableArchiveAndMuteNewChats,
    /// Suggests the user to check whether they still remember their 2-step verification password
    #[serde(rename(serialize = "suggestedActionCheckPassword", deserialize = "suggestedActionCheckPassword"))]
    CheckPassword,
    /// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
    #[serde(rename(serialize = "suggestedActionCheckPhoneNumber", deserialize = "suggestedActionCheckPhoneNumber"))]
    CheckPhoneNumber,
    /// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
    #[serde(rename(serialize = "suggestedActionViewChecksHint", deserialize = "suggestedActionViewChecksHint"))]
    ViewChecksHint,
    /// Suggests the user to convert specified supergroup to a broadcast group
    #[serde(rename(serialize = "suggestedActionConvertToBroadcastGroup", deserialize = "suggestedActionConvertToBroadcastGroup"))]
    ConvertToBroadcastGroup(crate::types::SuggestedActionConvertToBroadcastGroup),
    /// Suggests the user to set a 2-step verification password to be able to log in again
    #[serde(rename(serialize = "suggestedActionSetPassword", deserialize = "suggestedActionSetPassword"))]
    SetPassword(crate::types::SuggestedActionSetPassword),
    /// Suggests the user to upgrade the Premium subscription from monthly payments to annual payments
    #[serde(rename(serialize = "suggestedActionUpgradePremium", deserialize = "suggestedActionUpgradePremium"))]
    UpgradePremium,
    /// Suggests the user to restore a recently expired Premium subscription
    #[serde(rename(serialize = "suggestedActionRestorePremium", deserialize = "suggestedActionRestorePremium"))]
    RestorePremium,
    /// Suggests the user to subscribe to the Premium subscription with annual payments
    #[serde(rename(serialize = "suggestedActionSubscribeToAnnualPremium", deserialize = "suggestedActionSubscribeToAnnualPremium"))]
    SubscribeToAnnualPremium,
    /// Suggests the user to gift Telegram Premium to friends for Christmas
    #[serde(rename(serialize = "suggestedActionGiftPremiumForChristmas", deserialize = "suggestedActionGiftPremiumForChristmas"))]
    GiftPremiumForChristmas,
    /// Suggests the user to set birthdate
    #[serde(rename(serialize = "suggestedActionSetBirthdate", deserialize = "suggestedActionSetBirthdate"))]
    SetBirthdate,
    /// Suggests the user to set profile photo
    #[serde(rename(serialize = "suggestedActionSetProfilePhoto", deserialize = "suggestedActionSetProfilePhoto"))]
    SetProfilePhoto,
    /// Suggests the user to extend their expiring Telegram Premium subscription
    #[serde(rename(serialize = "suggestedActionExtendPremium", deserialize = "suggestedActionExtendPremium"))]
    ExtendPremium(crate::types::SuggestedActionExtendPremium),
    /// Suggests the user to extend their expiring Telegram Star subscriptions. Call getStarSubscriptions with only_expiring == true
    /// to get the number of expiring subscriptions and the number of required to buy Telegram Stars
    #[serde(rename(serialize = "suggestedActionExtendStarSubscriptions", deserialize = "suggestedActionExtendStarSubscriptions"))]
    ExtendStarSubscriptions,
    /// A custom suggestion to be shown at the top of the chat list
    #[serde(rename(serialize = "suggestedActionCustom", deserialize = "suggestedActionCustom"))]
    Custom(crate::types::SuggestedActionCustom),
    /// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
    #[serde(rename(serialize = "suggestedActionSetLoginEmailAddress", deserialize = "suggestedActionSetLoginEmailAddress"))]
    SetLoginEmailAddress(crate::types::SuggestedActionSetLoginEmailAddress),
    /// Suggests the user to add a passkey for login using addLoginPasskey
    #[serde(rename(serialize = "suggestedActionAddLoginPasskey", deserialize = "suggestedActionAddLoginPasskey"))]
    AddLoginPasskey,
}
