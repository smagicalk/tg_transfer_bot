#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionSetLoginEmailAddress {
    /// True, if the suggested action can be hidden using hideSuggestedAction. Otherwise, the user must not be able to use the app without setting up the email address
    pub can_be_hidden: bool,
}
