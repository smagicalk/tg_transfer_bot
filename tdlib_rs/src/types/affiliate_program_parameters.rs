#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes parameters of an affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramParameters {
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner;
    /// getOption("affiliate_program_commission_per_mille_min")-getOption("affiliate_program_commission_per_mille_max")
    pub commission_per_mille: i32,
    /// Number of months the program will be active; 0-36. If 0, then the program is eternal
    pub month_count: i32,
}
