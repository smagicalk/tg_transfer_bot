#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotWriteAccessAllowReason {
    /// The user connected a website by logging in using Telegram Login Widget on it
    #[serde(rename(serialize = "botWriteAccessAllowReasonConnectedWebsite", deserialize = "botWriteAccessAllowReasonConnectedWebsite"))]
    ConnectedWebsite(crate::types::BotWriteAccessAllowReasonConnectedWebsite),
    /// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
    #[serde(rename(serialize = "botWriteAccessAllowReasonAddedToAttachmentMenu", deserialize = "botWriteAccessAllowReasonAddedToAttachmentMenu"))]
    AddedToAttachmentMenu,
    /// The user launched a Web App using getWebAppLinkUrl
    #[serde(rename(serialize = "botWriteAccessAllowReasonLaunchedWebApp", deserialize = "botWriteAccessAllowReasonLaunchedWebApp"))]
    LaunchedWebApp(crate::types::BotWriteAccessAllowReasonLaunchedWebApp),
    /// The user accepted bot's request to send messages with allowBotToSendMessages
    #[serde(rename(serialize = "botWriteAccessAllowReasonAcceptedRequest", deserialize = "botWriteAccessAllowReasonAcceptedRequest"))]
    AcceptedRequest,
}
