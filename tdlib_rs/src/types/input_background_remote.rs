#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A background from the server
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundRemote {
    /// The background identifier
    #[serde_as(as = "DisplayFromStr")]
    pub background_id: i64,
}
