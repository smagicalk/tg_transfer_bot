#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a country
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CountryInfo {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// Native name of the country
    pub name: String,
    /// English name of the country
    pub english_name: String,
    /// True, if the country must be hidden from the list of all countries
    pub is_hidden: bool,
    /// List of country calling codes
    pub calling_codes: Vec<String>,
}
