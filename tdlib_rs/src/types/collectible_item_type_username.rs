#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A username
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemTypeUsername {
    /// The username
    pub username: String,
}
