#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about saved payment credentials
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedCredentials {
    /// Unique identifier of the saved credentials
    pub id: String,
    /// Title of the saved credentials
    pub title: String,
}
