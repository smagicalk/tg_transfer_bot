#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a Telegram Passport elements and corresponding errors
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementsWithErrors {
    /// Telegram Passport elements
    pub elements: Vec<crate::enums::PassportElement>,
    /// Errors in the elements that are already available
    pub errors: Vec<crate::types::PassportElementError>,
}
