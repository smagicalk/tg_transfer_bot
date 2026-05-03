#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of t.me URLs
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrls {
    /// List of URLs
    pub urls: Vec<crate::types::TmeUrl>,
}
