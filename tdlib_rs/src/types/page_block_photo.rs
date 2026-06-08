#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A photo
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockPhoto {
    /// Photo file; may be null
    pub photo: Option<crate::types::Photo>,
    /// Photo caption
    pub caption: crate::types::PageBlockCaption,
    /// URL that needs to be opened when the photo is clicked
    pub url: String,
}
