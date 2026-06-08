#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SettingsSection {
    /// The appearance section
    #[serde(rename(
        serialize = "settingsSectionAppearance",
        deserialize = "settingsSectionAppearance"
    ))]
    Appearance(crate::types::SettingsSectionAppearance),
    /// The "Ask a question" section
    #[serde(rename(
        serialize = "settingsSectionAskQuestion",
        deserialize = "settingsSectionAskQuestion"
    ))]
    AskQuestion,
    /// The "Telegram Business" section
    #[serde(rename(
        serialize = "settingsSectionBusiness",
        deserialize = "settingsSectionBusiness"
    ))]
    Business(crate::types::SettingsSectionBusiness),
    /// The chat folder settings section
    #[serde(rename(
        serialize = "settingsSectionChatFolders",
        deserialize = "settingsSectionChatFolders"
    ))]
    ChatFolders(crate::types::SettingsSectionChatFolders),
    /// The data and storage settings section
    #[serde(rename(
        serialize = "settingsSectionDataAndStorage",
        deserialize = "settingsSectionDataAndStorage"
    ))]
    DataAndStorage(crate::types::SettingsSectionDataAndStorage),
    /// The Devices section
    #[serde(rename(
        serialize = "settingsSectionDevices",
        deserialize = "settingsSectionDevices"
    ))]
    Devices(crate::types::SettingsSectionDevices),
    /// The profile edit section
    #[serde(rename(
        serialize = "settingsSectionEditProfile",
        deserialize = "settingsSectionEditProfile"
    ))]
    EditProfile(crate::types::SettingsSectionEditProfile),
    /// The FAQ section
    #[serde(rename(serialize = "settingsSectionFaq", deserialize = "settingsSectionFaq"))]
    Faq,
    /// The "Telegram Features" section
    #[serde(rename(
        serialize = "settingsSectionFeatures",
        deserialize = "settingsSectionFeatures"
    ))]
    Features,
    /// The in-app browser settings section
    #[serde(rename(
        serialize = "settingsSectionInAppBrowser",
        deserialize = "settingsSectionInAppBrowser"
    ))]
    InAppBrowser(crate::types::SettingsSectionInAppBrowser),
    /// The application language section
    #[serde(rename(
        serialize = "settingsSectionLanguage",
        deserialize = "settingsSectionLanguage"
    ))]
    Language(crate::types::SettingsSectionLanguage),
    /// The Telegram Star balance and transaction section
    #[serde(rename(
        serialize = "settingsSectionMyStars",
        deserialize = "settingsSectionMyStars"
    ))]
    MyStars(crate::types::SettingsSectionMyStars),
    /// The Toncoin balance and transaction section
    #[serde(rename(
        serialize = "settingsSectionMyToncoins",
        deserialize = "settingsSectionMyToncoins"
    ))]
    MyToncoins,
    /// The notification settings section
    #[serde(rename(
        serialize = "settingsSectionNotifications",
        deserialize = "settingsSectionNotifications"
    ))]
    Notifications(crate::types::SettingsSectionNotifications),
    /// The power saving settings section
    #[serde(rename(
        serialize = "settingsSectionPowerSaving",
        deserialize = "settingsSectionPowerSaving"
    ))]
    PowerSaving(crate::types::SettingsSectionPowerSaving),
    /// The "Telegram Premium" section
    #[serde(rename(
        serialize = "settingsSectionPremium",
        deserialize = "settingsSectionPremium"
    ))]
    Premium,
    /// The privacy and security section
    #[serde(rename(
        serialize = "settingsSectionPrivacyAndSecurity",
        deserialize = "settingsSectionPrivacyAndSecurity"
    ))]
    PrivacyAndSecurity(crate::types::SettingsSectionPrivacyAndSecurity),
    /// The "Privacy Policy" section
    #[serde(rename(
        serialize = "settingsSectionPrivacyPolicy",
        deserialize = "settingsSectionPrivacyPolicy"
    ))]
    PrivacyPolicy,
    /// The current user's QR code section
    #[serde(rename(
        serialize = "settingsSectionQrCode",
        deserialize = "settingsSectionQrCode"
    ))]
    QrCode(crate::types::SettingsSectionQrCode),
    /// Search in Settings
    #[serde(rename(
        serialize = "settingsSectionSearch",
        deserialize = "settingsSectionSearch"
    ))]
    Search,
    /// The "Send a gift" section
    #[serde(rename(
        serialize = "settingsSectionSendGift",
        deserialize = "settingsSectionSendGift"
    ))]
    SendGift(crate::types::SettingsSectionSendGift),
}
