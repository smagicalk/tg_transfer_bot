#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The freeze state of the current user's account has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFreezeState {
    /// True, if the account is frozen
    pub is_frozen: bool,
    /// Point in time (Unix timestamp) when the account was frozen; 0 if the account isn't frozen
    pub freezing_date: i32,
    /// Point in time (Unix timestamp) when the account will be deleted and can't be unfrozen; 0 if the account isn't frozen
    pub deletion_date: i32,
    /// The link to open to send an appeal to unfreeze the account
    pub appeal_link: String,
}
