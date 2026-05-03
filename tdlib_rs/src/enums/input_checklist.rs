#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChecklist {
    /// Describes a checklist to be sent
    #[serde(rename(serialize = "inputChecklist", deserialize = "inputChecklist"))]
    InputChecklist(crate::types::InputChecklist),
}
