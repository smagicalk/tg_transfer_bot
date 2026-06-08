#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A filled background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: crate::enums::BackgroundFill,
}
