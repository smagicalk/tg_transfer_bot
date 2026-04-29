#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A birthdate was suggested to be set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestBirthdate {
    /// The suggested birthdate. Use the method setBirthdate to apply the birthdate
    pub birthdate: crate::types::Birthdate,
}
