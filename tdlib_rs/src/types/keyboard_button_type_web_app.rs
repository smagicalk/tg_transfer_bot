#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that opens a Web App by calling getWebAppUrl
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeWebApp {
    /// An HTTP URL to pass to getWebAppUrl
    pub url: String,
}
