#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a part of the text that needs to be formatted in some unusual way
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TextEntity {
    /// Offset of the entity, in UTF-16 code units
    pub offset: i32,
    /// Length of the entity, in UTF-16 code units
    pub length: i32,
    /// Type of the entity
    pub r#type: crate::enums::TextEntityType,
}
