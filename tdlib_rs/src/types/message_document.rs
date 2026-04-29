#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A document message (general file)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageDocument {
    /// The document description
    pub document: crate::types::Document,
    /// Document caption
    pub caption: crate::types::FormattedText,
}
