#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageEffect {
    /// Contains information about an effect added to a message
    #[serde(rename(serialize = "messageEffect", deserialize = "messageEffect"))]
    MessageEffect(crate::types::MessageEffect),
}
