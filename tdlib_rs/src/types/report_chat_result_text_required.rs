#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user must add additional text details to the report
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultTextRequired {
    /// Option identifier for the next reportChat request
    pub option_id: String,
    /// True, if the user can skip text adding
    pub is_optional: bool,
}
