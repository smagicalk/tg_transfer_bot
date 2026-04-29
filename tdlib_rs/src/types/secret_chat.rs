#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a secret chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SecretChat {
    /// Secret chat identifier
    pub id: i32,
    /// Identifier of the chat partner
    pub user_id: i64,
    /// State of the secret chat
    pub state: crate::enums::SecretChatState,
    /// True, if the chat was created by the current user; false otherwise
    pub is_outbound: bool,
    /// Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 little-endian bytes, which must be split into groups of 2 bits, each denoting a pixel of one of 4 colors FFFFFF, D5E6F3, 2D5775, and 2F99C9.
    /// The pixels must be used to make a 12x12 square image filled from left to right, top to bottom. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers
    pub key_hash: String,
    /// Secret chat layer; determines features supported by the chat partner's application. Nested text entities and underline and strikethrough entities are supported if the layer >= 101,
    /// files bigger than 2000MB are supported if the layer >= 143, spoiler and custom emoji text entities are supported if the layer >= 144
    pub layer: i32,
}
