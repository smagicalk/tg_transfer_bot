#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents the result of an importContacts request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ImportedContacts {
    /// User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user
    pub user_ids: Vec<i64>,
    /// The number of users that imported the corresponding contact; 0 for already registered users or if unavailable
    pub importer_count: Vec<i32>,
}
