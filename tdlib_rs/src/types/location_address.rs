#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an address of a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LocationAddress {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable; empty if unknown
    pub state: String,
    /// City; empty if unknown
    pub city: String,
    /// The address; empty if unknown
    pub street: String,
}
