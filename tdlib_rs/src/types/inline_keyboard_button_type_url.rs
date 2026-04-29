#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that opens a specified URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeUrl {
    /// HTTP or tg: URL to open. If the link is of the type internalLinkTypeWebApp, then the button must be marked as a Web App button
    pub url: String,
}
