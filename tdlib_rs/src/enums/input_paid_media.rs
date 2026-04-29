#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPaidMedia {
    /// Describes a paid media to be sent
    #[serde(rename(serialize = "inputPaidMedia", deserialize = "inputPaidMedia"))]
    InputPaidMedia(crate::types::InputPaidMedia),
}
