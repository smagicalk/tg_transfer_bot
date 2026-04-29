#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a tg: deep link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeepLinkInfo {
    /// Text to be shown to the user
    pub text: crate::types::FormattedText,
    /// True, if the user must be asked to update the application
    pub need_update_application: bool,
}
