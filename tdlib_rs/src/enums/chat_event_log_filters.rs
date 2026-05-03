#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEventLogFilters {
    /// Represents a set of filters used to obtain a chat event log
    #[serde(rename(serialize = "chatEventLogFilters", deserialize = "chatEventLogFilters"))]
    ChatEventLogFilters(crate::types::ChatEventLogFilters),
}
