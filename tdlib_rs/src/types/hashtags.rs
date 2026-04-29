#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of hashtags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Hashtags {
    /// A list of hashtags
    pub hashtags: Vec<String>,
}
