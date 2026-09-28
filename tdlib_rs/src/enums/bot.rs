//!
//! TDLib `bot` domain enums.
//!
//! Types, enums, and functions for Telegram Bots, Web Apps, and inline queries.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `BotCommand` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommand {
    /// Represents a command supported by a bot
    #[serde(rename(serialize = "botCommand", deserialize = "botCommand"))]
    BotCommand(Box<crate::types::BotCommand>),
}

impl BotCommand {
    /// Convenience constructor to create a [`BotCommand::BotCommand`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_command(val: crate::types::BotCommand) -> Self {
        Self::BotCommand(Box::new(val))
    }

}

/// Converts a [`crate::types::BotCommand`] into [`BotCommand`].
impl From<crate::types::BotCommand> for BotCommand {
    fn from(val: crate::types::BotCommand) -> Self {
        Self::BotCommand(Box::new(val))
    }
}

/// TDLib `BotCommands` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommands {
    /// Contains a list of bot commands
    #[serde(rename(serialize = "botCommands", deserialize = "botCommands"))]
    BotCommands(Box<crate::types::BotCommands>),
}

impl BotCommands {
    /// Convenience constructor to create a [`BotCommands::BotCommands`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_commands(val: crate::types::BotCommands) -> Self {
        Self::BotCommands(Box::new(val))
    }

}

/// Converts a [`crate::types::BotCommands`] into [`BotCommands`].
impl From<crate::types::BotCommands> for BotCommands {
    fn from(val: crate::types::BotCommands) -> Self {
        Self::BotCommands(Box::new(val))
    }
}

/// TDLib `BotMenuButton` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMenuButton {
    /// Describes a button to be shown instead of bot commands menu button
    #[serde(rename(serialize = "botMenuButton", deserialize = "botMenuButton"))]
    BotMenuButton(Box<crate::types::BotMenuButton>),
}

impl BotMenuButton {
    /// Convenience constructor to create a [`BotMenuButton::BotMenuButton`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_menu_button(val: crate::types::BotMenuButton) -> Self {
        Self::BotMenuButton(Box::new(val))
    }

}

/// Converts a [`crate::types::BotMenuButton`] into [`BotMenuButton`].
impl From<crate::types::BotMenuButton> for BotMenuButton {
    fn from(val: crate::types::BotMenuButton) -> Self {
        Self::BotMenuButton(Box::new(val))
    }
}

/// TDLib `BotAccessSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotAccessSettings {
    /// Describes users that have access to a bot
    #[serde(rename(serialize = "botAccessSettings", deserialize = "botAccessSettings"))]
    BotAccessSettings(Box<crate::types::BotAccessSettings>),
}

impl BotAccessSettings {
    /// Convenience constructor to create a [`BotAccessSettings::BotAccessSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_access_settings(val: crate::types::BotAccessSettings) -> Self {
        Self::BotAccessSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::BotAccessSettings`] into [`BotAccessSettings`].
impl From<crate::types::BotAccessSettings> for BotAccessSettings {
    fn from(val: crate::types::BotAccessSettings) -> Self {
        Self::BotAccessSettings(Box::new(val))
    }
}

/// TDLib `BotVerificationParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotVerificationParameters {
    /// Describes parameters of verification that is provided by a bot
    #[serde(rename(serialize = "botVerificationParameters", deserialize = "botVerificationParameters"))]
    BotVerificationParameters(Box<crate::types::BotVerificationParameters>),
}

impl BotVerificationParameters {
    /// Convenience constructor to create a [`BotVerificationParameters::BotVerificationParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_verification_parameters(val: crate::types::BotVerificationParameters) -> Self {
        Self::BotVerificationParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::BotVerificationParameters`] into [`BotVerificationParameters`].
impl From<crate::types::BotVerificationParameters> for BotVerificationParameters {
    fn from(val: crate::types::BotVerificationParameters) -> Self {
        Self::BotVerificationParameters(Box::new(val))
    }
}

/// TDLib `BotVerification` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotVerification {
    /// Describes verification status provided by a bot
    #[serde(rename(serialize = "botVerification", deserialize = "botVerification"))]
    BotVerification(Box<crate::types::BotVerification>),
}

impl BotVerification {
    /// Convenience constructor to create a [`BotVerification::BotVerification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_verification(val: crate::types::BotVerification) -> Self {
        Self::BotVerification(Box::new(val))
    }

}

/// Converts a [`crate::types::BotVerification`] into [`BotVerification`].
impl From<crate::types::BotVerification> for BotVerification {
    fn from(val: crate::types::BotVerification) -> Self {
        Self::BotVerification(Box::new(val))
    }
}

/// TDLib `BotInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotInfo {
    /// Contains information about a bot
    #[serde(rename(serialize = "botInfo", deserialize = "botInfo"))]
    BotInfo(Box<crate::types::BotInfo>),
}

