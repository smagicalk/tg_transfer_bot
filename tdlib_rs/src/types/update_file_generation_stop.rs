#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// File generation is no longer needed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFileGenerationStop {
    /// Unique identifier for the generation process
    #[serde_as(as = "DisplayFromStr")]
    pub generation_id: i64,
}
