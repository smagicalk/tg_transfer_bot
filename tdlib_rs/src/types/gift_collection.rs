#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes collection of gifts
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftCollection {
    /// Unique identifier of the collection
    pub id: i32,
    /// Name of the collection
    pub name: String,
    /// Icon of the collection; may be null if none
    pub icon: Option<crate::types::Sticker>,
    /// Total number of gifts in the collection
    pub gift_count: i32,
}
