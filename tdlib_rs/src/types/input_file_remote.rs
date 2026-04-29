#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A file defined by its remote identifier. The remote identifier is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib.
/// For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputFileRemote {
    /// Remote file identifier
    pub id: String,
}
