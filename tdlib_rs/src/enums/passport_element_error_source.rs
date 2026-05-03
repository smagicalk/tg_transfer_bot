#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementErrorSource {
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename(
        serialize = "passportElementErrorSourceUnspecified",
        deserialize = "passportElementErrorSourceUnspecified"
    ))]
    Unspecified,
    /// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
    #[serde(rename(
        serialize = "passportElementErrorSourceDataField",
        deserialize = "passportElementErrorSourceDataField"
    ))]
    DataField(crate::types::PassportElementErrorSourceDataField),
    /// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
    #[serde(rename(
        serialize = "passportElementErrorSourceFrontSide",
        deserialize = "passportElementErrorSourceFrontSide"
    ))]
    FrontSide,
    /// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
    #[serde(rename(
        serialize = "passportElementErrorSourceReverseSide",
        deserialize = "passportElementErrorSourceReverseSide"
    ))]
    ReverseSide,
    /// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
    #[serde(rename(
        serialize = "passportElementErrorSourceSelfie",
        deserialize = "passportElementErrorSourceSelfie"
    ))]
    Selfie,
    /// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
    #[serde(rename(
        serialize = "passportElementErrorSourceTranslationFile",
        deserialize = "passportElementErrorSourceTranslationFile"
    ))]
    TranslationFile(crate::types::PassportElementErrorSourceTranslationFile),
    /// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
    #[serde(rename(
        serialize = "passportElementErrorSourceTranslationFiles",
        deserialize = "passportElementErrorSourceTranslationFiles"
    ))]
    TranslationFiles,
    /// The file contains an error. The error will be considered resolved when the file changes
    #[serde(rename(
        serialize = "passportElementErrorSourceFile",
        deserialize = "passportElementErrorSourceFile"
    ))]
    File(crate::types::PassportElementErrorSourceFile),
    /// The list of attached files contains an error. The error will be considered resolved when the list of files changes
    #[serde(rename(
        serialize = "passportElementErrorSourceFiles",
        deserialize = "passportElementErrorSourceFiles"
    ))]
    Files,
}
