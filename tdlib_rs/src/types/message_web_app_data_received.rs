#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Data from a Web App has been received; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageWebAppDataReceived {
    /// Text of the keyboardButtonTypeWebApp button, which opened the Web App
    pub button_text: String,
    /// The data
    pub data: String,
}
