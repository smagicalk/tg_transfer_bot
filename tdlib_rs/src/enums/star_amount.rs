#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarAmount {
    /// Describes a possibly non-integer Telegram Star amount
    #[serde(rename(serialize = "starAmount", deserialize = "starAmount"))]
    StarAmount(crate::types::StarAmount),
}
