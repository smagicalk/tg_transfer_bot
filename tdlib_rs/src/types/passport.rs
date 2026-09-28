//!
//! TDLib `passport` domain types.
//!
//! Types, enums, and functions for Telegram Passport and identity verification documents.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element containing the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypePersonalDetails {
}

/// A Telegram Passport element containing the user's passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypePassport {
}

/// A Telegram Passport element containing the user's driver license
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeDriverLicense {
}

/// A Telegram Passport element containing the user's identity card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeIdentityCard {
}

/// A Telegram Passport element containing the user's internal passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeInternalPassport {
}

/// A Telegram Passport element containing the user's address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeAddress {
}

/// A Telegram Passport element containing the user's utility bill
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeUtilityBill {
}

/// A Telegram Passport element containing the user's bank statement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeBankStatement {
}

/// A Telegram Passport element containing the user's rental agreement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeRentalAgreement {
}

/// A Telegram Passport element containing the registration page of the user's passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypePassportRegistration {
}

/// A Telegram Passport element containing the user's temporary registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeTemporaryRegistration {
}

/// A Telegram Passport element containing the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypePhoneNumber {
}

/// A Telegram Passport element containing the user's email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTypeEmailAddress {
}

/// A Telegram Passport element containing the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPersonalDetails {
    /// Personal details of the user
    pub personal_details: crate::types::PersonalDetails,
}

/// A Telegram Passport element containing the user's passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPassport {
    /// Passport
    pub passport: crate::types::IdentityDocument,
}

/// A Telegram Passport element containing the user's driver license
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementDriverLicense {
    /// Driver license
    pub driver_license: crate::types::IdentityDocument,
}

/// A Telegram Passport element containing the user's identity card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementIdentityCard {
    /// Identity card
    pub identity_card: crate::types::IdentityDocument,
}

/// A Telegram Passport element containing the user's internal passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementInternalPassport {
    /// Internal passport
    pub internal_passport: crate::types::IdentityDocument,
}

/// A Telegram Passport element containing the user's address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementAddress {
    /// Address
    pub address: crate::types::Address,
}

/// A Telegram Passport element containing the user's utility bill
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementUtilityBill {
    /// Utility bill
    pub utility_bill: crate::types::PersonalDocument,
}

/// A Telegram Passport element containing the user's bank statement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementBankStatement {
    /// Bank statement
    pub bank_statement: crate::types::PersonalDocument,
}

/// A Telegram Passport element containing the user's rental agreement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementRentalAgreement {
    /// Rental agreement
    pub rental_agreement: crate::types::PersonalDocument,
}

/// A Telegram Passport element containing the user's passport registration pages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPassportRegistration {
    /// Passport registration pages
    pub passport_registration: crate::types::PersonalDocument,
}

/// A Telegram Passport element containing the user's temporary registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTemporaryRegistration {
    /// Temporary registration
    pub temporary_registration: crate::types::PersonalDocument,
}

/// A Telegram Passport element containing the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPhoneNumber {
    /// Phone number
    pub phone_number: String,
}

/// A Telegram Passport element containing the user's email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementEmailAddress {
    /// Email address
    pub email_address: String,
}

/// A Telegram Passport element to be saved containing the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPersonalDetails {
    /// Personal details of the user
    pub personal_details: crate::types::PersonalDetails,
}

/// A Telegram Passport element to be saved containing the user's passport
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPassport {
    /// The passport to be saved
    pub passport: crate::types::InputIdentityDocument,
}

/// A Telegram Passport element to be saved containing the user's driver license
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementDriverLicense {
    /// The driver license to be saved
    pub driver_license: crate::types::InputIdentityDocument,
}

/// A Telegram Passport element to be saved containing the user's identity card
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementIdentityCard {
    /// The identity card to be saved
    pub identity_card: crate::types::InputIdentityDocument,
}

/// A Telegram Passport element to be saved containing the user's internal passport
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementInternalPassport {
    /// The internal passport to be saved
    pub internal_passport: crate::types::InputIdentityDocument,
}

/// A Telegram Passport element to be saved containing the user's address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementAddress {
    /// The address to be saved
    pub address: crate::types::Address,
}

/// A Telegram Passport element to be saved containing the user's utility bill
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementUtilityBill {
    /// The utility bill to be saved
    pub utility_bill: crate::types::InputPersonalDocument,
}

/// A Telegram Passport element to be saved containing the user's bank statement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementBankStatement {
    /// The bank statement to be saved
    pub bank_statement: crate::types::InputPersonalDocument,
}

/// A Telegram Passport element to be saved containing the user's rental agreement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementRentalAgreement {
    /// The rental agreement to be saved
    pub rental_agreement: crate::types::InputPersonalDocument,
}

/// A Telegram Passport element to be saved containing the user's passport registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPassportRegistration {
    /// The passport registration page to be saved
    pub passport_registration: crate::types::InputPersonalDocument,
}

/// A Telegram Passport element to be saved containing the user's temporary registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementTemporaryRegistration {
    /// The temporary registration document to be saved
    pub temporary_registration: crate::types::InputPersonalDocument,
}

/// A Telegram Passport element to be saved containing the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPhoneNumber {
    /// The phone number to be saved
    pub phone_number: String,
}

/// A Telegram Passport element to be saved containing the user's email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementEmailAddress {
    /// The email address to be saved
    pub email_address: String,
}

/// Contains information about saved Telegram Passport elements
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElements {
    /// Telegram Passport elements
    pub elements: Vec<crate::enums::PassportElement>,
}

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceUnspecified {
}

/// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceDataField {
    /// Field name
    pub field_name: String,
}

/// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceFrontSide {
}

/// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceReverseSide {
}

/// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceSelfie {
}

/// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceTranslationFile {
    /// Index of a file with the error
    pub file_index: i32,
}

/// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceTranslationFiles {
}

/// The file contains an error. The error will be considered resolved when the file changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceFile {
    /// Index of a file with the error
    pub file_index: i32,
}

/// The list of attached files contains an error. The error will be considered resolved when the list of files changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceFiles {
}

/// Contains the description of an error in a Telegram Passport element
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PassportElementError {
    /// Type of the Telegram Passport element which has the error
    #[serde(rename = "type")]
    pub r#type: crate::enums::PassportElementType,
    /// Error message
    pub message: String,
    /// Error source
    pub source: crate::enums::PassportElementErrorSource,
}

/// Contains information about a Telegram Passport element that was requested by a service
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PassportSuitableElement {
    /// Type of the element
    #[serde(rename = "type")]
    pub r#type: crate::enums::PassportElementType,
    /// True, if a selfie is required with the identity document
    pub is_selfie_required: bool,
    /// True, if a certified English translation is required with the document
    pub is_translation_required: bool,
    /// True, if personal details must include the user's name in the language of their country of residence
    pub is_native_name_required: bool,
}

/// Contains a description of the required Telegram Passport element that was requested by a service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportRequiredElement {
    /// List of Telegram Passport elements any of which is enough to provide
    pub suitable_elements: Vec<crate::types::PassportSuitableElement>,
}

/// Contains information about a Telegram Passport authorization form that was requested
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportAuthorizationForm {
    /// Unique identifier of the authorization form
    pub id: i32,
    /// Telegram Passport elements that must be provided to complete the form
    pub required_elements: Vec<crate::types::PassportRequiredElement>,
    /// URL for the privacy policy of the service; may be empty
    pub privacy_policy_url: String,
}

/// Contains information about a Telegram Passport elements and corresponding errors
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementsWithErrors {
    /// Telegram Passport elements
    pub elements: Vec<crate::enums::PassportElement>,
    /// Errors in the elements that are already available
    pub errors: Vec<crate::types::PassportElementError>,
}

/// Contains information about an encrypted Telegram Passport element; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptedPassportElement {
    /// Type of Telegram Passport element
    #[serde(rename = "type")]
    pub r#type: crate::enums::PassportElementType,
    /// Encrypted JSON-encoded data about the user
    pub data: String,
    /// The front side of an identity document
    pub front_side: crate::types::DatedFile,
    /// The reverse side of an identity document; may be null
    pub reverse_side: Option<crate::types::DatedFile>,
    /// Selfie with the document; may be null
    pub selfie: Option<crate::types::DatedFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::types::DatedFile>,
    /// List of attached files
    pub files: Vec<crate::types::DatedFile>,
    /// Unencrypted data, phone number or email address
    pub value: String,
    /// Hash of the entire element
    pub hash: String,
}

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceUnspecified {
    /// Current hash of the entire element
    pub element_hash: String,
}

/// A data field contains an error. The error is considered resolved when the field's value changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceDataField {
    /// Field name
    pub field_name: String,
    /// Current data hash
    pub data_hash: String,
}

/// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFrontSide {
    /// Current hash of the file containing the front side
    pub file_hash: String,
}

/// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceReverseSide {
    /// Current hash of the file containing the reverse side
    pub file_hash: String,
}

/// The selfie contains an error. The error is considered resolved when the file with the selfie changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceSelfie {
    /// Current hash of the file containing the selfie
    pub file_hash: String,
}

/// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceTranslationFile {
    /// Current hash of the file containing the translation
    pub file_hash: String,
}

/// The translation of the document contains an error. The error is considered resolved when the list of files changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceTranslationFiles {
    /// Current hashes of all files with the translation
    pub file_hashes: Vec<String>,
}

/// The file contains an error. The error is considered resolved when the file changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFile {
    /// Current hash of the file which has the error
    pub file_hash: String,
}

/// The list of attached files contains an error. The error is considered resolved when the file list changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFiles {
    /// Current hashes of all attached files
    pub file_hashes: Vec<String>,
}

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementError {
    /// Type of Telegram Passport element that has the error
    #[serde(rename = "type")]
    pub r#type: crate::enums::PassportElementType,
    /// Error message
    pub message: String,
    /// Error source
    pub source: crate::enums::InputPassportElementErrorSource,
}

/// Telegram Passport data has been sent to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePassportDataSent {
    /// List of Telegram Passport element types sent
    pub types: Vec<crate::enums::PassportElementType>,
}

/// Telegram Passport data has been received; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePassportDataReceived {
    /// List of received Telegram Passport elements
    pub elements: Vec<crate::types::EncryptedPassportElement>,
    /// Encrypted data credentials
    pub credentials: crate::types::EncryptedCredentials,
}

/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePassportDataRequest {
    /// User identifier of the service's bot; the corresponding user may be unknown yet
    pub bot_user_id: i64,
    /// Telegram Passport element types requested by the service
    pub scope: String,
    /// Service's public key
    pub public_key: String,
    /// Unique request identifier provided by the service
    pub nonce: String,
    /// An HTTP URL to open once the request is finished, canceled, or failed with the parameters tg_passport=success, tg_passport=cancel, or tg_passport=error&error=... respectively.
    /// If empty, then onActivityResult method must be used to return response on Android, or the link tgbot{bot_user_id}:passport/success or tgbot{bot_user_id}:passport/cancel must be opened otherwise
    pub callback_url: String,
}

