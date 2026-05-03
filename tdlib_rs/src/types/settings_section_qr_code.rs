#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The current user's QR code section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionQrCode {
    /// Subsection of the section; may be one of
    /// "", "share", "scan"
    pub subsection: String,
}
