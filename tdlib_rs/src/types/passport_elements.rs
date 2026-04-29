#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about saved Telegram Passport elements
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElements {
    /// Telegram Passport elements
    pub elements: Vec<crate::enums::PassportElement>,
}
