#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user is uploading a photo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionUploadingPhoto {
    /// Upload progress, as a percentage
    pub progress: i32,
}
