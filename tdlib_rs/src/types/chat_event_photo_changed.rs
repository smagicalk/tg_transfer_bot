#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat photo was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventPhotoChanged {
    /// Previous chat photo value; may be null
    pub old_photo: Option<crate::types::ChatPhoto>,
    /// New chat photo value; may be null
    pub new_photo: Option<crate::types::ChatPhoto>,
}
