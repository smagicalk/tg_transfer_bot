#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A button that opens a Web App by calling openWebApp
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeWebApp {
    /// An HTTP URL to pass to openWebApp
    pub url: String,
}
