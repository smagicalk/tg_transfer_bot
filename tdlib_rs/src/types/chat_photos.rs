#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat or user profile photos
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotos {
    /// Total number of photos
    pub total_count: i32,
    /// List of photos
    pub photos: Vec<crate::types::ChatPhoto>,
}
