#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The media is a photo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidMediaPhoto {
    /// The photo
    pub photo: crate::types::Photo,
}
