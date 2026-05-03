#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of gift collections
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftCollections {
    /// List of gift collections
    pub collections: Vec<crate::types::GiftCollection>,
}
