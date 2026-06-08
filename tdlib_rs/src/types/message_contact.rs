#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with a user contact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageContact {
    /// The contact description
    pub contact: crate::types::Contact,
}
