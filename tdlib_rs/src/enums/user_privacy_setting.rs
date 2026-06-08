#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySetting {
    /// A privacy setting for managing whether the user's online status is visible
    #[serde(rename(
        serialize = "userPrivacySettingShowStatus",
        deserialize = "userPrivacySettingShowStatus"
    ))]
    ShowStatus,
    /// A privacy setting for managing whether the user's profile photo is visible
    #[serde(rename(
        serialize = "userPrivacySettingShowProfilePhoto",
        deserialize = "userPrivacySettingShowProfilePhoto"
    ))]
    ShowProfilePhoto,
    /// A privacy setting for managing whether a link to the user's account is included in forwarded messages
    #[serde(rename(
        serialize = "userPrivacySettingShowLinkInForwardedMessages",
        deserialize = "userPrivacySettingShowLinkInForwardedMessages"
    ))]
    ShowLinkInForwardedMessages,
    /// A privacy setting for managing whether the user's phone number is visible
    #[serde(rename(
        serialize = "userPrivacySettingShowPhoneNumber",
        deserialize = "userPrivacySettingShowPhoneNumber"
    ))]
    ShowPhoneNumber,
    /// A privacy setting for managing whether the user's bio is visible
    #[serde(rename(
        serialize = "userPrivacySettingShowBio",
        deserialize = "userPrivacySettingShowBio"
    ))]
    ShowBio,
    /// A privacy setting for managing whether the user's birthdate is visible
    #[serde(rename(
        serialize = "userPrivacySettingShowBirthdate",
        deserialize = "userPrivacySettingShowBirthdate"
    ))]
    ShowBirthdate,
    /// A privacy setting for managing whether the user's profile audio files are visible
    #[serde(rename(
        serialize = "userPrivacySettingShowProfileAudio",
        deserialize = "userPrivacySettingShowProfileAudio"
    ))]
    ShowProfileAudio,
    /// A privacy setting for managing whether the user can be invited to chats
    #[serde(rename(
        serialize = "userPrivacySettingAllowChatInvites",
        deserialize = "userPrivacySettingAllowChatInvites"
    ))]
    AllowChatInvites,
    /// A privacy setting for managing whether the user can be called
    #[serde(rename(
        serialize = "userPrivacySettingAllowCalls",
        deserialize = "userPrivacySettingAllowCalls"
    ))]
    AllowCalls,
    /// A privacy setting for managing whether peer-to-peer connections can be used for calls
    #[serde(rename(
        serialize = "userPrivacySettingAllowPeerToPeerCalls",
        deserialize = "userPrivacySettingAllowPeerToPeerCalls"
    ))]
    AllowPeerToPeerCalls,
    /// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
    #[serde(rename(
        serialize = "userPrivacySettingAllowFindingByPhoneNumber",
        deserialize = "userPrivacySettingAllowFindingByPhoneNumber"
    ))]
    AllowFindingByPhoneNumber,
    /// A privacy setting for managing whether the user can receive voice and video messages in private chats; for Telegram Premium users only
    #[serde(rename(
        serialize = "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages",
        deserialize = "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages"
    ))]
    AllowPrivateVoiceAndVideoNoteMessages,
    /// A privacy setting for managing whether received gifts are automatically shown on the user's profile page
    #[serde(rename(
        serialize = "userPrivacySettingAutosaveGifts",
        deserialize = "userPrivacySettingAutosaveGifts"
    ))]
    AutosaveGifts,
    /// A privacy setting for managing whether the user can receive messages without additional payment
    #[serde(rename(
        serialize = "userPrivacySettingAllowUnpaidMessages",
        deserialize = "userPrivacySettingAllowUnpaidMessages"
    ))]
    AllowUnpaidMessages,
}
