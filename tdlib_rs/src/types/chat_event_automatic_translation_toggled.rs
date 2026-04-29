#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The has_automatic_translation setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAutomaticTranslationToggled {
    /// New value of has_automatic_translation
    pub has_automatic_translation: bool,
}
