#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a button to be shown instead of bot commands menu button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMenuButton {
    /// Text of the button
    pub text: String,
    /// URL of a Web App to open when the button is pressed. If the link is of the type internalLinkTypeWebApp, then it must be processed accordingly. Otherwise, the link must be passed to openWebApp
    pub url: String,
}
