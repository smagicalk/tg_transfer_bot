#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceDataField {
    /// Field name
    pub field_name: String,
}