impl BotInfo {
    /// Convenience constructor to create a [`BotInfo::BotInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_info(val: crate::types::BotInfo) -> Self {
        Self::BotInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BotInfo`] into [`BotInfo`].
impl From<crate::types::BotInfo> for BotInfo {
    fn from(val: crate::types::BotInfo) -> Self {
        Self::BotInfo(Box::new(val))
    }
}

/// TDLib `BotMediaPreview` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreview {
    /// Describes media previews of a bot
    #[serde(rename(serialize = "botMediaPreview", deserialize = "botMediaPreview"))]
    BotMediaPreview(Box<crate::types::BotMediaPreview>),
}

impl BotMediaPreview {
    /// Convenience constructor to create a [`BotMediaPreview::BotMediaPreview`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_media_preview(val: crate::types::BotMediaPreview) -> Self {
        Self::BotMediaPreview(Box::new(val))
    }

}

/// Converts a [`crate::types::BotMediaPreview`] into [`BotMediaPreview`].
impl From<crate::types::BotMediaPreview> for BotMediaPreview {
    fn from(val: crate::types::BotMediaPreview) -> Self {
        Self::BotMediaPreview(Box::new(val))
    }
}

/// TDLib `BotMediaPreviews` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreviews {
    /// Contains a list of media previews of a bot
    #[serde(rename(serialize = "botMediaPreviews", deserialize = "botMediaPreviews"))]
    BotMediaPreviews(Box<crate::types::BotMediaPreviews>),
}

impl BotMediaPreviews {
    /// Convenience constructor to create a [`BotMediaPreviews::BotMediaPreviews`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_media_previews(val: crate::types::BotMediaPreviews) -> Self {
        Self::BotMediaPreviews(Box::new(val))
    }

}

/// Converts a [`crate::types::BotMediaPreviews`] into [`BotMediaPreviews`].
impl From<crate::types::BotMediaPreviews> for BotMediaPreviews {
    fn from(val: crate::types::BotMediaPreviews) -> Self {
        Self::BotMediaPreviews(Box::new(val))
    }
}

/// TDLib `BotMediaPreviewInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMediaPreviewInfo {
    /// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
    #[serde(rename(serialize = "botMediaPreviewInfo", deserialize = "botMediaPreviewInfo"))]
    BotMediaPreviewInfo(Box<crate::types::BotMediaPreviewInfo>),
}

impl BotMediaPreviewInfo {
    /// Convenience constructor to create a [`BotMediaPreviewInfo::BotMediaPreviewInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_media_preview_info(val: crate::types::BotMediaPreviewInfo) -> Self {
        Self::BotMediaPreviewInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BotMediaPreviewInfo`] into [`BotMediaPreviewInfo`].
impl From<crate::types::BotMediaPreviewInfo> for BotMediaPreviewInfo {
    fn from(val: crate::types::BotMediaPreviewInfo) -> Self {
        Self::BotMediaPreviewInfo(Box::new(val))
    }
}

/// TDLib `AttachmentMenuBotColor` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttachmentMenuBotColor {
    /// Describes a color to highlight a bot added to attachment menu
    #[serde(rename(serialize = "attachmentMenuBotColor", deserialize = "attachmentMenuBotColor"))]
    AttachmentMenuBotColor(Box<crate::types::AttachmentMenuBotColor>),
}

impl AttachmentMenuBotColor {
    /// Convenience constructor to create a [`AttachmentMenuBotColor::AttachmentMenuBotColor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn attachment_menu_bot_color(val: crate::types::AttachmentMenuBotColor) -> Self {
        Self::AttachmentMenuBotColor(Box::new(val))
    }

}

/// Converts a [`crate::types::AttachmentMenuBotColor`] into [`AttachmentMenuBotColor`].
impl From<crate::types::AttachmentMenuBotColor> for AttachmentMenuBotColor {
    fn from(val: crate::types::AttachmentMenuBotColor) -> Self {
        Self::AttachmentMenuBotColor(Box::new(val))
    }
}

/// TDLib `AttachmentMenuBot` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttachmentMenuBot {
    /// Represents a bot, which can be added to attachment or side menu
    #[serde(rename(serialize = "attachmentMenuBot", deserialize = "attachmentMenuBot"))]
    AttachmentMenuBot(Box<crate::types::AttachmentMenuBot>),
}

impl AttachmentMenuBot {
    /// Convenience constructor to create a [`AttachmentMenuBot::AttachmentMenuBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn attachment_menu_bot(val: crate::types::AttachmentMenuBot) -> Self {
        Self::AttachmentMenuBot(Box::new(val))
    }

}

/// Converts a [`crate::types::AttachmentMenuBot`] into [`AttachmentMenuBot`].
impl From<crate::types::AttachmentMenuBot> for AttachmentMenuBot {
    fn from(val: crate::types::AttachmentMenuBot) -> Self {
        Self::AttachmentMenuBot(Box::new(val))
    }
}

