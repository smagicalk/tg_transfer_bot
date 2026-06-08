#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains some text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Text {
    /// Text
    pub text: String,
}
