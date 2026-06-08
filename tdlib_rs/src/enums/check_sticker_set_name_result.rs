#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CheckStickerSetNameResult {
    /// The name can be set
    #[serde(rename(
        serialize = "checkStickerSetNameResultOk",
        deserialize = "checkStickerSetNameResultOk"
    ))]
    Ok,
    /// The name is invalid
    #[serde(rename(
        serialize = "checkStickerSetNameResultNameInvalid",
        deserialize = "checkStickerSetNameResultNameInvalid"
    ))]
    NameInvalid,
    /// The name is occupied
    #[serde(rename(
        serialize = "checkStickerSetNameResultNameOccupied",
        deserialize = "checkStickerSetNameResultNameOccupied"
    ))]
    NameOccupied,
}
