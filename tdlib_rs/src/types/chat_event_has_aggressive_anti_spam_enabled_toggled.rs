#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The has_aggressive_anti_spam_enabled setting of a supergroup was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventHasAggressiveAntiSpamEnabledToggled {
    /// New value of has_aggressive_anti_spam_enabled
    pub has_aggressive_anti_spam_enabled: bool,
}
