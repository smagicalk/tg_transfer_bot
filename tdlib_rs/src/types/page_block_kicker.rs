#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A kicker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockKicker {
    /// Kicker
    pub kicker: crate::enums::RichText,
}
