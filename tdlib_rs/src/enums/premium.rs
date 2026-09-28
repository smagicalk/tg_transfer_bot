//!
//! TDLib `premium` domain enums.
//!
//! Types, enums, and functions for Telegram Premium, chat boosts, giveaways, and business features.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes conditions for sending of away messages by a Telegram Business account
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessAwayMessageSchedule {
    /// Send away messages always
    #[serde(rename(serialize = "businessAwayMessageScheduleAlways", deserialize = "businessAwayMessageScheduleAlways"))]
    Always,
    /// Send away messages outside of the business opening hours
    #[serde(rename(serialize = "businessAwayMessageScheduleOutsideOfOpeningHours", deserialize = "businessAwayMessageScheduleOutsideOfOpeningHours"))]
    OutsideOfOpeningHours,
    /// Send away messages only in the specified time span
    #[serde(rename(serialize = "businessAwayMessageScheduleCustom", deserialize = "businessAwayMessageScheduleCustom"))]
    Custom(Box<crate::types::BusinessAwayMessageScheduleCustom>),
}

impl BusinessAwayMessageSchedule {
    /// Convenience constructor to create a [`BusinessAwayMessageSchedule::Custom`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom(val: crate::types::BusinessAwayMessageScheduleCustom) -> Self {
        Self::Custom(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessAwayMessageScheduleCustom`] into [`BusinessAwayMessageSchedule`].
impl From<crate::types::BusinessAwayMessageScheduleCustom> for BusinessAwayMessageSchedule {
    fn from(val: crate::types::BusinessAwayMessageScheduleCustom) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// TDLib `BusinessLocation` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessLocation {
    /// Represents a location of a business
    #[serde(rename(serialize = "businessLocation", deserialize = "businessLocation"))]
    BusinessLocation(Box<crate::types::BusinessLocation>),
}

impl BusinessLocation {
    /// Convenience constructor to create a [`BusinessLocation::BusinessLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_location(val: crate::types::BusinessLocation) -> Self {
        Self::BusinessLocation(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessLocation`] into [`BusinessLocation`].
impl From<crate::types::BusinessLocation> for BusinessLocation {
    fn from(val: crate::types::BusinessLocation) -> Self {
        Self::BusinessLocation(Box::new(val))
    }
}

/// TDLib `BusinessRecipients` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessRecipients {
    /// Describes private chats chosen for automatic interaction with a business
    #[serde(rename(serialize = "businessRecipients", deserialize = "businessRecipients"))]
    BusinessRecipients(Box<crate::types::BusinessRecipients>),
}

impl BusinessRecipients {
    /// Convenience constructor to create a [`BusinessRecipients::BusinessRecipients`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_recipients(val: crate::types::BusinessRecipients) -> Self {
        Self::BusinessRecipients(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessRecipients`] into [`BusinessRecipients`].
impl From<crate::types::BusinessRecipients> for BusinessRecipients {
    fn from(val: crate::types::BusinessRecipients) -> Self {
        Self::BusinessRecipients(Box::new(val))
    }
}

/// TDLib `BusinessAwayMessageSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessAwayMessageSettings {
    /// Describes settings for messages that are automatically sent by a Telegram Business account when it is away
    #[serde(rename(serialize = "businessAwayMessageSettings", deserialize = "businessAwayMessageSettings"))]
    BusinessAwayMessageSettings(Box<crate::types::BusinessAwayMessageSettings>),
}

impl BusinessAwayMessageSettings {
    /// Convenience constructor to create a [`BusinessAwayMessageSettings::BusinessAwayMessageSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_away_message_settings(val: crate::types::BusinessAwayMessageSettings) -> Self {
        Self::BusinessAwayMessageSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessAwayMessageSettings`] into [`BusinessAwayMessageSettings`].
impl From<crate::types::BusinessAwayMessageSettings> for BusinessAwayMessageSettings {
    fn from(val: crate::types::BusinessAwayMessageSettings) -> Self {
        Self::BusinessAwayMessageSettings(Box::new(val))
    }
}

/// TDLib `BusinessGreetingMessageSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessGreetingMessageSettings {
    /// Describes settings for greeting messages that are automatically sent by a Telegram Business account as response to incoming messages in an inactive private chat
    #[serde(rename(serialize = "businessGreetingMessageSettings", deserialize = "businessGreetingMessageSettings"))]
    BusinessGreetingMessageSettings(Box<crate::types::BusinessGreetingMessageSettings>),
}

impl BusinessGreetingMessageSettings {
    /// Convenience constructor to create a [`BusinessGreetingMessageSettings::BusinessGreetingMessageSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_greeting_message_settings(val: crate::types::BusinessGreetingMessageSettings) -> Self {
        Self::BusinessGreetingMessageSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessGreetingMessageSettings`] into [`BusinessGreetingMessageSettings`].
impl From<crate::types::BusinessGreetingMessageSettings> for BusinessGreetingMessageSettings {
    fn from(val: crate::types::BusinessGreetingMessageSettings) -> Self {
        Self::BusinessGreetingMessageSettings(Box::new(val))
    }
}

/// TDLib `BusinessBotRights` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessBotRights {
    /// Describes rights of a business bot
    #[serde(rename(serialize = "businessBotRights", deserialize = "businessBotRights"))]
    BusinessBotRights(Box<crate::types::BusinessBotRights>),
}

impl BusinessBotRights {
    /// Convenience constructor to create a [`BusinessBotRights::BusinessBotRights`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_bot_rights(val: crate::types::BusinessBotRights) -> Self {
        Self::BusinessBotRights(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessBotRights`] into [`BusinessBotRights`].
impl From<crate::types::BusinessBotRights> for BusinessBotRights {
    fn from(val: crate::types::BusinessBotRights) -> Self {
        Self::BusinessBotRights(Box::new(val))
    }
}

/// TDLib `BusinessConnectedBot` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessConnectedBot {
    /// Describes a business bot connected to an account
    #[serde(rename(serialize = "businessConnectedBot", deserialize = "businessConnectedBot"))]
    BusinessConnectedBot(Box<crate::types::BusinessConnectedBot>),
}

impl BusinessConnectedBot {
    /// Convenience constructor to create a [`BusinessConnectedBot::BusinessConnectedBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_connected_bot(val: crate::types::BusinessConnectedBot) -> Self {
        Self::BusinessConnectedBot(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessConnectedBot`] into [`BusinessConnectedBot`].
impl From<crate::types::BusinessConnectedBot> for BusinessConnectedBot {
    fn from(val: crate::types::BusinessConnectedBot) -> Self {
        Self::BusinessConnectedBot(Box::new(val))
    }
}

/// TDLib `BusinessConnectedBotInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessConnectedBotInfo {
    /// Describes a connection of a bot to an account
    #[serde(rename(serialize = "businessConnectedBotInfo", deserialize = "businessConnectedBotInfo"))]
    BusinessConnectedBotInfo(Box<crate::types::BusinessConnectedBotInfo>),
}

impl BusinessConnectedBotInfo {
    /// Convenience constructor to create a [`BusinessConnectedBotInfo::BusinessConnectedBotInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_connected_bot_info(val: crate::types::BusinessConnectedBotInfo) -> Self {
        Self::BusinessConnectedBotInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessConnectedBotInfo`] into [`BusinessConnectedBotInfo`].
impl From<crate::types::BusinessConnectedBotInfo> for BusinessConnectedBotInfo {
    fn from(val: crate::types::BusinessConnectedBotInfo) -> Self {
        Self::BusinessConnectedBotInfo(Box::new(val))
    }
}

/// TDLib `BusinessStartPage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessStartPage {
    /// Describes settings for a business account start page
    #[serde(rename(serialize = "businessStartPage", deserialize = "businessStartPage"))]
    BusinessStartPage(Box<crate::types::BusinessStartPage>),
}

impl BusinessStartPage {
    /// Convenience constructor to create a [`BusinessStartPage::BusinessStartPage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_start_page(val: crate::types::BusinessStartPage) -> Self {
        Self::BusinessStartPage(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessStartPage`] into [`BusinessStartPage`].
impl From<crate::types::BusinessStartPage> for BusinessStartPage {
    fn from(val: crate::types::BusinessStartPage) -> Self {
        Self::BusinessStartPage(Box::new(val))
    }
}

/// TDLib `InputBusinessStartPage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBusinessStartPage {
    /// Describes settings for a business account start page to set
    #[serde(rename(serialize = "inputBusinessStartPage", deserialize = "inputBusinessStartPage"))]
    InputBusinessStartPage(Box<crate::types::InputBusinessStartPage>),
}

impl InputBusinessStartPage {
    /// Convenience constructor to create a [`InputBusinessStartPage::InputBusinessStartPage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_business_start_page(val: crate::types::InputBusinessStartPage) -> Self {
        Self::InputBusinessStartPage(Box::new(val))
    }

}

/// Converts a [`crate::types::InputBusinessStartPage`] into [`InputBusinessStartPage`].
impl From<crate::types::InputBusinessStartPage> for InputBusinessStartPage {
    fn from(val: crate::types::InputBusinessStartPage) -> Self {
        Self::InputBusinessStartPage(Box::new(val))
    }
}

/// TDLib `BusinessOpeningHoursInterval` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessOpeningHoursInterval {
    /// Describes an interval of time when the business is open
    #[serde(rename(serialize = "businessOpeningHoursInterval", deserialize = "businessOpeningHoursInterval"))]
    BusinessOpeningHoursInterval(Box<crate::types::BusinessOpeningHoursInterval>),
}

impl BusinessOpeningHoursInterval {
    /// Convenience constructor to create a [`BusinessOpeningHoursInterval::BusinessOpeningHoursInterval`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_opening_hours_interval(val: crate::types::BusinessOpeningHoursInterval) -> Self {
        Self::BusinessOpeningHoursInterval(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessOpeningHoursInterval`] into [`BusinessOpeningHoursInterval`].
impl From<crate::types::BusinessOpeningHoursInterval> for BusinessOpeningHoursInterval {
    fn from(val: crate::types::BusinessOpeningHoursInterval) -> Self {
        Self::BusinessOpeningHoursInterval(Box::new(val))
    }
}

/// TDLib `BusinessOpeningHours` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessOpeningHours {
    /// Describes opening hours of a business
    #[serde(rename(serialize = "businessOpeningHours", deserialize = "businessOpeningHours"))]
    BusinessOpeningHours(Box<crate::types::BusinessOpeningHours>),
}

impl BusinessOpeningHours {
    /// Convenience constructor to create a [`BusinessOpeningHours::BusinessOpeningHours`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_opening_hours(val: crate::types::BusinessOpeningHours) -> Self {
        Self::BusinessOpeningHours(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessOpeningHours`] into [`BusinessOpeningHours`].
impl From<crate::types::BusinessOpeningHours> for BusinessOpeningHours {
    fn from(val: crate::types::BusinessOpeningHours) -> Self {
        Self::BusinessOpeningHours(Box::new(val))
    }
}

/// TDLib `BusinessInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessInfo {
    /// Contains information about a Telegram Business account
    #[serde(rename(serialize = "businessInfo", deserialize = "businessInfo"))]
    BusinessInfo(Box<crate::types::BusinessInfo>),
}

impl BusinessInfo {
    /// Convenience constructor to create a [`BusinessInfo::BusinessInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_info(val: crate::types::BusinessInfo) -> Self {
        Self::BusinessInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessInfo`] into [`BusinessInfo`].
impl From<crate::types::BusinessInfo> for BusinessInfo {
    fn from(val: crate::types::BusinessInfo) -> Self {
        Self::BusinessInfo(Box::new(val))
    }
}

/// TDLib `BusinessChatLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLink {
    /// Contains information about a business chat link
    #[serde(rename(serialize = "businessChatLink", deserialize = "businessChatLink"))]
    BusinessChatLink(Box<crate::types::BusinessChatLink>),
}

impl BusinessChatLink {
    /// Convenience constructor to create a [`BusinessChatLink::BusinessChatLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_chat_link(val: crate::types::BusinessChatLink) -> Self {
        Self::BusinessChatLink(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessChatLink`] into [`BusinessChatLink`].
impl From<crate::types::BusinessChatLink> for BusinessChatLink {
    fn from(val: crate::types::BusinessChatLink) -> Self {
        Self::BusinessChatLink(Box::new(val))
    }
}

/// TDLib `BusinessChatLinks` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLinks {
    /// Contains a list of business chat links created by the user
    #[serde(rename(serialize = "businessChatLinks", deserialize = "businessChatLinks"))]
    BusinessChatLinks(Box<crate::types::BusinessChatLinks>),
}

impl BusinessChatLinks {
    /// Convenience constructor to create a [`BusinessChatLinks::BusinessChatLinks`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_chat_links(val: crate::types::BusinessChatLinks) -> Self {
        Self::BusinessChatLinks(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessChatLinks`] into [`BusinessChatLinks`].
impl From<crate::types::BusinessChatLinks> for BusinessChatLinks {
    fn from(val: crate::types::BusinessChatLinks) -> Self {
        Self::BusinessChatLinks(Box::new(val))
    }
}

/// TDLib `InputBusinessChatLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBusinessChatLink {
    /// Describes a business chat link to create or edit
    #[serde(rename(serialize = "inputBusinessChatLink", deserialize = "inputBusinessChatLink"))]
    InputBusinessChatLink(Box<crate::types::InputBusinessChatLink>),
}

impl InputBusinessChatLink {
    /// Convenience constructor to create a [`InputBusinessChatLink::InputBusinessChatLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_business_chat_link(val: crate::types::InputBusinessChatLink) -> Self {
        Self::InputBusinessChatLink(Box::new(val))
    }

}

/// Converts a [`crate::types::InputBusinessChatLink`] into [`InputBusinessChatLink`].
impl From<crate::types::InputBusinessChatLink> for InputBusinessChatLink {
    fn from(val: crate::types::InputBusinessChatLink) -> Self {
        Self::InputBusinessChatLink(Box::new(val))
    }
}

/// TDLib `BusinessChatLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLinkInfo {
    /// Contains information about a business chat link
    #[serde(rename(serialize = "businessChatLinkInfo", deserialize = "businessChatLinkInfo"))]
    BusinessChatLinkInfo(Box<crate::types::BusinessChatLinkInfo>),
}

impl BusinessChatLinkInfo {
    /// Convenience constructor to create a [`BusinessChatLinkInfo::BusinessChatLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_chat_link_info(val: crate::types::BusinessChatLinkInfo) -> Self {
        Self::BusinessChatLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessChatLinkInfo`] into [`BusinessChatLinkInfo`].
impl From<crate::types::BusinessChatLinkInfo> for BusinessChatLinkInfo {
    fn from(val: crate::types::BusinessChatLinkInfo) -> Self {
        Self::BusinessChatLinkInfo(Box::new(val))
    }
}

/// Describes type of affiliate for an affiliate program
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateType {
    /// The affiliate is the current user
    #[serde(rename(serialize = "affiliateTypeCurrentUser", deserialize = "affiliateTypeCurrentUser"))]
    CurrentUser,
    /// The affiliate is a bot owned by the current user
    #[serde(rename(serialize = "affiliateTypeBot", deserialize = "affiliateTypeBot"))]
    Bot(Box<crate::types::AffiliateTypeBot>),
    /// The affiliate is a channel chat where the current user has can_post_messages administrator right
    #[serde(rename(serialize = "affiliateTypeChannel", deserialize = "affiliateTypeChannel"))]
    Channel(Box<crate::types::AffiliateTypeChannel>),
}

impl AffiliateType {
    /// Convenience constructor to create a [`AffiliateType::Bot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot(val: crate::types::AffiliateTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }

    /// Convenience constructor to create a [`AffiliateType::Channel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel(val: crate::types::AffiliateTypeChannel) -> Self {
        Self::Channel(Box::new(val))
    }

}

/// Converts a [`crate::types::AffiliateTypeBot`] into [`AffiliateType`].
impl From<crate::types::AffiliateTypeBot> for AffiliateType {
    fn from(val: crate::types::AffiliateTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }
}

/// Converts a [`crate::types::AffiliateTypeChannel`] into [`AffiliateType`].
impl From<crate::types::AffiliateTypeChannel> for AffiliateType {
    fn from(val: crate::types::AffiliateTypeChannel) -> Self {
        Self::Channel(Box::new(val))
    }
}

/// Describes the order of the found affiliate programs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramSortOrder {
    /// The affiliate programs must be sorted by the profitability
    #[serde(rename(serialize = "affiliateProgramSortOrderProfitability", deserialize = "affiliateProgramSortOrderProfitability"))]
    Profitability,
    /// The affiliate programs must be sorted by creation date
    #[serde(rename(serialize = "affiliateProgramSortOrderCreationDate", deserialize = "affiliateProgramSortOrderCreationDate"))]
    CreationDate,
    /// The affiliate programs must be sorted by the expected revenue
    #[serde(rename(serialize = "affiliateProgramSortOrderRevenue", deserialize = "affiliateProgramSortOrderRevenue"))]
    Revenue,
}

/// TDLib `AffiliateProgramParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramParameters {
    /// Describes parameters of an affiliate program
    #[serde(rename(serialize = "affiliateProgramParameters", deserialize = "affiliateProgramParameters"))]
    AffiliateProgramParameters(Box<crate::types::AffiliateProgramParameters>),
}

impl AffiliateProgramParameters {
    /// Convenience constructor to create a [`AffiliateProgramParameters::AffiliateProgramParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn affiliate_program_parameters(val: crate::types::AffiliateProgramParameters) -> Self {
        Self::AffiliateProgramParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::AffiliateProgramParameters`] into [`AffiliateProgramParameters`].
impl From<crate::types::AffiliateProgramParameters> for AffiliateProgramParameters {
    fn from(val: crate::types::AffiliateProgramParameters) -> Self {
        Self::AffiliateProgramParameters(Box::new(val))
    }
}

/// TDLib `AffiliateProgramInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramInfo {
    /// Contains information about an active affiliate program
    #[serde(rename(serialize = "affiliateProgramInfo", deserialize = "affiliateProgramInfo"))]
    AffiliateProgramInfo(Box<crate::types::AffiliateProgramInfo>),
}

impl AffiliateProgramInfo {
    /// Convenience constructor to create a [`AffiliateProgramInfo::AffiliateProgramInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn affiliate_program_info(val: crate::types::AffiliateProgramInfo) -> Self {
        Self::AffiliateProgramInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::AffiliateProgramInfo`] into [`AffiliateProgramInfo`].
impl From<crate::types::AffiliateProgramInfo> for AffiliateProgramInfo {
    fn from(val: crate::types::AffiliateProgramInfo) -> Self {
        Self::AffiliateProgramInfo(Box::new(val))
    }
}

/// TDLib `AffiliateInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateInfo {
    /// Contains information about an affiliate that received commission from a Telegram Star transaction
    #[serde(rename(serialize = "affiliateInfo", deserialize = "affiliateInfo"))]
    AffiliateInfo(Box<crate::types::AffiliateInfo>),
}

impl AffiliateInfo {
    /// Convenience constructor to create a [`AffiliateInfo::AffiliateInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn affiliate_info(val: crate::types::AffiliateInfo) -> Self {
        Self::AffiliateInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::AffiliateInfo`] into [`AffiliateInfo`].
impl From<crate::types::AffiliateInfo> for AffiliateInfo {
    fn from(val: crate::types::AffiliateInfo) -> Self {
        Self::AffiliateInfo(Box::new(val))
    }
}

/// TDLib `FoundAffiliateProgram` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundAffiliateProgram {
    /// Describes a found affiliate program
    #[serde(rename(serialize = "foundAffiliateProgram", deserialize = "foundAffiliateProgram"))]
    FoundAffiliateProgram(Box<crate::types::FoundAffiliateProgram>),
}

impl FoundAffiliateProgram {
    /// Convenience constructor to create a [`FoundAffiliateProgram::FoundAffiliateProgram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_affiliate_program(val: crate::types::FoundAffiliateProgram) -> Self {
        Self::FoundAffiliateProgram(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundAffiliateProgram`] into [`FoundAffiliateProgram`].
impl From<crate::types::FoundAffiliateProgram> for FoundAffiliateProgram {
    fn from(val: crate::types::FoundAffiliateProgram) -> Self {
        Self::FoundAffiliateProgram(Box::new(val))
    }
}

/// TDLib `FoundAffiliatePrograms` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundAffiliatePrograms {
    /// Represents a list of found affiliate programs
    #[serde(rename(serialize = "foundAffiliatePrograms", deserialize = "foundAffiliatePrograms"))]
    FoundAffiliatePrograms(Box<crate::types::FoundAffiliatePrograms>),
}

impl FoundAffiliatePrograms {
    /// Convenience constructor to create a [`FoundAffiliatePrograms::FoundAffiliatePrograms`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_affiliate_programs(val: crate::types::FoundAffiliatePrograms) -> Self {
        Self::FoundAffiliatePrograms(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundAffiliatePrograms`] into [`FoundAffiliatePrograms`].
impl From<crate::types::FoundAffiliatePrograms> for FoundAffiliatePrograms {
    fn from(val: crate::types::FoundAffiliatePrograms) -> Self {
        Self::FoundAffiliatePrograms(Box::new(val))
    }
}

/// TDLib `ConnectedAffiliateProgram` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedAffiliateProgram {
    /// Describes an affiliate program that was connected to an affiliate
    #[serde(rename(serialize = "connectedAffiliateProgram", deserialize = "connectedAffiliateProgram"))]
    ConnectedAffiliateProgram(Box<crate::types::ConnectedAffiliateProgram>),
}

impl ConnectedAffiliateProgram {
    /// Convenience constructor to create a [`ConnectedAffiliateProgram::ConnectedAffiliateProgram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_affiliate_program(val: crate::types::ConnectedAffiliateProgram) -> Self {
        Self::ConnectedAffiliateProgram(Box::new(val))
    }

}

/// Converts a [`crate::types::ConnectedAffiliateProgram`] into [`ConnectedAffiliateProgram`].
impl From<crate::types::ConnectedAffiliateProgram> for ConnectedAffiliateProgram {
    fn from(val: crate::types::ConnectedAffiliateProgram) -> Self {
        Self::ConnectedAffiliateProgram(Box::new(val))
    }
}

/// TDLib `ConnectedAffiliatePrograms` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedAffiliatePrograms {
    /// Represents a list of affiliate programs that were connected to an affiliate
    #[serde(rename(serialize = "connectedAffiliatePrograms", deserialize = "connectedAffiliatePrograms"))]
    ConnectedAffiliatePrograms(Box<crate::types::ConnectedAffiliatePrograms>),
}

impl ConnectedAffiliatePrograms {
    /// Convenience constructor to create a [`ConnectedAffiliatePrograms::ConnectedAffiliatePrograms`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_affiliate_programs(val: crate::types::ConnectedAffiliatePrograms) -> Self {
        Self::ConnectedAffiliatePrograms(Box::new(val))
    }

}

/// Converts a [`crate::types::ConnectedAffiliatePrograms`] into [`ConnectedAffiliatePrograms`].
impl From<crate::types::ConnectedAffiliatePrograms> for ConnectedAffiliatePrograms {
    fn from(val: crate::types::ConnectedAffiliatePrograms) -> Self {
        Self::ConnectedAffiliatePrograms(Box::new(val))
    }
}

/// TDLib `PremiumGiftCodeInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftCodeInfo {
    /// Contains information about a Telegram Premium gift code
    #[serde(rename(serialize = "premiumGiftCodeInfo", deserialize = "premiumGiftCodeInfo"))]
    PremiumGiftCodeInfo(Box<crate::types::PremiumGiftCodeInfo>),
}

