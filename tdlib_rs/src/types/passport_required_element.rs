#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a description of the required Telegram Passport element that was requested by a service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportRequiredElement {
    /// List of Telegram Passport elements any of which is enough to provide
    pub suitable_elements: Vec<crate::types::PassportSuitableElement>,
}
