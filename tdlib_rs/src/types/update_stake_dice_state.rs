#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The stake dice state has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStakeDiceState {
    /// The new state. The state can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
    pub state: crate::types::StakeDiceState,
}
