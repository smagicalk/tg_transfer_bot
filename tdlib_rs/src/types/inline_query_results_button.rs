#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a button to be shown above inline query results
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButton {
    /// The text of the button
    pub text: String,
    /// Type of the button
    pub r#type: crate::enums::InlineQueryResultsButtonType,
}
