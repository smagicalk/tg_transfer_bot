#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a bot's answer to a callback query
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryAnswer {
    /// Text of the answer
    pub text: String,
    /// True, if an alert must be shown to the user instead of a toast notification
    pub show_alert: bool,
    /// URL to be opened
    pub url: String,
}
