#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a Web App found by its short name
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FoundWebApp {
    /// The Web App
    pub web_app: crate::types::WebApp,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
    /// True, if there is no need to show an ordinary open URL confirmation before opening the Web App. The field must be ignored and confirmation must be shown anyway if the Web App link was hidden
    pub skip_confirmation: bool,
}
