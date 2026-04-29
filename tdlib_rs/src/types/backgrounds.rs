#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of backgrounds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Backgrounds {
    /// A list of backgrounds
    pub backgrounds: Vec<crate::types::Background>,
}
