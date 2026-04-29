#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportReason {
    /// The chat contains spam messages
    #[serde(rename(serialize = "reportReasonSpam", deserialize = "reportReasonSpam"))]
    Spam,
    /// The chat promotes violence
    #[serde(rename(serialize = "reportReasonViolence", deserialize = "reportReasonViolence"))]
    Violence,
    /// The chat contains pornographic messages
    #[serde(rename(serialize = "reportReasonPornography", deserialize = "reportReasonPornography"))]
    Pornography,
    /// The chat has child abuse related content
    #[serde(rename(serialize = "reportReasonChildAbuse", deserialize = "reportReasonChildAbuse"))]
    ChildAbuse,
    /// The chat contains copyrighted content
    #[serde(rename(serialize = "reportReasonCopyright", deserialize = "reportReasonCopyright"))]
    Copyright,
    /// The location-based chat is unrelated to its stated location
    #[serde(rename(serialize = "reportReasonUnrelatedLocation", deserialize = "reportReasonUnrelatedLocation"))]
    UnrelatedLocation,
    /// The chat represents a fake account
    #[serde(rename(serialize = "reportReasonFake", deserialize = "reportReasonFake"))]
    Fake,
    /// The chat has illegal drugs related content
    #[serde(rename(serialize = "reportReasonIllegalDrugs", deserialize = "reportReasonIllegalDrugs"))]
    IllegalDrugs,
    /// The chat contains messages with personal details
    #[serde(rename(serialize = "reportReasonPersonalDetails", deserialize = "reportReasonPersonalDetails"))]
    PersonalDetails,
    /// A custom reason provided by the user
    #[serde(rename(serialize = "reportReasonCustom", deserialize = "reportReasonCustom"))]
    Custom,
}
