#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSelfDestructType {
    /// The message will be self-destructed in the specified time after its content was opened
    #[serde(rename(serialize = "messageSelfDestructTypeTimer", deserialize = "messageSelfDestructTypeTimer"))]
    Timer(crate::types::MessageSelfDestructTypeTimer),
    /// The message can be opened only once and will be self-destructed once closed
    #[serde(rename(serialize = "messageSelfDestructTypeImmediately", deserialize = "messageSelfDestructTypeImmediately"))]
    Immediately,
}