impl PremiumGiftCodeInfo {
    /// Convenience constructor to create a [`PremiumGiftCodeInfo::PremiumGiftCodeInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_code_info(val: crate::types::PremiumGiftCodeInfo) -> Self {
        Self::PremiumGiftCodeInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumGiftCodeInfo`] into [`PremiumGiftCodeInfo`].
impl From<crate::types::PremiumGiftCodeInfo> for PremiumGiftCodeInfo {
    fn from(val: crate::types::PremiumGiftCodeInfo) -> Self {
        Self::PremiumGiftCodeInfo(Box::new(val))
    }
}

/// TDLib `StarGiveawayWinnerOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayWinnerOption {
    /// Describes an option for the number of winners of a Telegram Star giveaway
    #[serde(rename(serialize = "starGiveawayWinnerOption", deserialize = "starGiveawayWinnerOption"))]
    StarGiveawayWinnerOption(Box<crate::types::StarGiveawayWinnerOption>),
}

impl StarGiveawayWinnerOption {
    /// Convenience constructor to create a [`StarGiveawayWinnerOption::StarGiveawayWinnerOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_giveaway_winner_option(val: crate::types::StarGiveawayWinnerOption) -> Self {
        Self::StarGiveawayWinnerOption(Box::new(val))
    }

}

