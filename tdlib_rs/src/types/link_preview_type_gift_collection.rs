#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a gift collection
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeGiftCollection {
    /// Icons for some gifts from the collection; may be empty
    pub icons: Vec<crate::types::Sticker>,
}