/// Describes a reason why a bot was allowed to write messages to the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotWriteAccessAllowReason {
    /// The user connected a website by logging in using Telegram Login Widget on it
    #[serde(rename(serialize = "botWriteAccessAllowReasonConnectedWebsite", deserialize = "botWriteAccessAllowReasonConnectedWebsite"))]
    ConnectedWebsite(Box<crate::types::BotWriteAccessAllowReasonConnectedWebsite>),
    /// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
    #[serde(rename(serialize = "botWriteAccessAllowReasonAddedToAttachmentMenu", deserialize = "botWriteAccessAllowReasonAddedToAttachmentMenu"))]
    AddedToAttachmentMenu,
    /// The user launched a Web App using getWebAppLinkUrl
    #[serde(rename(serialize = "botWriteAccessAllowReasonLaunchedWebApp", deserialize = "botWriteAccessAllowReasonLaunchedWebApp"))]
    LaunchedWebApp(Box<crate::types::BotWriteAccessAllowReasonLaunchedWebApp>),
    /// The user accepted bot's request to send messages with allowBotToSendMessages
    #[serde(rename(serialize = "botWriteAccessAllowReasonAcceptedRequest", deserialize = "botWriteAccessAllowReasonAcceptedRequest"))]
    AcceptedRequest,
}

impl BotWriteAccessAllowReason {
    /// Convenience constructor to create a [`BotWriteAccessAllowReason::ConnectedWebsite`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_website(val: crate::types::BotWriteAccessAllowReasonConnectedWebsite) -> Self {
        Self::ConnectedWebsite(Box::new(val))
    }

    /// Convenience constructor to create a [`BotWriteAccessAllowReason::LaunchedWebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn launched_web_app(val: crate::types::BotWriteAccessAllowReasonLaunchedWebApp) -> Self {
        Self::LaunchedWebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::BotWriteAccessAllowReasonConnectedWebsite`] into [`BotWriteAccessAllowReason`].
impl From<crate::types::BotWriteAccessAllowReasonConnectedWebsite> for BotWriteAccessAllowReason {
    fn from(val: crate::types::BotWriteAccessAllowReasonConnectedWebsite) -> Self {
        Self::ConnectedWebsite(Box::new(val))
    }
}

/// Converts a [`crate::types::BotWriteAccessAllowReasonLaunchedWebApp`] into [`BotWriteAccessAllowReason`].
impl From<crate::types::BotWriteAccessAllowReasonLaunchedWebApp> for BotWriteAccessAllowReason {
    fn from(val: crate::types::BotWriteAccessAllowReasonLaunchedWebApp) -> Self {
        Self::LaunchedWebApp(Box::new(val))
    }
}

/// Represents the scope to which bot commands are relevant
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommandScope {
    /// A scope covering all users
    #[serde(rename(serialize = "botCommandScopeDefault", deserialize = "botCommandScopeDefault"))]
    Default,
    /// A scope covering all private chats
    #[serde(rename(serialize = "botCommandScopeAllPrivateChats", deserialize = "botCommandScopeAllPrivateChats"))]
    AllPrivateChats,
    /// A scope covering all group and supergroup chats
    #[serde(rename(serialize = "botCommandScopeAllGroupChats", deserialize = "botCommandScopeAllGroupChats"))]
    AllGroupChats,
    /// A scope covering all group and supergroup chat administrators
    #[serde(rename(serialize = "botCommandScopeAllChatAdministrators", deserialize = "botCommandScopeAllChatAdministrators"))]
    AllChatAdministrators,
    /// A scope covering all members of a chat
    #[serde(rename(serialize = "botCommandScopeChat", deserialize = "botCommandScopeChat"))]
    Chat(Box<crate::types::BotCommandScopeChat>),
    /// A scope covering all administrators of a chat
    #[serde(rename(serialize = "botCommandScopeChatAdministrators", deserialize = "botCommandScopeChatAdministrators"))]
    ChatAdministrators(Box<crate::types::BotCommandScopeChatAdministrators>),
    /// A scope covering a member of a chat
    #[serde(rename(serialize = "botCommandScopeChatMember", deserialize = "botCommandScopeChatMember"))]
    ChatMember(Box<crate::types::BotCommandScopeChatMember>),
}

impl BotCommandScope {
    /// Convenience constructor to create a [`BotCommandScope::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::BotCommandScopeChat) -> Self {
        Self::Chat(Box::new(val))
    }

    /// Convenience constructor to create a [`BotCommandScope::ChatAdministrators`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_administrators(val: crate::types::BotCommandScopeChatAdministrators) -> Self {
        Self::ChatAdministrators(Box::new(val))
    }

    /// Convenience constructor to create a [`BotCommandScope::ChatMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_member(val: crate::types::BotCommandScopeChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }

}

/// Converts a [`crate::types::BotCommandScopeChat`] into [`BotCommandScope`].
impl From<crate::types::BotCommandScopeChat> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// Converts a [`crate::types::BotCommandScopeChatAdministrators`] into [`BotCommandScope`].
impl From<crate::types::BotCommandScopeChatAdministrators> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChatAdministrators) -> Self {
        Self::ChatAdministrators(Box::new(val))
    }
}

/// Converts a [`crate::types::BotCommandScopeChatMember`] into [`BotCommandScope`].
impl From<crate::types::BotCommandScopeChatMember> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }
}

