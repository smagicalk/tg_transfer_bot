#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitRegistration {
    /// Telegram terms of service
    pub terms_of_service: crate::types::TermsOfService,
}
