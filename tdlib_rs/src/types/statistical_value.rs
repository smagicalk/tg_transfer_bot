#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A value with information about its recent changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalValue {
    /// The current value
    pub value: f64,
    /// The value for the previous day
    pub previous_value: f64,
    /// The growth rate of the value, as a percentage
    pub growth_rate_percentage: f64,
}
