#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a background. Link preview title and description are available only for filled backgrounds
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeBackground {
    /// Document with the background; may be null for filled backgrounds
    pub document: Option<crate::types::Document>,
    /// Type of the background; may be null if unknown
    pub background_type: Option<crate::enums::BackgroundType>,
}
