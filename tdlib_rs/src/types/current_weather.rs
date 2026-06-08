#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes the current weather
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CurrentWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
}
