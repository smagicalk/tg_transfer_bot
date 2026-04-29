#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResultsButtonType {
    /// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
    #[serde(rename(serialize = "inlineQueryResultsButtonTypeStartBot", deserialize = "inlineQueryResultsButtonTypeStartBot"))]
    StartBot(crate::types::InlineQueryResultsButtonTypeStartBot),
    /// Describes the button that opens a Web App by calling getWebAppUrl
    #[serde(rename(serialize = "inlineQueryResultsButtonTypeWebApp", deserialize = "inlineQueryResultsButtonTypeWebApp"))]
    WebApp(crate::types::InlineQueryResultsButtonTypeWebApp),
}