/// Converts a [`crate::types::StarGiveawayWinnerOption`] into [`StarGiveawayWinnerOption`].
impl From<crate::types::StarGiveawayWinnerOption> for StarGiveawayWinnerOption {
    fn from(val: crate::types::StarGiveawayWinnerOption) -> Self {
        Self::StarGiveawayWinnerOption(Box::new(val))
    }
}

/// Contains information about status of a user in a giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayParticipantStatus {
    /// The user is eligible for the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusEligible", deserialize = "giveawayParticipantStatusEligible"))]
    Eligible,
    /// The user participates in the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusParticipating", deserialize = "giveawayParticipantStatusParticipating"))]
    Participating,
    /// The user can't participate in the giveaway, because they have already been member of the chat
    #[serde(rename(serialize = "giveawayParticipantStatusAlreadyWasMember", deserialize = "giveawayParticipantStatusAlreadyWasMember"))]
    AlreadyWasMember(Box<crate::types::GiveawayParticipantStatusAlreadyWasMember>),
    /// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
    #[serde(rename(serialize = "giveawayParticipantStatusAdministrator", deserialize = "giveawayParticipantStatusAdministrator"))]
    Administrator(Box<crate::types::GiveawayParticipantStatusAdministrator>),
    /// The user can't participate in the giveaway, because their phone number is from a disallowed country
    #[serde(rename(serialize = "giveawayParticipantStatusDisallowedCountry", deserialize = "giveawayParticipantStatusDisallowedCountry"))]
    DisallowedCountry(Box<crate::types::GiveawayParticipantStatusDisallowedCountry>),
}

