#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A background from a local file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundLocal {
    /// Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns
    pub background: crate::enums::InputFile,
}
