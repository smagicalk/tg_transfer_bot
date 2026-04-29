#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An updated chat photo
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageChatChangePhoto {
    /// New chat photo
    pub photo: crate::types::ChatPhoto,
}