impl GiveawayParticipantStatus {
    /// Convenience constructor to create a [`GiveawayParticipantStatus::AlreadyWasMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn already_was_member(val: crate::types::GiveawayParticipantStatusAlreadyWasMember) -> Self {
        Self::AlreadyWasMember(Box::new(val))
    }

    /// Convenience constructor to create a [`GiveawayParticipantStatus::Administrator`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn administrator(val: crate::types::GiveawayParticipantStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }

    /// Convenience constructor to create a [`GiveawayParticipantStatus::DisallowedCountry`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn disallowed_country(val: crate::types::GiveawayParticipantStatusDisallowedCountry) -> Self {
        Self::DisallowedCountry(Box::new(val))
    }

}

/// Converts a [`crate::types::GiveawayParticipantStatusAlreadyWasMember`] into [`GiveawayParticipantStatus`].
impl From<crate::types::GiveawayParticipantStatusAlreadyWasMember> for GiveawayParticipantStatus {
    fn from(val: crate::types::GiveawayParticipantStatusAlreadyWasMember) -> Self {
        Self::AlreadyWasMember(Box::new(val))
    }
}

/// Converts a [`crate::types::GiveawayParticipantStatusAdministrator`] into [`GiveawayParticipantStatus`].
impl From<crate::types::GiveawayParticipantStatusAdministrator> for GiveawayParticipantStatus {
    fn from(val: crate::types::GiveawayParticipantStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }
}

/// Converts a [`crate::types::GiveawayParticipantStatusDisallowedCountry`] into [`GiveawayParticipantStatus`].
impl From<crate::types::GiveawayParticipantStatusDisallowedCountry> for GiveawayParticipantStatus {
    fn from(val: crate::types::GiveawayParticipantStatusDisallowedCountry) -> Self {
        Self::DisallowedCountry(Box::new(val))
    }
}

/// Contains information about a giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayInfo {
    /// Describes an ongoing giveaway
    #[serde(rename(serialize = "giveawayInfoOngoing", deserialize = "giveawayInfoOngoing"))]
    Ongoing(Box<crate::types::GiveawayInfoOngoing>),
    /// Describes a completed giveaway
    #[serde(rename(serialize = "giveawayInfoCompleted", deserialize = "giveawayInfoCompleted"))]
    Completed(Box<crate::types::GiveawayInfoCompleted>),
}

impl GiveawayInfo {
    /// Convenience constructor to create a [`GiveawayInfo::Ongoing`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ongoing(val: crate::types::GiveawayInfoOngoing) -> Self {
        Self::Ongoing(Box::new(val))
    }

    /// Convenience constructor to create a [`GiveawayInfo::Completed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn completed(val: crate::types::GiveawayInfoCompleted) -> Self {
        Self::Completed(Box::new(val))
    }

}

/// Converts a [`crate::types::GiveawayInfoOngoing`] into [`GiveawayInfo`].
impl From<crate::types::GiveawayInfoOngoing> for GiveawayInfo {
    fn from(val: crate::types::GiveawayInfoOngoing) -> Self {
        Self::Ongoing(Box::new(val))
    }
}

/// Converts a [`crate::types::GiveawayInfoCompleted`] into [`GiveawayInfo`].
impl From<crate::types::GiveawayInfoCompleted> for GiveawayInfo {
    fn from(val: crate::types::GiveawayInfoCompleted) -> Self {
        Self::Completed(Box::new(val))
    }
}

/// Contains information about a giveaway prize
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayPrize {
    /// The giveaway sends Telegram Premium subscriptions to the winners
    #[serde(rename(serialize = "giveawayPrizePremium", deserialize = "giveawayPrizePremium"))]
    Premium(Box<crate::types::GiveawayPrizePremium>),
    /// The giveaway sends Telegram Stars to the winners
    #[serde(rename(serialize = "giveawayPrizeStars", deserialize = "giveawayPrizeStars"))]
    Stars(Box<crate::types::GiveawayPrizeStars>),
}

impl GiveawayPrize {
    /// Convenience constructor to create a [`GiveawayPrize::Premium`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium(val: crate::types::GiveawayPrizePremium) -> Self {
        Self::Premium(Box::new(val))
    }

    /// Convenience constructor to create a [`GiveawayPrize::Stars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stars(val: crate::types::GiveawayPrizeStars) -> Self {
        Self::Stars(Box::new(val))
    }

}

