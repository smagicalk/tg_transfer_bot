#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Audios {
    /// Contains a list of audio files
    #[serde(rename(serialize = "audios", deserialize = "audios"))]
    Audios(crate::types::Audios),
}
