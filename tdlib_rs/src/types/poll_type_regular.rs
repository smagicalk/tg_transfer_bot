#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A regular poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollTypeRegular {
    /// True, if multiple answer options can be chosen simultaneously
    pub allow_multiple_answers: bool,
}