/// Converts a [`crate::types::GiveawayPrizePremium`] into [`GiveawayPrize`].
impl From<crate::types::GiveawayPrizePremium> for GiveawayPrize {
    fn from(val: crate::types::GiveawayPrizePremium) -> Self {
        Self::Premium(Box::new(val))
    }
}

/// Converts a [`crate::types::GiveawayPrizeStars`] into [`GiveawayPrize`].
impl From<crate::types::GiveawayPrizeStars> for GiveawayPrize {
    fn from(val: crate::types::GiveawayPrizeStars) -> Self {
        Self::Stars(Box::new(val))
    }
}

/// TDLib `BusinessMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessMessage {
    /// Describes a message from a business account as received by a bot
    #[serde(rename(serialize = "businessMessage", deserialize = "businessMessage"))]
    BusinessMessage(Box<crate::types::BusinessMessage>),
}

impl BusinessMessage {
    /// Convenience constructor to create a [`BusinessMessage::BusinessMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_message(val: crate::types::BusinessMessage) -> Self {
        Self::BusinessMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessMessage`] into [`BusinessMessage`].
impl From<crate::types::BusinessMessage> for BusinessMessage {
    fn from(val: crate::types::BusinessMessage) -> Self {
        Self::BusinessMessage(Box::new(val))
    }
}

/// TDLib `BusinessMessages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessMessages {
    /// Contains a list of messages from a business account as received by a bot
    #[serde(rename(serialize = "businessMessages", deserialize = "businessMessages"))]
    BusinessMessages(Box<crate::types::BusinessMessages>),
}

impl BusinessMessages {
    /// Convenience constructor to create a [`BusinessMessages::BusinessMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_messages(val: crate::types::BusinessMessages) -> Self {
        Self::BusinessMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessMessages`] into [`BusinessMessages`].
impl From<crate::types::BusinessMessages> for BusinessMessages {
    fn from(val: crate::types::BusinessMessages) -> Self {
        Self::BusinessMessages(Box::new(val))
    }
}

/// TDLib `BusinessBotManageBar` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessBotManageBar {
    /// Contains information about a business bot that manages the chat
    #[serde(rename(serialize = "businessBotManageBar", deserialize = "businessBotManageBar"))]
    BusinessBotManageBar(Box<crate::types::BusinessBotManageBar>),
}

impl BusinessBotManageBar {
    /// Convenience constructor to create a [`BusinessBotManageBar::BusinessBotManageBar`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_bot_manage_bar(val: crate::types::BusinessBotManageBar) -> Self {
        Self::BusinessBotManageBar(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessBotManageBar`] into [`BusinessBotManageBar`].
impl From<crate::types::BusinessBotManageBar> for BusinessBotManageBar {
    fn from(val: crate::types::BusinessBotManageBar) -> Self {
        Self::BusinessBotManageBar(Box::new(val))
    }
}

/// TDLib `GiveawayParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayParameters {
    /// Describes parameters of a giveaway
    #[serde(rename(serialize = "giveawayParameters", deserialize = "giveawayParameters"))]
    GiveawayParameters(Box<crate::types::GiveawayParameters>),
}

impl GiveawayParameters {
    /// Convenience constructor to create a [`GiveawayParameters::GiveawayParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn giveaway_parameters(val: crate::types::GiveawayParameters) -> Self {
        Self::GiveawayParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::GiveawayParameters`] into [`GiveawayParameters`].
impl From<crate::types::GiveawayParameters> for GiveawayParameters {
    fn from(val: crate::types::GiveawayParameters) -> Self {
        Self::GiveawayParameters(Box::new(val))
    }
}

/// TDLib `ChatBoostLevelFeatures` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLevelFeatures {
    /// Contains a list of features available on a specific chat boost level
    #[serde(rename(serialize = "chatBoostLevelFeatures", deserialize = "chatBoostLevelFeatures"))]
    ChatBoostLevelFeatures(Box<crate::types::ChatBoostLevelFeatures>),
}

impl ChatBoostLevelFeatures {
    /// Convenience constructor to create a [`ChatBoostLevelFeatures::ChatBoostLevelFeatures`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_level_features(val: crate::types::ChatBoostLevelFeatures) -> Self {
        Self::ChatBoostLevelFeatures(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostLevelFeatures`] into [`ChatBoostLevelFeatures`].
impl From<crate::types::ChatBoostLevelFeatures> for ChatBoostLevelFeatures {
    fn from(val: crate::types::ChatBoostLevelFeatures) -> Self {
        Self::ChatBoostLevelFeatures(Box::new(val))
    }
}

/// TDLib `ChatBoostFeatures` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostFeatures {
    /// Contains a list of features available on the first chat boost levels
    #[serde(rename(serialize = "chatBoostFeatures", deserialize = "chatBoostFeatures"))]
    ChatBoostFeatures(Box<crate::types::ChatBoostFeatures>),
}

impl ChatBoostFeatures {
    /// Convenience constructor to create a [`ChatBoostFeatures::ChatBoostFeatures`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_features(val: crate::types::ChatBoostFeatures) -> Self {
        Self::ChatBoostFeatures(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostFeatures`] into [`ChatBoostFeatures`].
impl From<crate::types::ChatBoostFeatures> for ChatBoostFeatures {
    fn from(val: crate::types::ChatBoostFeatures) -> Self {
        Self::ChatBoostFeatures(Box::new(val))
    }
}

/// Describes source of a chat boost
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSource {
    /// The chat created a Telegram Premium gift code for a user
    #[serde(rename(serialize = "chatBoostSourceGiftCode", deserialize = "chatBoostSourceGiftCode"))]
    GiftCode(Box<crate::types::ChatBoostSourceGiftCode>),
    /// The chat created a giveaway
    #[serde(rename(serialize = "chatBoostSourceGiveaway", deserialize = "chatBoostSourceGiveaway"))]
    Giveaway(Box<crate::types::ChatBoostSourceGiveaway>),
    /// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
    #[serde(rename(serialize = "chatBoostSourcePremium", deserialize = "chatBoostSourcePremium"))]
    Premium(Box<crate::types::ChatBoostSourcePremium>),
}

impl ChatBoostSource {
    /// Convenience constructor to create a [`ChatBoostSource::GiftCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_code(val: crate::types::ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatBoostSource::Giveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn giveaway(val: crate::types::ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatBoostSource::Premium`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium(val: crate::types::ChatBoostSourcePremium) -> Self {
        Self::Premium(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostSourceGiftCode`] into [`ChatBoostSource`].
impl From<crate::types::ChatBoostSourceGiftCode> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatBoostSourceGiveaway`] into [`ChatBoostSource`].
impl From<crate::types::ChatBoostSourceGiveaway> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatBoostSourcePremium`] into [`ChatBoostSource`].
impl From<crate::types::ChatBoostSourcePremium> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourcePremium) -> Self {
        Self::Premium(Box::new(val))
    }
}

/// TDLib `PrepaidGiveaway` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PrepaidGiveaway {
    /// Describes a prepaid giveaway
    #[serde(rename(serialize = "prepaidGiveaway", deserialize = "prepaidGiveaway"))]
    PrepaidGiveaway(Box<crate::types::PrepaidGiveaway>),
}

impl PrepaidGiveaway {
    /// Convenience constructor to create a [`PrepaidGiveaway::PrepaidGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn prepaid_giveaway(val: crate::types::PrepaidGiveaway) -> Self {
        Self::PrepaidGiveaway(Box::new(val))
    }

}

/// Converts a [`crate::types::PrepaidGiveaway`] into [`PrepaidGiveaway`].
impl From<crate::types::PrepaidGiveaway> for PrepaidGiveaway {
    fn from(val: crate::types::PrepaidGiveaway) -> Self {
        Self::PrepaidGiveaway(Box::new(val))
    }
}

