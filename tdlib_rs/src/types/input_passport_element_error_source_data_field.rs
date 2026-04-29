#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A data field contains an error. The error is considered resolved when the field's value changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceDataField {
    /// Field name
    pub field_name: String,
    /// Current data hash
    pub data_hash: String,
}
