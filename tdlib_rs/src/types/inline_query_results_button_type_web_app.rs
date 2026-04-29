#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes the button that opens a Web App by calling getWebAppUrl
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButtonTypeWebApp {
    /// An HTTP URL to pass to getWebAppUrl
    pub url: String,
}
