#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ButtonStyle {
    /// The button has default style
    #[serde(rename(serialize = "buttonStyleDefault", deserialize = "buttonStyleDefault"))]
    Default,
    /// The button has dark blue color
    #[serde(rename(serialize = "buttonStylePrimary", deserialize = "buttonStylePrimary"))]
    Primary,
    /// The button has red color
    #[serde(rename(serialize = "buttonStyleDanger", deserialize = "buttonStyleDanger"))]
    Danger,
    /// The button has green color
    #[serde(rename(serialize = "buttonStyleSuccess", deserialize = "buttonStyleSuccess"))]
    Success,
}