/// TDLib `ChatBoostStatus` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostStatus {
    /// Describes current boost status of a chat
    #[serde(rename(serialize = "chatBoostStatus", deserialize = "chatBoostStatus"))]
    ChatBoostStatus(Box<crate::types::ChatBoostStatus>),
}

impl ChatBoostStatus {
    /// Convenience constructor to create a [`ChatBoostStatus::ChatBoostStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_status(val: crate::types::ChatBoostStatus) -> Self {
        Self::ChatBoostStatus(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostStatus`] into [`ChatBoostStatus`].
impl From<crate::types::ChatBoostStatus> for ChatBoostStatus {
    fn from(val: crate::types::ChatBoostStatus) -> Self {
        Self::ChatBoostStatus(Box::new(val))
    }
}

/// TDLib `ChatBoost` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoost {
    /// Describes a boost applied to a chat
    #[serde(rename(serialize = "chatBoost", deserialize = "chatBoost"))]
    ChatBoost(Box<crate::types::ChatBoost>),
}

impl ChatBoost {
    /// Convenience constructor to create a [`ChatBoost::ChatBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost(val: crate::types::ChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoost`] into [`ChatBoost`].
impl From<crate::types::ChatBoost> for ChatBoost {
    fn from(val: crate::types::ChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }
}

/// TDLib `FoundChatBoosts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundChatBoosts {
    /// Contains a list of boosts applied to a chat
    #[serde(rename(serialize = "foundChatBoosts", deserialize = "foundChatBoosts"))]
    FoundChatBoosts(Box<crate::types::FoundChatBoosts>),
}

impl FoundChatBoosts {
    /// Convenience constructor to create a [`FoundChatBoosts::FoundChatBoosts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_chat_boosts(val: crate::types::FoundChatBoosts) -> Self {
        Self::FoundChatBoosts(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundChatBoosts`] into [`FoundChatBoosts`].
impl From<crate::types::FoundChatBoosts> for FoundChatBoosts {
    fn from(val: crate::types::FoundChatBoosts) -> Self {
        Self::FoundChatBoosts(Box::new(val))
    }
}

/// TDLib `ChatBoostSlot` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSlot {
    /// Describes a slot for chat boost
    #[serde(rename(serialize = "chatBoostSlot", deserialize = "chatBoostSlot"))]
    ChatBoostSlot(Box<crate::types::ChatBoostSlot>),
}

impl ChatBoostSlot {
    /// Convenience constructor to create a [`ChatBoostSlot::ChatBoostSlot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_slot(val: crate::types::ChatBoostSlot) -> Self {
        Self::ChatBoostSlot(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostSlot`] into [`ChatBoostSlot`].
impl From<crate::types::ChatBoostSlot> for ChatBoostSlot {
    fn from(val: crate::types::ChatBoostSlot) -> Self {
        Self::ChatBoostSlot(Box::new(val))
    }
}

/// TDLib `ChatBoostSlots` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostSlots {
    /// Contains a list of chat boost slots
    #[serde(rename(serialize = "chatBoostSlots", deserialize = "chatBoostSlots"))]
    ChatBoostSlots(Box<crate::types::ChatBoostSlots>),
}

impl ChatBoostSlots {
    /// Convenience constructor to create a [`ChatBoostSlots::ChatBoostSlots`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_slots(val: crate::types::ChatBoostSlots) -> Self {
        Self::ChatBoostSlots(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostSlots`] into [`ChatBoostSlots`].
impl From<crate::types::ChatBoostSlots> for ChatBoostSlots {
    fn from(val: crate::types::ChatBoostSlots) -> Self {
        Self::ChatBoostSlots(Box::new(val))
    }
}

/// TDLib `BusinessConnection` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessConnection {
    /// Describes a connection of the bot with a business account
    #[serde(rename(serialize = "businessConnection", deserialize = "businessConnection"))]
    BusinessConnection(Box<crate::types::BusinessConnection>),
}

impl BusinessConnection {
    /// Convenience constructor to create a [`BusinessConnection::BusinessConnection`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_connection(val: crate::types::BusinessConnection) -> Self {
        Self::BusinessConnection(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessConnection`] into [`BusinessConnection`].
impl From<crate::types::BusinessConnection> for BusinessConnection {
    fn from(val: crate::types::BusinessConnection) -> Self {
        Self::BusinessConnection(Box::new(val))
    }
}

/// Describes type of limit, increased for Premium users
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
    /// The maximum length of text of sent messages
    #[serde(rename(serialize = "premiumLimitTypeMessageTextLength", deserialize = "premiumLimitTypeMessageTextLength"))]
    MessageTextLength,
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
    /// The maximum number of owned bots
    #[serde(rename(serialize = "premiumLimitTypeOwnedBotCount", deserialize = "premiumLimitTypeOwnedBotCount"))]
    OwnedBotCount,
    /// The maximum number of added text composition styles
    #[serde(rename(serialize = "premiumLimitTypeCustomTextCompositionStyleCount", deserialize = "premiumLimitTypeCustomTextCompositionStyleCount"))]
    CustomTextCompositionStyleCount,
}

/// Describes a feature available to Premium users
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeature {
    /// Increased limits
    #[serde(rename(serialize = "premiumFeatureIncreasedLimits", deserialize = "premiumFeatureIncreasedLimits"))]
    IncreasedLimits,
    /// Increased maximum upload file size
    #[serde(rename(serialize = "premiumFeatureIncreasedUploadFileSize", deserialize = "premiumFeatureIncreasedUploadFileSize"))]
    IncreasedUploadFileSize,
    /// Improved download speed
    #[serde(rename(serialize = "premiumFeatureImprovedDownloadSpeed", deserialize = "premiumFeatureImprovedDownloadSpeed"))]
    ImprovedDownloadSpeed,
    /// The ability to convert voice notes to text
    #[serde(rename(serialize = "premiumFeatureVoiceRecognition", deserialize = "premiumFeatureVoiceRecognition"))]
    VoiceRecognition,
    /// Disabled ads
    #[serde(rename(serialize = "premiumFeatureDisabledAds", deserialize = "premiumFeatureDisabledAds"))]
    DisabledAds,
    /// Allowed to use more reactions
    #[serde(rename(serialize = "premiumFeatureUniqueReactions", deserialize = "premiumFeatureUniqueReactions"))]
    UniqueReactions,
    /// Allowed to use premium stickers with unique effects
    #[serde(rename(serialize = "premiumFeatureUniqueStickers", deserialize = "premiumFeatureUniqueStickers"))]
    UniqueStickers,
    /// Allowed to use custom emoji stickers in message texts and captions
    #[serde(rename(serialize = "premiumFeatureCustomEmoji", deserialize = "premiumFeatureCustomEmoji"))]
    CustomEmoji,
    /// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
    #[serde(rename(serialize = "premiumFeatureAdvancedChatManagement", deserialize = "premiumFeatureAdvancedChatManagement"))]
    AdvancedChatManagement,
    /// A badge in the user's profile
    #[serde(rename(serialize = "premiumFeatureProfileBadge", deserialize = "premiumFeatureProfileBadge"))]
    ProfileBadge,
    /// The ability to show an emoji status along with the user's name
    #[serde(rename(serialize = "premiumFeatureEmojiStatus", deserialize = "premiumFeatureEmojiStatus"))]
    EmojiStatus,
    /// Profile photo animation on message and chat screens
    #[serde(rename(serialize = "premiumFeatureAnimatedProfilePhoto", deserialize = "premiumFeatureAnimatedProfilePhoto"))]
    AnimatedProfilePhoto,
    /// The ability to set a custom emoji as a forum topic icon
    #[serde(rename(serialize = "premiumFeatureForumTopicIcon", deserialize = "premiumFeatureForumTopicIcon"))]
    ForumTopicIcon,
    /// Allowed to set a premium application icons
    #[serde(rename(serialize = "premiumFeatureAppIcons", deserialize = "premiumFeatureAppIcons"))]
    AppIcons,
    /// Allowed to translate chat messages real-time
    #[serde(rename(serialize = "premiumFeatureRealTimeChatTranslation", deserialize = "premiumFeatureRealTimeChatTranslation"))]
    RealTimeChatTranslation,
    /// Allowed to use many additional features for stories
    #[serde(rename(serialize = "premiumFeatureUpgradedStories", deserialize = "premiumFeatureUpgradedStories"))]
    UpgradedStories,
    /// The ability to boost chats
    #[serde(rename(serialize = "premiumFeatureChatBoost", deserialize = "premiumFeatureChatBoost"))]
    ChatBoost,
    /// The ability to choose accent color for replies and user profile
    #[serde(rename(serialize = "premiumFeatureAccentColor", deserialize = "premiumFeatureAccentColor"))]
    AccentColor,
    /// The ability to set private chat background for both users
    #[serde(rename(serialize = "premiumFeatureBackgroundForBoth", deserialize = "premiumFeatureBackgroundForBoth"))]
    BackgroundForBoth,
    /// The ability to use tags in Saved Messages
    #[serde(rename(serialize = "premiumFeatureSavedMessagesTags", deserialize = "premiumFeatureSavedMessagesTags"))]
    SavedMessagesTags,
    /// The ability to disallow incoming voice and video note messages in private chats using setUserPrivacySettingRules with userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages
    /// and to restrict incoming messages from non-contacts using setNewChatPrivacySettings
    #[serde(rename(serialize = "premiumFeatureMessagePrivacy", deserialize = "premiumFeatureMessagePrivacy"))]
    MessagePrivacy,
    /// The ability to view last seen and read times of other users even if they can't view last seen or read time for the current user
    #[serde(rename(serialize = "premiumFeatureLastSeenTimes", deserialize = "premiumFeatureLastSeenTimes"))]
    LastSeenTimes,
    /// The ability to use Business features
    #[serde(rename(serialize = "premiumFeatureBusiness", deserialize = "premiumFeatureBusiness"))]
    Business,
    /// The ability to use all available message effects
    #[serde(rename(serialize = "premiumFeatureMessageEffects", deserialize = "premiumFeatureMessageEffects"))]
    MessageEffects,
    /// The ability to create and use checklist messages
    #[serde(rename(serialize = "premiumFeatureChecklists", deserialize = "premiumFeatureChecklists"))]
    Checklists,
    /// The ability to require a payment for incoming messages in new chats
    #[serde(rename(serialize = "premiumFeaturePaidMessages", deserialize = "premiumFeaturePaidMessages"))]
    PaidMessages,
    /// The ability to enable content protection in private chats
    #[serde(rename(serialize = "premiumFeatureProtectPrivateChatContent", deserialize = "premiumFeatureProtectPrivateChatContent"))]
    ProtectPrivateChatContent,
    /// The ability to compose text with AI
    #[serde(rename(serialize = "premiumFeatureTextComposition", deserialize = "premiumFeatureTextComposition"))]
    TextComposition,
    /// The ability to send rich messages
    #[serde(rename(serialize = "premiumFeatureRichMessages", deserialize = "premiumFeatureRichMessages"))]
    RichMessages,
}

/// Describes a feature available to Business user accounts
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeature {
    /// The ability to set location
    #[serde(rename(serialize = "businessFeatureLocation", deserialize = "businessFeatureLocation"))]
    Location,
    /// The ability to set opening hours
    #[serde(rename(serialize = "businessFeatureOpeningHours", deserialize = "businessFeatureOpeningHours"))]
    OpeningHours,
    /// The ability to use quick replies
    #[serde(rename(serialize = "businessFeatureQuickReplies", deserialize = "businessFeatureQuickReplies"))]
    QuickReplies,
    /// The ability to set up a greeting message
    #[serde(rename(serialize = "businessFeatureGreetingMessage", deserialize = "businessFeatureGreetingMessage"))]
    GreetingMessage,
    /// The ability to set up an away message
    #[serde(rename(serialize = "businessFeatureAwayMessage", deserialize = "businessFeatureAwayMessage"))]
    AwayMessage,
    /// The ability to create links to the business account with predefined message text
    #[serde(rename(serialize = "businessFeatureAccountLinks", deserialize = "businessFeatureAccountLinks"))]
    AccountLinks,
    /// The ability to customize start page
    #[serde(rename(serialize = "businessFeatureStartPage", deserialize = "businessFeatureStartPage"))]
    StartPage,
    /// The ability to connect a bot to the account
    #[serde(rename(serialize = "businessFeatureBots", deserialize = "businessFeatureBots"))]
    Bots,
    /// The ability to show an emoji status along with the business name
    #[serde(rename(serialize = "businessFeatureEmojiStatus", deserialize = "businessFeatureEmojiStatus"))]
    EmojiStatus,
    /// The ability to display folder names for each chat in the chat list
    #[serde(rename(serialize = "businessFeatureChatFolderTags", deserialize = "businessFeatureChatFolderTags"))]
    ChatFolderTags,
    /// Allowed to use many additional features for stories
    #[serde(rename(serialize = "businessFeatureUpgradedStories", deserialize = "businessFeatureUpgradedStories"))]
    UpgradedStories,
}

/// TDLib `PremiumLimit` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumLimit {
    /// Contains information about a limit, increased for Premium users
    #[serde(rename(serialize = "premiumLimit", deserialize = "premiumLimit"))]
    PremiumLimit(Box<crate::types::PremiumLimit>),
}

impl PremiumLimit {
    /// Convenience constructor to create a [`PremiumLimit::PremiumLimit`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_limit(val: crate::types::PremiumLimit) -> Self {
        Self::PremiumLimit(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumLimit`] into [`PremiumLimit`].
impl From<crate::types::PremiumLimit> for PremiumLimit {
    fn from(val: crate::types::PremiumLimit) -> Self {
        Self::PremiumLimit(Box::new(val))
    }
}

/// TDLib `PremiumFeatures` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeatures {
    /// Contains information about features, available to Premium users
    #[serde(rename(serialize = "premiumFeatures", deserialize = "premiumFeatures"))]
    PremiumFeatures(Box<crate::types::PremiumFeatures>),
}

impl PremiumFeatures {
    /// Convenience constructor to create a [`PremiumFeatures::PremiumFeatures`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_features(val: crate::types::PremiumFeatures) -> Self {
        Self::PremiumFeatures(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumFeatures`] into [`PremiumFeatures`].
impl From<crate::types::PremiumFeatures> for PremiumFeatures {
    fn from(val: crate::types::PremiumFeatures) -> Self {
        Self::PremiumFeatures(Box::new(val))
    }
}

/// TDLib `BusinessFeatures` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeatures {
    /// Contains information about features, available to Business user accounts
    #[serde(rename(serialize = "businessFeatures", deserialize = "businessFeatures"))]
    BusinessFeatures(Box<crate::types::BusinessFeatures>),
}

impl BusinessFeatures {
    /// Convenience constructor to create a [`BusinessFeatures::BusinessFeatures`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_features(val: crate::types::BusinessFeatures) -> Self {
        Self::BusinessFeatures(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessFeatures`] into [`BusinessFeatures`].
impl From<crate::types::BusinessFeatures> for BusinessFeatures {
    fn from(val: crate::types::BusinessFeatures) -> Self {
        Self::BusinessFeatures(Box::new(val))
    }
}

/// Describes a source from which the Premium features screen is opened
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumSource {
    /// A limit was exceeded
    #[serde(rename(serialize = "premiumSourceLimitExceeded", deserialize = "premiumSourceLimitExceeded"))]
    LimitExceeded(Box<crate::types::PremiumSourceLimitExceeded>),
    /// A user tried to use a Premium feature
    #[serde(rename(serialize = "premiumSourceFeature", deserialize = "premiumSourceFeature"))]
    Feature(Box<crate::types::PremiumSourceFeature>),
    /// A user tried to use a Business feature
    #[serde(rename(serialize = "premiumSourceBusinessFeature", deserialize = "premiumSourceBusinessFeature"))]
    BusinessFeature(Box<crate::types::PremiumSourceBusinessFeature>),
    /// A user tried to use a Premium story feature
    #[serde(rename(serialize = "premiumSourceStoryFeature", deserialize = "premiumSourceStoryFeature"))]
    StoryFeature(Box<crate::types::PremiumSourceStoryFeature>),
    /// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
    #[serde(rename(serialize = "premiumSourceLink", deserialize = "premiumSourceLink"))]
    Link(Box<crate::types::PremiumSourceLink>),
    /// A user opened the Premium features screen from settings
    #[serde(rename(serialize = "premiumSourceSettings", deserialize = "premiumSourceSettings"))]
    Settings,
}

impl PremiumSource {
    /// Convenience constructor to create a [`PremiumSource::LimitExceeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn limit_exceeded(val: crate::types::PremiumSourceLimitExceeded) -> Self {
        Self::LimitExceeded(Box::new(val))
    }

    /// Convenience constructor to create a [`PremiumSource::Feature`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn feature(val: crate::types::PremiumSourceFeature) -> Self {
        Self::Feature(Box::new(val))
    }

    /// Convenience constructor to create a [`PremiumSource::BusinessFeature`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_feature(val: crate::types::PremiumSourceBusinessFeature) -> Self {
        Self::BusinessFeature(Box::new(val))
    }

    /// Convenience constructor to create a [`PremiumSource::StoryFeature`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_feature(val: crate::types::PremiumSourceStoryFeature) -> Self {
        Self::StoryFeature(Box::new(val))
    }

    /// Convenience constructor to create a [`PremiumSource::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::PremiumSourceLink) -> Self {
        Self::Link(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumSourceLimitExceeded`] into [`PremiumSource`].
impl From<crate::types::PremiumSourceLimitExceeded> for PremiumSource {
    fn from(val: crate::types::PremiumSourceLimitExceeded) -> Self {
        Self::LimitExceeded(Box::new(val))
    }
}

/// Converts a [`crate::types::PremiumSourceFeature`] into [`PremiumSource`].
impl From<crate::types::PremiumSourceFeature> for PremiumSource {
    fn from(val: crate::types::PremiumSourceFeature) -> Self {
        Self::Feature(Box::new(val))
    }
}

/// Converts a [`crate::types::PremiumSourceBusinessFeature`] into [`PremiumSource`].
impl From<crate::types::PremiumSourceBusinessFeature> for PremiumSource {
    fn from(val: crate::types::PremiumSourceBusinessFeature) -> Self {
        Self::BusinessFeature(Box::new(val))
    }
}

/// Converts a [`crate::types::PremiumSourceStoryFeature`] into [`PremiumSource`].
impl From<crate::types::PremiumSourceStoryFeature> for PremiumSource {
    fn from(val: crate::types::PremiumSourceStoryFeature) -> Self {
        Self::StoryFeature(Box::new(val))
    }
}

/// Converts a [`crate::types::PremiumSourceLink`] into [`PremiumSource`].
impl From<crate::types::PremiumSourceLink> for PremiumSource {
    fn from(val: crate::types::PremiumSourceLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// TDLib `PremiumFeaturePromotionAnimation` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeaturePromotionAnimation {
    /// Describes a promotion animation for a Premium feature
    #[serde(rename(serialize = "premiumFeaturePromotionAnimation", deserialize = "premiumFeaturePromotionAnimation"))]
    PremiumFeaturePromotionAnimation(Box<crate::types::PremiumFeaturePromotionAnimation>),
}

impl PremiumFeaturePromotionAnimation {
    /// Convenience constructor to create a [`PremiumFeaturePromotionAnimation::PremiumFeaturePromotionAnimation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_feature_promotion_animation(val: crate::types::PremiumFeaturePromotionAnimation) -> Self {
        Self::PremiumFeaturePromotionAnimation(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumFeaturePromotionAnimation`] into [`PremiumFeaturePromotionAnimation`].
impl From<crate::types::PremiumFeaturePromotionAnimation> for PremiumFeaturePromotionAnimation {
    fn from(val: crate::types::PremiumFeaturePromotionAnimation) -> Self {
        Self::PremiumFeaturePromotionAnimation(Box::new(val))
    }
}

/// TDLib `BusinessFeaturePromotionAnimation` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeaturePromotionAnimation {
    /// Describes a promotion animation for a Business feature
    #[serde(rename(serialize = "businessFeaturePromotionAnimation", deserialize = "businessFeaturePromotionAnimation"))]
    BusinessFeaturePromotionAnimation(Box<crate::types::BusinessFeaturePromotionAnimation>),
}

impl BusinessFeaturePromotionAnimation {
    /// Convenience constructor to create a [`BusinessFeaturePromotionAnimation::BusinessFeaturePromotionAnimation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_feature_promotion_animation(val: crate::types::BusinessFeaturePromotionAnimation) -> Self {
        Self::BusinessFeaturePromotionAnimation(Box::new(val))
    }

}

/// Converts a [`crate::types::BusinessFeaturePromotionAnimation`] into [`BusinessFeaturePromotionAnimation`].
impl From<crate::types::BusinessFeaturePromotionAnimation> for BusinessFeaturePromotionAnimation {
    fn from(val: crate::types::BusinessFeaturePromotionAnimation) -> Self {
        Self::BusinessFeaturePromotionAnimation(Box::new(val))
    }
}

/// TDLib `PremiumState` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumState {
    /// Contains state of Telegram Premium subscription and promotion videos for Premium features
    #[serde(rename(serialize = "premiumState", deserialize = "premiumState"))]
    PremiumState(Box<crate::types::PremiumState>),
}

impl PremiumState {
    /// Convenience constructor to create a [`PremiumState::PremiumState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_state(val: crate::types::PremiumState) -> Self {
        Self::PremiumState(Box::new(val))
    }

}

/// Converts a [`crate::types::PremiumState`] into [`PremiumState`].
impl From<crate::types::PremiumState> for PremiumState {
    fn from(val: crate::types::PremiumState) -> Self {
        Self::PremiumState(Box::new(val))
    }
}

/// TDLib `ChatBoostLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLink {
    /// Contains an HTTPS link to boost a chat
    #[serde(rename(serialize = "chatBoostLink", deserialize = "chatBoostLink"))]
    ChatBoostLink(Box<crate::types::ChatBoostLink>),
}

impl ChatBoostLink {
    /// Convenience constructor to create a [`ChatBoostLink::ChatBoostLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_link(val: crate::types::ChatBoostLink) -> Self {
        Self::ChatBoostLink(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostLink`] into [`ChatBoostLink`].
impl From<crate::types::ChatBoostLink> for ChatBoostLink {
    fn from(val: crate::types::ChatBoostLink) -> Self {
        Self::ChatBoostLink(Box::new(val))
    }
}

/// TDLib `ChatBoostLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLinkInfo {
    /// Contains information about a link to boost a chat
    #[serde(rename(serialize = "chatBoostLinkInfo", deserialize = "chatBoostLinkInfo"))]
    ChatBoostLinkInfo(Box<crate::types::ChatBoostLinkInfo>),
}

impl ChatBoostLinkInfo {
    /// Convenience constructor to create a [`ChatBoostLinkInfo::ChatBoostLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost_link_info(val: crate::types::ChatBoostLinkInfo) -> Self {
        Self::ChatBoostLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBoostLinkInfo`] into [`ChatBoostLinkInfo`].
impl From<crate::types::ChatBoostLinkInfo> for ChatBoostLinkInfo {
    fn from(val: crate::types::ChatBoostLinkInfo) -> Self {
        Self::ChatBoostLinkInfo(Box::new(val))
    }
}

