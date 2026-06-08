#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CurrentWeather {
    /// Describes the current weather
    #[serde(rename(serialize = "currentWeather", deserialize = "currentWeather"))]
    CurrentWeather(crate::types::CurrentWeather),
}
