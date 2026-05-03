#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Address {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line of the address
    pub street_line1: String,
    /// Second line of the address
    pub street_line2: String,
    /// Address postal code
    pub postal_code: String,
}
