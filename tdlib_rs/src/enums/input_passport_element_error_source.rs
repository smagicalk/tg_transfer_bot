#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElementErrorSource {
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename(serialize = "inputPassportElementErrorSourceUnspecified", deserialize = "inputPassportElementErrorSourceUnspecified"))]
    Unspecified(crate::types::InputPassportElementErrorSourceUnspecified),
    /// A data field contains an error. The error is considered resolved when the field's value changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceDataField", deserialize = "inputPassportElementErrorSourceDataField"))]
    DataField(crate::types::InputPassportElementErrorSourceDataField),
    /// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFrontSide", deserialize = "inputPassportElementErrorSourceFrontSide"))]
    FrontSide(crate::types::InputPassportElementErrorSourceFrontSide),
    /// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceReverseSide", deserialize = "inputPassportElementErrorSourceReverseSide"))]
    ReverseSide(crate::types::InputPassportElementErrorSourceReverseSide),
    /// The selfie contains an error. The error is considered resolved when the file with the selfie changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceSelfie", deserialize = "inputPassportElementErrorSourceSelfie"))]
    Selfie(crate::types::InputPassportElementErrorSourceSelfie),
    /// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceTranslationFile", deserialize = "inputPassportElementErrorSourceTranslationFile"))]
    TranslationFile(crate::types::InputPassportElementErrorSourceTranslationFile),
    /// The translation of the document contains an error. The error is considered resolved when the list of files changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceTranslationFiles", deserialize = "inputPassportElementErrorSourceTranslationFiles"))]
    TranslationFiles(crate::types::InputPassportElementErrorSourceTranslationFiles),
    /// The file contains an error. The error is considered resolved when the file changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFile", deserialize = "inputPassportElementErrorSourceFile"))]
    File(crate::types::InputPassportElementErrorSourceFile),
    /// The list of attached files contains an error. The error is considered resolved when the file list changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFiles", deserialize = "inputPassportElementErrorSourceFiles"))]
    Files(crate::types::InputPassportElementErrorSourceFiles),
}
