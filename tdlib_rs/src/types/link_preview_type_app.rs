#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to an app at App Store or Google Play
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeApp {
    /// Photo for the app
    pub photo: crate::types::Photo,
}
