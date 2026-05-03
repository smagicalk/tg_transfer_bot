#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about countries
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Countries {
    /// The list of countries
    pub countries: Vec<crate::types::CountryInfo>,
}
