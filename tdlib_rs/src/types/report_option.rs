#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an option to report an entity to Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportOption {
    /// Unique identifier of the option
    pub id: String,
    /// Text of the option
    pub text: String,
}
