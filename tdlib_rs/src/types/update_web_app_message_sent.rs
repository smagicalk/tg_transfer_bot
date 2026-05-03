#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message was sent by an opened Web App, so the Web App needs to be closed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateWebAppMessageSent {
    /// Identifier of Web App launch
    #[serde_as(as = "DisplayFromStr")]
    pub web_app_launch_id: i64,
}
