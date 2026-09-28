//!
//! TDLib `misc` domain types.
//!
//! Miscellaneous types, enums, network settings, and core TDLib options.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An object of this type can be returned on every function call, in case of an error
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Error {
    /// Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
    pub code: i32,
    /// Error message; subject to future changes
    pub message: String,
}

/// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSms {
    /// Length of the code
    pub length: i32,
}

/// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSmsWord {
    /// The first letters of the word if known
    pub first_letter: String,
}

/// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSmsPhrase {
    /// The first word of the phrase if known
    pub first_word: String,
}

/// A digit-only authentication code is delivered to https:fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFragment {
    /// URL to open to receive the code
    pub url: String,
    /// Length of the code
    pub length: i32,
}

/// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFirebaseAndroid {
    /// Parameters to be used for device verification
    pub device_verification_parameters: crate::enums::FirebaseDeviceVerificationParameters,
    /// Length of the code
    pub length: i32,
}

/// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFirebaseIos {
    /// Receipt of successful application token validation to compare with receipt from push notification
    pub receipt: String,
    /// Time after the next authentication method is expected to be used if verification push notification isn't received, in seconds
    pub push_timeout: i32,
    /// Length of the code
    pub length: i32,
}

/// Information about the authentication code that was sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeInfo {
    /// A phone number that is being authenticated
    pub phone_number: String,
    /// The way the code was sent to the user
    #[serde(rename = "type")]
    pub r#type: crate::enums::AuthenticationCodeType,
    /// The way the next code will be sent to the user; may be null
    pub next_type: Option<crate::enums::AuthenticationCodeType>,
    /// Timeout before the code can be re-sent, in seconds
    pub timeout: i32,
}

/// Information about the email address authentication code that was sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationCodeInfo {
    /// Pattern of the email address to which an authentication code was sent
    pub email_address_pattern: String,
    /// Length of the code; 0 if unknown
    pub length: i32,
}

/// An authentication code delivered to a user's email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationCode {
    /// The code
    pub code: String,
}

/// An authentication token received through Apple ID
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationAppleId {
    /// The token
    pub token: String,
}

/// An authentication token received through Google ID
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationGoogleId {
    /// The token
    pub token: String,
}

/// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressResetStateAvailable {
    /// Time required to wait before the email address can be reset; 0 if the user is subscribed to Telegram Premium
    pub wait_period: i32,
}

/// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressResetStatePending {
    /// Left time before the email address will be reset, in seconds. updateAuthorizationState is not sent when this field changes
    pub reset_in: i32,
}

/// Represents a change of a text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffEntity {
    /// Offset of the entity, in UTF-16 code units
    pub offset: i32,
    /// Length of the entity, in UTF-16 code units
    pub length: i32,
    /// Type of the entity
    #[serde(rename = "type")]
    pub r#type: crate::enums::DiffEntityType,
}

/// Contains Telegram terms of service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TermsOfService {
    /// Text of the terms of service
    pub text: crate::types::FormattedText,
    /// The minimum age of a user to be able to accept the terms; 0 if age isn't restricted
    pub min_user_age: i32,
    /// True, if a blocking popup with terms of service must be shown to the user
    pub show_popup: bool,
}

/// Describes a passkey
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Passkey {
    /// Unique identifier of the passkey
    pub id: String,
    /// Name of the passkey
    pub name: String,
    /// Point in time (Unix timestamp) when the passkey was added
    pub addition_date: i32,
    /// Point in time (Unix timestamp) when the passkey was used last time; 0 if never
    pub last_usage_date: i32,
    /// Identifier of the custom emoji that is used as the icon of the software that created the passkey; 0 if unknown
    #[serde_as(as = "DisplayFromStr")]
    pub software_icon_custom_emoji_id: i64,
}

/// Contains a list of passkeys
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Passkeys {
    /// List of passkeys
    pub passkeys: Vec<crate::types::Passkey>,
}

/// Initialization parameters are needed. Call setTdlibParameters to provide them
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitTdlibParameters {
}

/// TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number,
/// or use requestQrCodeAuthentication, getAuthenticationPasskeyParameters, checkAuthenticationWebToken, or checkAuthenticationBotToken for other authentication options
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitPhoneNumber {
}

/// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitEmailAddress {
    /// True, if authorization through Apple ID is allowed
    pub allow_apple_id: bool,
    /// True, if authorization through Google ID is allowed
    pub allow_google_id: bool,
}

/// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitEmailCode {
    /// True, if authorization through Apple ID is allowed
    pub allow_apple_id: bool,
    /// True, if authorization through Google ID is allowed
    pub allow_google_id: bool,
    /// Information about the sent authentication code
    pub code_info: crate::types::EmailAddressAuthenticationCodeInfo,
    /// Reset state of the email address; may be null if the email address can't be reset
    pub email_address_reset_state: Option<crate::enums::EmailAddressResetState>,
}

/// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitCode {
    /// Information about the authorization code that was sent
    pub code_info: crate::types::AuthenticationCodeInfo,
}

/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
    /// A tg: URL for the QR code. The link will be updated frequently
    pub link: String,
}

/// The user is unregistered and needs to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitRegistration {
    /// Telegram terms of service
    pub terms_of_service: crate::types::TermsOfService,
}

/// The user has been authorized, but needs to enter a 2-step verification password to start using the application.
/// Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitPassword {
    /// Hint for the password; may be empty
    pub password_hint: String,
    /// True, if a recovery email address has been set up
    pub has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved
    pub has_passport_data: bool,
    /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
    pub recovery_email_address_pattern: String,
}

/// The user has been successfully authorized. TDLib is now ready to answer general requests
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateReady {
}

/// The user is currently logging out
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateLoggingOut {
}

/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateClosing {
}

/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to
/// with error code 500. To continue working, one must create a new instance of the TDLib client
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateClosed {
}

/// Device verification must be performed with the SafetyNet Attestation API
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseDeviceVerificationParametersSafetyNet {
    /// Nonce to pass to the SafetyNet Attestation API
    pub nonce: String,
}

/// Device verification must be performed with the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic)
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseDeviceVerificationParametersPlayIntegrity {
    /// Base64url-encoded nonce to pass to the Play Integrity API
    pub nonce: String,
    /// Cloud project number to pass to the Play Integrity API
    #[serde_as(as = "DisplayFromStr")]
    pub cloud_project_number: i64,
}

/// Represents the current state of 2-step verification
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PasswordState {
    /// True, if a 2-step verification password is set
    pub has_password: bool,
    /// Hint for the password; may be empty
    pub password_hint: String,
    /// True, if a recovery email is set
    pub has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved
    pub has_passport_data: bool,
    /// Information about the recovery email address to which the confirmation email was sent; may be null
    pub recovery_email_address_code_info: Option<crate::types::EmailAddressAuthenticationCodeInfo>,
    /// Pattern of the email address set up for logging in
    pub login_email_address_pattern: String,
    /// If not 0, point in time (Unix timestamp) after which the 2-step verification password can be reset immediately using resetPassword
    pub pending_reset_date: i32,
}

/// Contains information about the current recovery email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecoveryEmailAddress {
    /// Recovery email address
    pub recovery_email_address: String,
}

/// Returns information about the availability of a temporary password, which can be used for payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TemporaryPasswordState {
    /// True, if a temporary password is available
    pub has_password: bool,
    /// Time left before the temporary password expires, in seconds
    pub valid_for: i32,
}

/// The mask is placed relatively to the forehead
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MaskPointForehead {
}

/// The mask is placed relatively to the eyes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MaskPointEyes {
}

/// The mask is placed relatively to the mouth
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MaskPointMouth {
}

/// The mask is placed relatively to the chin
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MaskPointChin {
}

/// Position on a photo where a mask is placed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MaskPosition {
    /// Part of the face, relative to which the mask is placed
    pub point: crate::enums::MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position)
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)
    pub y_shift: f64,
    /// Mask scaling coefficient. (For example, 2.0 means a doubled size)
    pub scale: f64,
}

/// Represents a closed vector path. The path begins at the end point of the last command. The coordinate system origin is in the upper-left corner
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ClosedVectorPath {
    /// List of vector path commands
    pub commands: Vec<crate::enums::VectorPathCommand>,
}

/// Represents outline of an image
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Outline {
    /// The list of closed vector paths
    pub paths: Vec<crate::types::ClosedVectorPath>,
}

/// Describes a task in a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i32,
    /// Text of the task; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, DateTime and automatically found entities
    pub text: crate::types::FormattedText,
    /// Identifier of the user or chat that completed the task; may be null if the task isn't completed yet
    pub completed_by: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task isn't completed
    pub completion_date: i32,
}

/// Describes a task in a checklist to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive
    pub id: i32,
    /// Text of the task; 1-getOption("checklist_task_text_length_max") characters without line feeds. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
    pub text: crate::types::FormattedText,
}

/// Describes a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Checklist {
    /// Title of the checklist; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
    pub title: crate::types::FormattedText,
    /// List of tasks in the checklist
    pub tasks: Vec<crate::types::ChecklistTask>,
    /// True, if users other than creator of the list can add tasks to the list
    pub others_can_add_tasks: bool,
    /// True, if the current user can add tasks to the list if they have Telegram Premium subscription
    pub can_add_tasks: bool,
    /// True, if users other than creator of the list can mark tasks as done or not done. If true, then the checklist is called "group checklist"
    pub others_can_mark_tasks_as_done: bool,
    /// True, if the current user can mark tasks as done or not done if they have Telegram Premium subscription
    pub can_mark_tasks_as_done: bool,
}

/// Describes a checklist to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-getOption("checklist_title_length_max") characters. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
    pub title: crate::types::FormattedText,
    /// List of tasks in the checklist; 1-getOption("checklist_task_count_max") tasks
    pub tasks: Vec<crate::types::InputChecklistTask>,
    /// True, if other users can add tasks to the list
    pub others_can_add_tasks: bool,
    /// True, if other users can mark tasks as done or not done
    pub others_can_mark_tasks_as_done: bool,
}

/// Describes a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VoiceNote {
    /// Duration of the voice note, in seconds; as defined by the sender
    pub duration: i32,
    /// A waveform representation of the voice note in 5-bit format
    pub waveform: String,
    /// MIME type of the file; as defined by the sender. Usually, one of "audio/ogg" for Opus in an OGG container, "audio/mpeg" for an MP3 audio, or "audio/mp4" for an M4A audio
    pub mime_type: String,
    /// Result of speech recognition in the voice note; may be null
    pub speech_recognition_result: Option<crate::enums::SpeechRecognitionResult>,
    /// File containing the voice note
    pub voice: crate::types::File,
}

/// Describes a location on planet Earth
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Location {
    /// Latitude of the location in degrees; as defined by the sender
    pub latitude: f64,
    /// Longitude of the location, in degrees; as defined by the sender
    pub longitude: f64,
    /// The estimated horizontal accuracy of the location, in meters; as defined by the sender. 0 if unknown
    pub horizontal_accuracy: f64,
}

/// A live location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LiveLocation {
    /// The current location
    pub location: crate::types::Location,
    /// Time relative to the message send date, for which the location can be updated, in seconds; if 0x7FFFFFFF, then location can be updated forever
    pub live_period: i32,
    /// The direction in which the location moves, in degrees; 1-360; 0 if unknown
    pub heading: i32,
    /// The maximum distance to another chat member for proximity alerts, in meters (0-100000). 0 if the notification is disabled.
    /// Can't be enabled in direct messages chats, channels and Saved Messages. Available only to the message sender
    pub proximity_alert_radius: i32,
}

/// Describes a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Venue {
    /// Venue location; as defined by the sender
    pub location: crate::types::Location,
    /// Venue name; as defined by the sender
    pub title: String,
    /// Venue address; as defined by the sender
    pub address: String,
    /// Provider of the venue database; as defined by the sender. Currently, only "foursquare" and "gplaces" (Google Places) need to be supported
    pub provider: String,
    /// Identifier of the venue in the provider database; as defined by the sender
    pub id: String,
    /// Type of the venue in the provider database; as defined by the sender
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Describes a game. Use getInternalLink with internalLinkTypeGame to share the game
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Game {
    /// Unique game identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Game short name
    pub short_name: String,
    /// Game title
    pub title: String,
    /// Game text, usually containing scoreboards for a game
    pub text: crate::types::FormattedText,
    /// Game description
    pub description: String,
    /// Game photo
    pub photo: crate::types::Photo,
    /// Game animation; may be null
    pub animation: Option<crate::types::Animation>,
}

/// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WebApp {
    /// Web App short name
    pub short_name: String,
    /// Web App title
    pub title: String,
    /// Web App description
    pub description: String,
    /// Web App photo
    pub photo: crate::types::Photo,
    /// Web App animation; may be null
    pub animation: Option<crate::types::Animation>,
}

/// Describes a chat background
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Background {
    /// Unique background identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// True, if this is one of default backgrounds
    pub is_default: bool,
    /// True, if the background is dark and is recommended to be used with dark theme
    pub is_dark: bool,
    /// Unique background name
    pub name: String,
    /// Document with the background; may be null. Null only for filled and chat theme backgrounds
    pub document: Option<crate::types::Document>,
    /// Type of the background
    #[serde(rename = "type")]
    pub r#type: crate::enums::BackgroundType,
}

/// Contains a list of backgrounds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Backgrounds {
    /// A list of backgrounds
    pub backgrounds: Vec<crate::types::Background>,
}

/// Contains information about verification status of a chat or a user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VerificationStatus {
    /// True, if the chat or the user is verified by Telegram
    pub is_verified: bool,
    /// True, if the chat or the user is marked as scam by Telegram
    pub is_scam: bool,
    /// True, if the chat or the user is marked as fake by Telegram
    pub is_fake: bool,
    /// Identifier of the custom emoji to be shown as verification sign provided by a bot for the user; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub bot_verification_icon_custom_emoji_id: i64,
}

/// Represents a birthdate of a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Birthdate {
    /// Day of the month; 1-31
    pub day: i32,
    /// Month of the year; 1-12
    pub month: i32,
    /// Birth year; 0 if unknown
    pub year: i32,
}

/// Contains parameters of the application theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ThemeParameters {
    /// A color of the background in the RGB format
    pub background_color: i32,
    /// A secondary color for the background in the RGB format
    pub secondary_background_color: i32,
    /// A color of the header background in the RGB format
    pub header_background_color: i32,
    /// A color of the bottom bar background in the RGB format
    pub bottom_bar_background_color: i32,
    /// A color of the section background in the RGB format
    pub section_background_color: i32,
    /// A color of the section separator in the RGB format
    pub section_separator_color: i32,
    /// A color of text in the RGB format
    pub text_color: i32,
    /// An accent color of the text in the RGB format
    pub accent_text_color: i32,
    /// A color of text on the section headers in the RGB format
    pub section_header_text_color: i32,
    /// A color of the subtitle text in the RGB format
    pub subtitle_text_color: i32,
    /// A color of the text for destructive actions in the RGB format
    pub destructive_text_color: i32,
    /// A color of hints in the RGB format
    pub hint_color: i32,
    /// A color of links in the RGB format
    pub link_color: i32,
    /// A color of the buttons in the RGB format
    pub button_color: i32,
    /// A color of text on the buttons in the RGB format
    pub button_text_color: i32,
}

/// The Web App is opened in the compact mode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppOpenModeCompact {
}

/// The Web App is opened in the full-size mode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppOpenModeFullSize {
}

/// The Web App is opened in the full-screen mode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppOpenModeFullScreen {
}

/// Contains information about a Web App found by its short name
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FoundWebApp {
    /// The Web App
    pub web_app: crate::types::WebApp,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
    /// True, if there is no need to show an ordinary open URL confirmation before opening the Web App. The field must be ignored and confirmation must be shown anyway if the Web App link was hidden
    pub skip_confirmation: bool,
}

/// Contains information about a Web App URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppUrl {
    /// The Web App URL to open in a web view
    pub url: String,
    /// True, if events from the Web App must be accepted only from the same origin as the URL
    pub require_same_origin: bool,
}

/// Contains information about a Web App
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppInfo {
    /// Unique identifier for the Web App launch
    #[serde_as(as = "DisplayFromStr")]
    pub launch_id: i64,
    /// The Web App URL to open in a web view
    pub url: crate::types::WebAppUrl,
}

/// Contains information about the main Web App of a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MainWebApp {
    /// URL of the Web App to open
    pub url: crate::types::WebAppUrl,
    /// The mode in which the Web App must be opened
    pub mode: crate::enums::WebAppOpenMode,
}

/// Options to be used when a Web App is opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppOpenParameters {
    /// Preferred Web App theme; pass null to use the default theme
    pub theme: Option<crate::types::ThemeParameters>,
    /// Short name of the current application; 0-64 English letters, digits, and underscores
    pub application_name: String,
    /// The mode in which the Web App is opened; pass null to open in webAppOpenModeFullSize
    pub mode: Option<crate::enums::WebAppOpenMode>,
}

/// Describes price of a resold gift in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResalePriceStar {
    /// The Telegram Star amount expected to be paid for the gift. Must be in the range
    /// getOption("gift_resale_star_count_min")-getOption("gift_resale_star_count_max") for gifts put for resale
    pub star_count: i64,
}

/// Describes price of a resold gift in TON Grams
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResalePriceGram {
    /// The amount of 1/100 of Gram expected to be paid for the gift. Must be in the range
    /// getOption("gift_resale_gram_cent_count_min")-getOption("gift_resale_gram_cent_count_max")
    pub gram_cent_count: i64,
}

/// The offer must be accepted or rejected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftPurchaseOfferStatePending {
}

/// The offer was accepted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftPurchaseOfferStateAccepted {
}

/// The offer was rejected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftPurchaseOfferStateRejected {
}

/// Describes price of a suggested post in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostPriceStar {
    /// The Telegram Star amount expected to be paid for the post; getOption("suggested_post_star_count_min")-getOption("suggested_post_star_count_max")
    pub star_count: i64,
}

/// Describes price of a suggested post in TON Grams
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostPriceGram {
    /// The amount of 1/100 of Gram expected to be paid for the post; getOption("suggested_post_gram_cent_count_min")-getOption("suggested_post_gram_cent_count_max")
    pub gram_cent_count: i64,
}

/// The post must be approved or declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostStatePending {
}

/// The post was approved
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostStateApproved {
}

/// The post was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostStateDeclined {
}

/// Contains information about a suggested post. If the post can be approved or declined, then changes to the post can be also suggested. Use sendMessage with reply to the message
/// and suggested post information to suggest message changes. Use addOffer to suggest price or time changes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostInfo {
    /// Price of the suggested post; may be null if the post is non-paid
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published; 0 if the specific date isn't set yet
    pub send_date: i32,
    /// State of the post
    pub state: crate::enums::SuggestedPostState,
    /// True, if the suggested post can be approved by the current user using approveSuggestedPost; updates aren't sent when value of this field changes
    pub can_be_approved: bool,
    /// True, if the suggested post can be declined by the current user using declineSuggestedPost; updates aren't sent when value of this field changes
    pub can_be_declined: bool,
}

/// Contains information about a post to suggest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputSuggestedPostInfo {
    /// Price of the suggested post; pass null to suggest a post without payment. If the current user isn't an administrator of the channel direct messages chat
    /// and doesn't have enough funds to pay for the post, then the error "BALANCE_TOO_LOW" will be returned immediately
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published; pass 0 if the date isn't restricted. If specified,
    /// then the date must be getOption("suggested_post_send_delay_min")-getOption("suggested_post_send_delay_max") seconds in the future
    pub send_date: i32,
}

/// The post was refunded, because it was deleted by channel administrators in less than getOption("suggested_post_lifetime_min") seconds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostRefundReasonPostDeleted {
}

/// Describes a possibly non-integer Telegram Star amount
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarAmount {
    /// The integer Telegram Star amount rounded to 0
    pub star_count: i64,
    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999
    pub nanostar_count: i32,
}

/// Contains information about a product that can be paid with invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductInfo {
    /// Product title
    pub title: String,
    /// Product description
    pub description: crate::types::FormattedText,
    /// Product photo; may be null
    pub photo: Option<crate::types::Photo>,
}

/// Describes gift types that are accepted by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AcceptedGiftTypes {
    /// True, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,
    /// True, if limited regular gifts are accepted
    pub limited_gifts: bool,
    /// True, if upgraded gifts and regular gifts that can be upgraded for free are accepted
    pub upgraded_gifts: bool,
    /// True, if gifts from channels are accepted subject to other restrictions
    pub gifts_from_channels: bool,
    /// True, if Telegram Premium subscription is accepted
    pub premium_subscription: bool,
}

/// Contains settings for gift receiving for a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftSettings {
    /// True, if a button for sending a gift to the user or by the user must always be shown in the input field
    pub show_gift_button: bool,
    /// Types of gifts accepted by the user; for Telegram Premium users only
    pub accepted_gift_types: crate::types::AcceptedGiftTypes,
}

/// Describes an auction on which a gift can be purchased
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftAuction {
    /// Identifier of the auction
    pub id: String,
    /// Number of gifts distributed in each round
    pub gifts_per_round: i32,
    /// Point in time (Unix timestamp) when the auction will start
    pub start_date: i32,
}

/// Describes background of a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftBackground {
    /// Center color in RGB format
    pub center_color: i32,
    /// Edge color in RGB format
    pub edge_color: i32,
    /// Text color in RGB format
    pub text_color: i32,
}

/// Describes the maximum number of times that a specific gift can be purchased
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftPurchaseLimits {
    /// The maximum number of times the gifts can be purchased
    pub total_count: i32,
    /// Number of remaining times the gift can be purchased
    pub remaining_count: i32,
}

/// Describes parameters of a unique gift available for resale
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleParameters {
    /// Resale price of the gift in Telegram Stars
    pub star_count: i64,
    /// Resale price of the gift in 1/100 of TON Gram
    pub gram_cent_count: i64,
    /// True, if the gift can be bought only using Grams
    pub gram_only: bool,
}

/// Describes collection of gifts
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftCollection {
    /// Unique identifier of the collection
    pub id: i32,
    /// Name of the collection
    pub name: String,
    /// Icon of the collection; may be null if none
    pub icon: Option<crate::types::Sticker>,
    /// Total number of gifts in the collection
    pub gift_count: i32,
}

/// Contains a list of gift collections
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftCollections {
    /// List of gift collections
    pub collections: Vec<crate::types::GiftCollection>,
}

/// The gift can be sent now by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendGiftResultOk {
}

/// The gift can't be sent now by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendGiftResultFail {
    /// Reason to be shown to the user
    pub reason: crate::types::FormattedText,
}

/// The gift was obtained by upgrading of a previously received gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginUpgrade {
    /// Identifier of the message with the regular gift that was upgraded; may be 0 or an identifier of a deleted message
    pub gift_message_id: i64,
}

/// The gift was transferred from another owner
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginTransfer {
}

/// The gift was bought from another user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginResale {
    /// Price paid for the gift
    pub price: crate::enums::GiftResalePrice,
}

/// The gift was assigned from blockchain and isn't owned by the current user. The gift can't be transferred, resold or withdrawn to blockchain
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginBlockchain {
}

/// The sender or receiver of the message has paid for upgrade of the gift, which has been completed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginPrepaidUpgrade {
}

/// The gift was bought through an offer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginOffer {
    /// Price paid for the gift
    pub price: crate::enums::GiftResalePrice,
}

/// The gift was crafted from other gifts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginCraft {
}

/// The rarity is represented as the numeric frequency of the model
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeRarityPerMille {
    /// The number of upgraded gifts that receive this attribute for each 1000 gifts upgraded; if 0, then it can be shown as "<0.1%"
    pub per_mille: i32,
}

/// The attribute is uncommon
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeRarityUncommon {
}

/// The attribute is rare
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeRarityRare {
}

/// The attribute is epic
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeRarityEpic {
}

/// The attribute is legendary
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeRarityLegendary {
}

/// Describes a model of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftModel {
    /// Name of the model
    pub name: String,
    /// The sticker representing the upgraded gift
    pub sticker: crate::types::Sticker,
    /// The rarity of the model
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
    /// True, if the model can be obtained only through gift crafting
    pub is_crafted: bool,
}

/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftSymbol {
    /// Name of the symbol
    pub name: String,
    /// The sticker representing the symbol
    pub sticker: crate::types::Sticker,
    /// The rarity of the symbol
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
}

/// Describes colors of a backdrop of an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdropColors {
    /// A color in the center of the backdrop in the RGB format
    pub center_color: i32,
    /// A color on the edges of the backdrop in the RGB format
    pub edge_color: i32,
    /// A color to be applied for the symbol in the RGB format
    pub symbol_color: i32,
    /// A color for the text on the backdrop in the RGB format
    pub text_color: i32,
}

/// Describes a backdrop of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdrop {
    /// Unique identifier of the backdrop
    pub id: i32,
    /// Name of the backdrop
    pub name: String,
    /// Colors of the backdrop
    pub colors: crate::types::UpgradedGiftBackdropColors,
    /// The rarity of the backdrop
    pub rarity: crate::enums::UpgradedGiftAttributeRarity,
}

/// Describes the original details about the gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginalDetails {
    /// Identifier of the user or the chat that sent the gift; may be null if the gift was private
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Identifier of the user or the chat that received the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Point in time (Unix timestamp) when the gift was sent
    pub date: i32,
}

/// Contains information about color scheme for user's name, background of empty chat photo, replies to messages and link previews
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftColors {
    /// Unique identifier of the upgraded gift colors
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Custom emoji identifier of the model of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub model_custom_emoji_id: i64,
    /// Custom emoji identifier of the symbol of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub symbol_custom_emoji_id: i64,
    /// Accent color to use in light themes in RGB format
    pub light_theme_accent_color: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
    pub light_theme_colors: Vec<i32>,
    /// Accent color to use in dark themes in RGB format
    pub dark_theme_accent_color: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
    pub dark_theme_colors: Vec<i32>,
}

/// Describes a gift that can be sent to another user or channel chat
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Gift {
    /// Unique identifier of the gift
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the chat that published the gift; 0 if none
    pub publisher_chat_id: i64,
    /// The sticker representing the gift
    pub sticker: crate::types::Sticker,
    /// Number of Telegram Stars that must be paid for the gift
    pub star_count: i64,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift by default. If the gift was paid with just bought Telegram Stars, then full value can be claimed
    pub default_sell_star_count: i64,
    /// Number of Telegram Stars that must be paid to upgrade the gift; 0 if upgrade isn't possible
    pub upgrade_star_count: i64,
    /// Number of unique gift variants that are available for the upgraded gift; 0 if unknown
    pub upgrade_variant_count: i32,
    /// True, if the gift can be used to customize the user's name, and backgrounds of profile photo, reply header, and link preview
    pub has_colors: bool,
    /// True, if the gift is a birthday gift
    pub is_for_birthday: bool,
    /// True, if the gift can be bought only by Telegram Premium subscribers
    pub is_premium: bool,
    /// Information about the auction on which the gift can be purchased; may be null if the gift can be purchased directly
    pub auction_info: Option<crate::types::GiftAuction>,
    /// Point in time (Unix timestamp) when the gift can be sent next time by the current user; may be 0 or a date in the past.
    /// If the date is in the future, then call canSendGift to get the reason, why the gift can't be sent now
    pub next_send_date: i32,
    /// Number of times the gift can be purchased by the current user; may be null if not limited
    pub user_limits: Option<crate::types::GiftPurchaseLimits>,
    /// Number of times the gift can be purchased by all users; may be null if not limited
    pub overall_limits: Option<crate::types::GiftPurchaseLimits>,
    /// Background of the gift
    pub background: crate::types::GiftBackground,
    /// Point in time (Unix timestamp) when the gift was sent for the first time; for sold out gifts only
    pub first_send_date: i32,
    /// Point in time (Unix timestamp) when the gift was sent for the last time; for sold out gifts only
    pub last_send_date: i32,
}

/// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGift {
    /// Unique identifier of the gift
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Unique identifier of the regular gift from which the gift was upgraded; may be 0 for short period of time for old gifts from database
    #[serde_as(as = "DisplayFromStr")]
    pub regular_gift_id: i64,
    /// Identifier of the chat that published the gift; 0 if none
    pub publisher_chat_id: i64,
    /// The title of the upgraded gift
    pub title: String,
    /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift or sendResoldGift
    pub name: String,
    /// Unique number of the upgraded gift among gifts upgraded from the same gift
    pub number: i32,
    /// Total number of gifts that were upgraded from the same gift
    pub total_upgraded_count: i32,
    /// The maximum number of gifts that can be upgraded from the same gift
    pub max_upgraded_count: i32,
    /// True, if the gift was used to craft another gift
    pub is_burned: bool,
    /// True, if the gift was crafted from other gifts
    pub is_crafted: bool,
    /// True, if the original gift could have been bought only by Telegram Premium subscribers
    pub is_premium: bool,
    /// True, if the gift can be used to set a theme in a chat
    pub is_theme_available: bool,
    /// Identifier of the chat for which the gift is used to set a theme; 0 if none or the gift isn't owned by the current user
    pub used_theme_chat_id: i64,
    /// Identifier of the user or the chat to which the upgraded gift was assigned from blockchain; may be null if none or unknown
    pub host_id: Option<crate::enums::MessageSender>,
    /// Identifier of the user or the chat that owns the upgraded gift; may be null if none or unknown
    pub owner_id: Option<crate::enums::MessageSender>,
    /// Address of the gift NFT owner in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
    pub owner_address: String,
    /// Name of the owner for the case when owner identifier and address aren't known
    pub owner_name: String,
    /// Address of the gift NFT in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
    pub gift_address: String,
    /// Model of the upgraded gift
    pub model: crate::types::UpgradedGiftModel,
    /// Symbol of the upgraded gift
    pub symbol: crate::types::UpgradedGiftSymbol,
    /// Backdrop of the upgraded gift
    pub backdrop: crate::types::UpgradedGiftBackdrop,
    /// Information about the originally sent gift; may be null if unknown
    pub original_details: Option<crate::types::UpgradedGiftOriginalDetails>,
    /// Colors that can be set for user's name, background of empty chat photo, replies to messages and link previews; may be null if none or unknown
    pub colors: Option<crate::types::UpgradedGiftColors>,
    /// Resale parameters of the gift; may be null if resale isn't possible
    pub resale_parameters: Option<crate::types::GiftResaleParameters>,
    /// True, if an offer to purchase the gift can be sent using sendGiftPurchaseOffer
    pub can_send_purchase_offer: bool,
    /// Probability that the gift adds to the chance of successful crafting of a new gift; 0 if the gift can't be used for crafting
    pub craft_probability_per_mille: i32,
    /// ISO 4217 currency code of the currency in which value of the gift is represented; may be empty if unavailable
    pub value_currency: String,
    /// Estimated value of the gift; in the smallest units of the currency; 0 if unavailable
    pub value_amount: i64,
    /// Estimated value of the gift in USD; in USD cents; 0 if unavailable
    pub value_usd_amount: i64,
}

/// Contains information about value of an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftValueInfo {
    /// ISO 4217 currency code of the currency in which the prices are represented
    pub currency: String,
    /// Estimated value of the gift; in the smallest units of the currency
    pub value: i64,
    /// True, if the value is calculated as average value of similar sold gifts. Otherwise, it is based on the sale price of the gift
    pub is_value_average: bool,
    /// Point in time (Unix timestamp) when the corresponding regular gift was originally purchased
    pub initial_sale_date: i32,
    /// The Telegram Star amount that was paid for the gift
    pub initial_sale_star_count: i64,
    /// Initial price of the gift; in the smallest units of the currency
    pub initial_sale_price: i64,
    /// Point in time (Unix timestamp) when the upgraded gift was purchased last time; 0 if never
    pub last_sale_date: i32,
    /// Last purchase price of the gift; in the smallest units of the currency; 0 if the gift has never been resold
    pub last_sale_price: i64,
    /// True, if the last sale was completed on Fragment
    pub is_last_sale_on_fragment: bool,
    /// The current minimum price of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there are no such gifts
    pub minimum_price: i64,
    /// The average sale price in the last month of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there were no such sales
    pub average_sale_price: i64,
    /// Number of gifts upgraded from the same gift being resold on Telegram
    pub telegram_listed_gift_count: i32,
    /// Number of gifts upgraded from the same gift being resold on Fragment
    pub fragment_listed_gift_count: i32,
    /// The HTTPS link to the Fragment for the gift; may be empty if there are no such gifts being sold on Fragment
    pub fragment_url: String,
}

/// Contains result of gift upgrading
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradeGiftResult {
    /// The upgraded gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user
    pub received_gift_id: String,
    /// True, if the gift is displayed on the user's or the channel's profile page
    pub is_saved: bool,
    /// True, if the gift can be transferred to another owner
    pub can_be_transferred: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the gift can be transferred to the TON blockchain as an NFT; can be in the past
    pub export_date: i32,
}

/// Crafting was successful
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultSuccess {
    /// The created gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user
    pub received_gift_id: String,
}

/// Crafting isn't possible because one of the gifts can't be used for crafting yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultTooEarly {
    /// Time left before the gift can be used for crafting
    pub retry_after: i32,
}

/// Crafting isn't possible because one of the gifts isn't suitable for crafting
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultInvalidGift {
}

/// Crafting has failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultFail {
}

/// Describes a gift that is available for purchase
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AvailableGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Number of gifts that are available for resale
    pub resale_count: i32,
    /// The minimum price for the gifts available for resale in Telegram Star equivalent; 0 if there are no such gifts
    pub min_resale_star_count: i64,
    /// The title of the upgraded gift; empty if the gift isn't available for resale
    pub title: String,
}

/// Contains a list of gifts that can be sent to another user or channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AvailableGifts {
    /// The list of gifts
    pub gifts: Vec<crate::types::AvailableGift>,
}

/// Describes a price required to pay to upgrade a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradePrice {
    /// Point in time (Unix timestamp) when the price will be in effect
    pub date: i32,
    /// The Telegram Star amount required to pay to upgrade the gift
    pub star_count: i64,
}

/// Identifier of a gift model
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdModel {
    /// Identifier of the sticker representing the model
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}

/// Identifier of a gift symbol
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdSymbol {
    /// Identifier of the sticker representing the symbol
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}

/// Identifier of a gift backdrop
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdBackdrop {
    /// Identifier of the backdrop
    pub backdrop_id: i32,
}

/// Describes a model of an upgraded gift with the number of gifts found
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftModelCount {
    /// The model
    pub model: crate::types::UpgradedGiftModel,
    /// Total number of gifts with the model
    pub total_count: i32,
}

/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftSymbolCount {
    /// The symbol
    pub symbol: crate::types::UpgradedGiftSymbol,
    /// Total number of gifts with the symbol
    pub total_count: i32,
}

/// Describes a backdrop of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdropCount {
    /// The backdrop
    pub backdrop: crate::types::UpgradedGiftBackdrop,
    /// Total number of gifts with the symbol
    pub total_count: i32,
}

/// The gifts will be sorted by their price from the lowest to the highest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftForResaleOrderPrice {
}

/// The gifts will be sorted by the last date when their price was changed from the newest to the oldest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftForResaleOrderPriceChangeDate {
}

/// The gifts will be sorted by their number from the smallest to the largest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftForResaleOrderNumber {
}

/// Describes a gift available for resale
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftForResale {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Unique identifier of the received gift for the current user; only for the gifts owned by the current user
    pub received_gift_id: String,
}

/// Describes gifts available for resale
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftsForResale {
    /// Total number of gifts found
    pub total_count: i32,
    /// The gifts
    pub gifts: Vec<crate::types::GiftForResale>,
    /// Available models; for searchGiftsForResale requests without offset and attributes only
    pub models: Vec<crate::types::UpgradedGiftModelCount>,
    /// Available symbols; for searchGiftsForResale requests without offset and attributes only
    pub symbols: Vec<crate::types::UpgradedGiftSymbolCount>,
    /// Available backdrops; for searchGiftsForResale requests without offset and attributes only
    pub backdrops: Vec<crate::types::UpgradedGiftBackdropCount>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Operation was successfully completed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleResultOk {
    /// Unique identifier of the received gift; only for the gifts sent to the current user
    pub received_gift_id: String,
}

/// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleResultPriceIncreased {
    /// New price for the gift
    pub price: crate::enums::GiftResalePrice,
}

/// Regular gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SentGiftRegular {
    /// The gift
    pub gift: crate::types::Gift,
}

/// Upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SentGiftUpgraded {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// Represents a gift received by a user or a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReceivedGift {
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// Identifier of a user or a chat that sent the gift; may be null if unknown
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone are able to see them
    pub is_private: bool,
    /// True, if the gift is displayed on the chat's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the gift is pinned to the top of the chat's profile page
    pub is_pinned: bool,
    /// True, if the gift is a regular gift that can be upgraded to a unique gift; only for the receiver of the gift
    pub can_be_upgraded: bool,
    /// True, if the gift is an upgraded gift that can be transferred to another owner; only for the receiver of the gift
    pub can_be_transferred: bool,
    /// True, if the gift was refunded and isn't available anymore
    pub was_refunded: bool,
    /// Point in time (Unix timestamp) when the gift was sent
    pub date: i32,
    /// The gift
    pub gift: crate::enums::SentGift,
    /// Identifiers of collections to which the gift is added; only for the receiver of the gift
    pub collection_ids: Vec<i32>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift; 0 if the gift can't be sold by the current user
    pub sell_star_count: i64,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: i64,
    /// True, if the upgrade was bought after the gift was sent. In this case, prepaid upgrade cost must not be added to the gift cost
    pub is_upgrade_separate: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift; only for the receiver of the gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available; only for the receiver of the gift
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible; only for the receiver of the gift
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the upgraded gift can be transferred to the TON blockchain as an NFT; can be in the past; 0 if NFT export isn't possible; only for the receiver of the gift
    pub export_date: i32,
    /// If non-empty, then the user can pay for an upgrade of the gift using buyGiftUpgrade
    pub prepaid_upgrade_hash: String,
    /// Point in time (Unix timestamp) when the gift can be used to craft another gift; can be in the past; only for the receiver of the gift
    pub craft_date: i32,
}

/// Represents a list of gifts received by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReceivedGifts {
    /// The total number of received gifts
    pub total_count: i32,
    /// The list of gifts
    pub gifts: Vec<crate::types::ReceivedGift>,
    /// True, if notifications about new gifts of the owner are enabled
    pub are_notifications_enabled: bool,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Describes chance of the crafted gift to have the backdrop or symbol of one of the original gifts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttributeCraftPersistenceProbability {
    /// The 4 numbers that describe probability of the craft result to have the same attribute as one of the original gifts
    /// if 1, 2, 3, or 4 gifts with the attribute are used in the craft. Each number represents the number of crafted gifts with the original attribute per 1000 successful craftings
    pub persistence_chance_per_mille: Vec<i32>,
}

/// Represents a list of gifts received by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftsForCrafting {
    /// The total number of received gifts
    pub total_count: i32,
    /// The list of gifts
    pub gifts: Vec<crate::types::ReceivedGift>,
    /// The 4 objects that describe probabilities of the crafted gift to have the backdrop or symbol of one of the original gifts
    /// for the cases when 1, 2, 3 or 4 gifts are used in the craft correspondingly
    pub attribute_persistence_probabilities: Vec<crate::types::AttributeCraftPersistenceProbability>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains examples of possible upgraded gifts for the given regular gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradePreview {
    /// Examples of possible models that can be chosen for the gift after upgrade
    pub models: Vec<crate::types::UpgradedGiftModel>,
    /// Examples of possible symbols that can be chosen for the gift after upgrade
    pub symbols: Vec<crate::types::UpgradedGiftSymbol>,
    /// Examples of possible backdrops that can be chosen for the gift after upgrade
    pub backdrops: Vec<crate::types::UpgradedGiftBackdrop>,
    /// Examples of price for gift upgrade from the maximum price to the minimum price
    pub prices: Vec<crate::types::GiftUpgradePrice>,
    /// Next changes for the price for gift upgrade with more granularity than in prices
    pub next_prices: Vec<crate::types::GiftUpgradePrice>,
}

/// Contains all possible variants of upgraded gifts for the given regular gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradeVariants {
    /// Models that can be chosen for the gift after upgrade
    pub models: Vec<crate::types::UpgradedGiftModel>,
    /// Symbols that can be chosen for the gift after upgrade
    pub symbols: Vec<crate::types::UpgradedGiftSymbol>,
    /// Backdrops that can be chosen for the gift after upgrade
    pub backdrops: Vec<crate::types::UpgradedGiftBackdrop>,
}

/// Describes a bid in an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionBid {
    /// The number of Telegram Stars that were put in the bid
    pub star_count: i64,
    /// Point in time (Unix timestamp) when the bid was made
    pub bid_date: i32,
    /// Position of the bid in the list of all bids
    pub position: i32,
}

/// Describes a round of an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionRound {
    /// 1-based number of the round
    pub number: i32,
    /// Duration of the round, in seconds
    pub duration: i32,
    /// The number of seconds for which the round will be extended if there are changes in the top winners
    pub extend_time: i32,
    /// The number of top winners who trigger round extension if changed
    pub top_winner_count: i32,
}

/// Contains information about an ongoing or scheduled auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuctionStateActive {
    /// Point in time (Unix timestamp) when the auction started or will start
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub end_date: i32,
    /// The minimum possible bid in the auction in Telegram Stars
    pub min_bid: i64,
    /// A sparse list of bids that were made in the auction
    pub bid_levels: Vec<crate::types::AuctionBid>,
    /// User identifiers of at most 3 users with the biggest bids
    pub top_bidder_user_ids: Vec<i64>,
    /// Rounds of the auction in which their duration or extension rules are changed
    pub rounds: Vec<crate::types::AuctionRound>,
    /// Point in time (Unix timestamp) when the current round will end
    pub current_round_end_date: i32,
    /// 1-based number of the current round
    pub current_round_number: i32,
    /// The total number of rounds
    pub total_round_count: i32,
    /// The number of items that were purchased on the auction by all users
    pub distributed_item_count: i32,
    /// The number of items that have to be distributed on the auction
    pub left_item_count: i32,
    /// The number of items that were purchased by the current user on the auction
    pub acquired_item_count: i32,
    /// Bid of the current user in the auction; may be null if none
    pub user_bid: Option<crate::types::UserAuctionBid>,
}

/// Contains information about a finished auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuctionStateFinished {
    /// Point in time (Unix timestamp) when the auction started
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub end_date: i32,
    /// Average price of bought items in Telegram Stars
    pub average_price: i64,
    /// The number of items that were purchased by the current user on the auction
    pub acquired_item_count: i32,
    /// Number of items from the auction being resold on Telegram
    pub telegram_listed_item_count: i32,
    /// Number of items from the auction being resold on Fragment
    pub fragment_listed_item_count: i32,
    /// The HTTPS link to the Fragment for the resold items; may be empty if there are no such items being sold on Fragment
    pub fragment_url: String,
}

/// Represent auction state of a gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionState {
    /// The gift
    pub gift: crate::types::Gift,
    /// Auction state of the gift
    pub state: crate::enums::AuctionState,
}

/// Represents a gift that was acquired by the current user on an auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionAcquiredGift {
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the gift was acquired
    pub date: i32,
    /// The number of Telegram Stars that were paid for the gift
    pub star_count: i64,
    /// Identifier of the auction round in which the gift was acquired
    pub auction_round_number: i32,
    /// Position of the user in the round among all auction participants
    pub auction_round_position: i32,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: bool,
}

/// Represents a list of gifts that were acquired by the current user on an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionAcquiredGifts {
    /// The list of acquired gifts
    pub gifts: Vec<crate::types::GiftAuctionAcquiredGift>,
}

/// The transaction is incoming and increases the amount of owned currency
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TransactionDirectionIncoming {
}

/// The transaction is outgoing and decreases the amount of owned currency
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TransactionDirectionOutgoing {
}

/// The transaction is a deposit of Telegram Stars from App Store; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeAppStoreDeposit {
}

/// The transaction is a deposit of Telegram Stars from Google Play; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGooglePlayDeposit {
}

/// The transaction is a deposit of Telegram Stars from Fragment; relevant for regular users and bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeFragmentDeposit {
}

/// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeFragmentWithdrawal {
    /// State of the withdrawal; may be null for refunds from Fragment
    pub withdrawal_state: Option<crate::enums::RevenueWithdrawalState>,
}

/// The transaction is a withdrawal of earned Telegram Stars to Telegram Ad platform; relevant for bots and channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeTelegramAdsWithdrawal {
}

/// The transaction is a payment for Telegram API usage; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeTelegramApiUsage {
    /// The number of billed requests
    pub request_count: i32,
}

/// The transaction is a bid on a gift auction; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftAuctionBid {
    /// Identifier of the user who will receive the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}

/// The transaction is a purchase of a regular gift; relevant for regular users and bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftPurchase {
    /// Identifier of the user or the channel that received the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}

/// The transaction is an offer of gift purchase; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftPurchaseOffer {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a transfer of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftTransfer {
    /// Identifier of the user or the channel that received the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftOriginalDetailsDrop {
    /// Identifier of the user or the channel that owns the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a sale of a received gift; relevant for regular users and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftSale {
    /// Identifier of the user who sent the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::Gift,
}

/// The transaction is an upgrade of a gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftUpgrade {
    /// Identifier of the user who initially sent the gift
    pub user_id: i64,
    /// The upgraded gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftUpgradePurchase {
    /// Owner of the upgraded gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}

/// The transaction is a purchase of an upgraded gift for some user or channel; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUpgradedGiftPurchase {
    /// Identifier of the user who sold the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a sale of an upgraded gift; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUpgradedGiftSale {
    /// Identifier of the user who bought the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars received by the seller of the gift
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
    /// True, if the gift was sold through a purchase offer
    pub via_offer: bool,
}

/// The transaction is a payment for search of posts in public Telegram channels; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePublicPostSearch {
}

/// The transaction is a transaction of an unsupported type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUnsupported {
}

/// Represents a transaction changing the amount of owned Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction
    pub id: String,
    /// The amount of added owned Telegram Stars; negative for outgoing transactions
    pub star_amount: crate::types::StarAmount,
    /// True, if the transaction is a refund of a previous transaction
    pub is_refund: bool,
    /// Point in time (Unix timestamp) when the transaction was completed
    pub date: i32,
    /// Type of the transaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::StarTransactionType,
}

/// Represents a list of Telegram Star transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactions {
    /// The amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// List of transactions with Telegram Stars
    pub transactions: Vec<crate::types::StarTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// The transaction is a deposit of Grams from Fragment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeFragmentDeposit {
    /// True, if the transaction is a gift from another user
    pub is_gift: bool,
    /// The sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// The transaction is a withdrawal of earned Grams to Fragment
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeFragmentWithdrawal {
    /// State of the withdrawal; may be null for refunds from Fragment
    pub withdrawal_state: Option<crate::enums::RevenueWithdrawalState>,
}

/// The transaction is an offer of gift purchase
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeGiftPurchaseOffer {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a purchase of an upgraded gift for some user or channel
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeUpgradedGiftPurchase {
    /// Identifier of the user who sold the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The transaction is a sale of an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeUpgradedGiftSale {
    /// Identifier of the user who bought the gift
    pub user_id: i64,
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The number of Grams received by the Telegram for each 1000 Grams received by the seller of the gift
    pub commission_per_mille: i32,
    /// The Gram amount that was received by the Telegram; in the smallest units of the currency
    pub commission_gram_amount: i64,
    /// True, if the gift was sold through a purchase offer
    pub via_offer: bool,
}

/// The transaction is a transaction of an unsupported type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeUnsupported {
}

/// Represents a transaction changing the amount of owned TON Grams
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransaction {
    /// Unique identifier of the transaction
    pub id: String,
    /// The amount of added owned Grams, in the smallest units of the cryptocurrency; negative for outgoing transactions
    pub gram_amount: i64,
    /// True, if the transaction is a refund of a previous transaction
    pub is_refund: bool,
    /// Point in time (Unix timestamp) when the transaction was completed
    pub date: i32,
    /// Type of the transaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::TonTransactionType,
}

/// Represents a list of TON Gram transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactions {
    /// The total amount of owned Grams, in the smallest units of the cryptocurrency
    pub gram_amount: i64,
    /// List of Gram transactions
    pub transactions: Vec<crate::types::TonTransaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Options to be used for generation of a link preview
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewOptions {
    /// True, if link preview must be disabled
    pub is_disabled: bool,
    /// URL to use for link preview. If empty, then the first URL found in the message text will be used
    pub url: String,
    /// True, if shown media preview must be small; ignored in secret chats or if the URL isn't explicitly specified
    pub force_small_media: bool,
    /// True, if shown media preview must be large; ignored in secret chats or if the URL isn't explicitly specified
    pub force_large_media: bool,
    /// True, if link preview must be shown above message text; otherwise, the link preview will be shown below the message text; ignored in secret chats
    pub show_above_text: bool,
}

/// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccentColor {
    /// Accent color identifier
    pub id: i32,
    /// Identifier of a built-in color to use in places, where only one color is needed; 0-6
    pub built_in_accent_color_id: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
    pub light_theme_colors: Vec<i32>,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
    pub dark_theme_colors: Vec<i32>,
    /// The minimum chat boost level required to use the color in a channel chat
    pub min_channel_chat_boost_level: i32,
}

/// Contains identifier of a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityId {
    /// Community identifier
    pub id: i64,
}

/// Describes actions that a user is allowed to take in a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityPermissions {
    /// True, if the user can change the chats added to the community
    pub can_edit_chat_list: bool,
}

/// Describes rights of the administrator in a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityAdministratorRights {
    /// True, if the user is an administrator. Implied by any other privilege
    pub can_manage_community: bool,
    /// True, if the administrator can change the community name, photo, and other settings
    pub can_change_info: bool,
    /// True, if the user can change the chats added to the community
    pub can_edit_chat_list: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them
    pub can_promote_members: bool,
    /// True, if the administrator can ban, or unban community members
    pub can_ban_members: bool,
}

/// The user is the owner of the community and has all the administrator privileges
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityMemberStatusCreator {
}

/// The user is a member of the community and has some additional privileges
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityMemberStatusAdministrator {
    /// True, if the current user can edit the administrator privileges for the called user
    pub can_be_edited: bool,
    /// Rights of the administrator
    pub rights: crate::types::CommunityAdministratorRights,
}

/// The user is a member of the community, without any additional privileges or restrictions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityMemberStatusMember {
}

/// The user or the chat is not a community member
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityMemberStatusLeft {
}

/// The user or the chat was banned in the community; implies ban in all chats in the community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityMemberStatusBanned {
}

/// Represents a community consisting of supergroup chats, channel chats and chats with bots
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Community {
    /// Community identifier
    pub id: i64,
    /// If false, the community is inaccessible, and the only information known about the community is inside this class. Identifier of the community can't be passed to any method
    pub have_access: bool,
    /// Community name
    pub name: String,
    /// Community photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Point in time (Unix timestamp) when the community was joined, or the point in time when the community was created, in case the user is not a member of any chat in the community
    pub date: i32,
    /// Status of the current user in the community
    pub status: crate::enums::CommunityMemberStatus,
    /// Actions that non-administrator community members are allowed to take in the community
    pub permissions: crate::types::CommunityPermissions,
}

/// Contains full information about a community
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CommunityFullInfo {
    /// Photo of the community
    pub photo: crate::types::ChatPhoto,
    /// Chats belonging to the community
    pub chats: Vec<crate::types::CommunityChat>,
    /// Number of privileged users in the community; 0 if the current user isn't an administrator of the community
    pub administrator_count: i32,
    /// Number of users banned from the community; 0 if the current user isn't an administrator of the community
    pub banned_count: i32,
    /// Number of pending requests for addition of chats to the community; 0 if the current user isn't an administrator of the community
    pub add_chat_request_count: i32,
}

/// Contains information about restrictions that must be applied to a chat or a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RestrictionInfo {
    /// A human-readable description of the reason why access to the content must be restricted. If empty, then the content can be accessed,
    /// but may be covered by hidden with 18+ spoiler anyway
    pub restriction_reason: String,
    /// True, if media content of the messages must be hidden with 18+ spoiler.
    /// Use value of the option "can_ignore_sensitive_content_restrictions" to check whether the current user can ignore the restriction.
    /// If age verification parameters were received in updateAgeVerificationParameters, then the user must complete age verification to ignore the restriction.
    /// Set the option "ignore_sensitive_content_restrictions" to true if the user passes age verification
    pub has_sensitive_content: bool,
}

/// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BasicGroup {
    /// Group identifier
    pub id: i64,
    /// Number of members in the group
    pub member_count: i32,
    /// Status of the current user in the group
    pub status: crate::enums::ChatMemberStatus,
    /// True, if the group is active
    pub is_active: bool,
    /// Identifier of the supergroup to which this group was upgraded; 0 if none
    pub upgraded_to_supergroup_id: i64,
}

/// Contains full information about a basic group
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BasicGroupFullInfo {
    /// Chat photo; may be null if empty or unknown. If non-null, then it is the same photo as in chat.photo
    pub photo: Option<crate::types::ChatPhoto>,
    /// Group description. Updated only after the basic group is opened
    pub description: String,
    /// User identifier of the creator of the group; 0 if unknown
    pub creator_user_id: i64,
    /// Group members
    pub members: Vec<crate::types::ChatMember>,
    /// True, if non-administrators and non-bots can be hidden in responses to getSupergroupMembers and searchChatMembers for non-administrators after upgrading the basic group to a supergroup
    pub can_hide_members: bool,
    /// True, if aggressive anti-spam checks can be enabled or disabled in the supergroup after upgrading the basic group to a supergroup
    pub can_toggle_aggressive_anti_spam: bool,
    /// Primary invite link for this group; may be null. For chat administrators with can_invite_users right only. Updated only after the basic group is opened
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// List of commands of bots in the group
    pub bot_commands: Vec<crate::types::BotCommands>,
}

/// Contains information about public post search limits
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicPostSearchLimits {
    /// Number of queries that can be sent daily for free
    pub daily_free_query_count: i32,
    /// Number of remaining free queries today
    pub remaining_free_query_count: i32,
    /// Amount of time till the next free query can be sent; 0 if it can be sent now
    pub next_free_query_in: i32,
    /// Number of Telegram Stars that must be paid for each non-free query
    pub star_count: i64,
    /// True, if the search for the specified query isn't charged
    pub is_current_query_free: bool,
}

/// Contains information about the last message from which a new message was forwarded last time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForwardSource {
    /// Identifier of the chat to which the message that was forwarded belonged; may be 0 if unknown
    pub chat_id: i64,
    /// Identifier of the message; may be 0 if unknown
    pub message_id: i64,
    /// Identifier of the sender of the message; may be null if unknown or the new message was forwarded not to Saved Messages
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Name of the sender of the message if the sender is hidden by their privacy settings
    pub sender_name: String,
    /// Point in time (Unix timestamp) when the message is sent; 0 if unknown
    pub date: i32,
    /// True, if the message that was forwarded is outgoing; always false if sender is unknown
    pub is_outgoing: bool,
}

/// Contains information about a user who added paid reactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactor {
    /// Identifier of the user or chat that added the reactions; may be null for anonymous reactors that aren't the current user
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Number of Telegram Stars added
    pub star_count: i64,
    /// True, if the reactor is one of the most active reactors; may be false if the reactor is the current user
    pub is_top: bool,
    /// True, if the paid reaction was added by the current user
    pub is_me: bool,
    /// True, if the reactor is anonymous
    pub is_anonymous: bool,
}

/// Describes a fact-check added to the message by an independent checker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FactCheck {
    /// Text of the fact-check
    pub text: crate::types::FormattedText,
    /// A two-letter ISO 3166-1 alpha-2 country code of the country for which the fact-check is shown
    pub country_code: String,
}

/// Contains a list of messages found by a public post search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPublicPosts {
    /// List of found public posts
    pub messages: Vec<crate::types::Message>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
    /// Updated public post search limits after the query; repeated requests with the same query will be free; may be null if they didn't change
    pub search_limits: Option<crate::types::PublicPostSearchLimits>,
    /// True, if the query has failed because search limits are exceeded. In this case search_limits.daily_free_query_count will be equal to 0
    pub are_limits_exceeded: bool,
}

/// Information about the sponsor of an advertisement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AdvertisementSponsor {
    /// URL of the sponsor to be opened when the advertisement is clicked
    pub url: String,
    /// Photo of the sponsor; may be null if must not be shown
    pub photo: Option<crate::types::Photo>,
    /// Additional optional information about the sponsor to be shown along with the advertisement
    pub info: String,
}

/// Describes an option to report an entity to Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportOption {
    /// Unique identifier of the option
    pub id: String,
    /// Text of the option
    pub text: String,
}

/// The message was reported successfully
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportSponsoredResultOk {
}

/// The sponsored message is too old or not found
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportSponsoredResultFailed {
}

/// The user must choose an option to report the message and repeat request with the chosen option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportSponsoredResultOptionRequired {
    /// Title for the option choice
    pub title: String,
    /// List of available options
    pub options: Vec<crate::types::ReportOption>,
}

/// Sponsored messages were hidden for the user in all chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportSponsoredResultAdsHidden {
}

/// Contains information about a user who has failed to be added to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FailedToAddMember {
    /// User identifier
    pub user_id: i64,
    /// True, if subscription to Telegram Premium would have allowed to add the user to the chat
    pub premium_would_allow_invite: bool,
    /// True, if subscription to Telegram Premium is required to send the user chat invite link
    pub premium_required_to_send_messages: bool,
}

/// Represents a list of users that has failed to be added to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FailedToAddMembers {
    /// Information about users that weren't added to the chat
    pub failed_to_add_members: Vec<crate::types::FailedToAddMember>,
}

/// Contains basic information about another user who started a chat with the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccountInfo {
    /// Month when the user was registered in Telegram; 0-12; may be 0 if unknown
    pub registration_month: i32,
    /// Year when the user was registered in Telegram; 0-9999; may be 0 if unknown
    pub registration_year: i32,
    /// A two-letter ISO 3166-1 alpha-2 country code based on the phone number of the user; may be empty if unknown
    pub phone_number_country_code: String,
    /// Point in time (Unix timestamp) when the user changed name last time; 0 if unknown
    pub last_name_change_date: i32,
    /// Point in time (Unix timestamp) when the user changed photo last time; 0 if unknown
    pub last_photo_change_date: i32,
}

/// The button has default style
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ButtonStyleDefault {
}

/// The button has dark blue color
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ButtonStylePrimary {
}

/// The button has red color
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ButtonStyleDanger {
}

/// The button has green color
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ButtonStyleSuccess {
}

/// The button must be shown as a link. The style is allowed only for callback buttons in inlineButton
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ButtonStyleLink {
}

/// A button that sends the user's phone number when pressed; available only in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
}

/// A button that sends the user's location when pressed; available only in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestLocation {
}

/// A button that opens a Web App by calling getWebAppUrl
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeWebApp {
    /// An HTTP URL to pass to getWebAppUrl
    pub url: String,
}

/// Represents a single button in a bot keyboard
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButton {
    /// Text of the button
    pub text: String,
    /// Identifier of the custom emoji that must be shown on the button; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Style of the button
    pub style: crate::enums::ButtonStyle,
    /// Type of the button
    #[serde(rename = "type")]
    pub r#type: crate::enums::KeyboardButtonType,
}

/// A button that opens a specified URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeUrl {
    /// HTTP or tg: URL to open. If the link is of the type internalLinkTypeWebApp, then the button must be marked as a Web App button
    pub url: String,
}

/// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo; not supported in ephemeral messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
    /// An HTTP URL to pass to getLoginUrlInfo
    pub url: String,
    /// Unique button identifier
    pub id: i64,
    /// If non-empty, new text of the button in forwarded messages
    pub forward_text: String,
}

/// A button that opens a Web App by calling openWebApp
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeWebApp {
    /// An HTTP URL to pass to openWebApp
    pub url: String,
}

/// A button that forces an inline query to the bot to be inserted in the input field
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
    /// Inline query to be sent to the bot
    pub query: String,
    /// Target chat from which to send the inline query
    pub target_chat: crate::enums::TargetChat,
}

/// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeBuy {
}

/// A disabled button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeDisabled {
}

/// The button is a prepared keyboard button from a Mini App received via getPreparedKeyboardButton
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonSourceWebApp {
    /// Identifier of the bot that created the button
    pub bot_user_id: i64,
    /// Identifier of the prepared button
    pub prepared_button_id: String,
}

/// Represents a single button in an inline keyboard
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    /// Text of the button
    pub text: String,
    /// Identifier of the custom emoji that must be shown on the button; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Style of the button
    pub style: crate::enums::ButtonStyle,
    /// Type of the button
    #[serde(rename = "type")]
    pub r#type: crate::enums::InlineKeyboardButtonType,
}

/// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, updateChatReplyMarkup with reply_markup_message == null will be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupRemoveKeyboard {
    /// True, if the keyboard is removed only for the mentioned users or the target user of a reply
    pub is_personal: bool,
}

/// Instructs application to force a reply to this message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupForceReply {
    /// True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply
    pub is_personal: bool,
    /// If non-empty, the placeholder to be shown in the input field when the reply is active; 0-64 characters
    pub input_field_placeholder: String,
}

/// Contains a custom keyboard layout to quickly reply to bots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupShowKeyboard {
    /// A list of rows of bot keyboard buttons
    pub rows: Vec<Vec<crate::types::KeyboardButton>>,
    /// True, if the keyboard is expected to always be shown when the ordinary keyboard is hidden
    pub is_persistent: bool,
    /// True, if the application needs to resize the keyboard vertically
    pub resize_keyboard: bool,
    /// True, if the application needs to hide the keyboard after use
    pub one_time: bool,
    /// True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply
    pub is_personal: bool,
    /// True, if the keyboard must force reply to the message with the keyboard
    pub force_reply: bool,
    /// If non-empty, the placeholder to be shown in the input field when the keyboard is active; 0-64 characters
    pub input_field_placeholder: String,
}

/// Contains an inline keyboard layout
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupInlineKeyboard {
    /// A list of rows of inline keyboard buttons
    pub rows: Vec<Vec<crate::types::InlineKeyboardButton>>,
    /// True, if a reply to the message must be forced when the message is received
    pub force_reply: bool,
}

/// An HTTP URL needs to be open
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LoginUrlInfoOpen {
    /// The URL to open
    pub url: String,
    /// True, if there is no need to show an ordinary open URL confirmation
    pub skip_confirmation: bool,
}

/// An authorization confirmation dialog needs to be shown to the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LoginUrlInfoRequestConfirmation {
    /// An HTTP URL to be opened
    pub url: String,
    /// A domain of the URL
    pub domain: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
}

/// Information about the OAuth authorization
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OauthLinkInfo {
    /// Identifier of the user for which the link was generated; may be 0 if unknown. The corresponding user may be unknown.
    /// If the user is logged in the application, then they must be chosen for authorization by default
    pub user_id: i64,
    /// An HTTP URL where the user authorizes
    pub url: String,
    /// A domain of the URL
    pub domain: String,
    /// True, if the authorization originates from an application
    pub from_app: bool,
    /// Verified name of the application; if empty, then "Unverified App" must be shown instead
    pub verified_app_name: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
    /// True, if the user must be asked for the permission to share their phone number
    pub request_phone_number_access: bool,
    /// The version of a browser used for the authorization
    pub browser: String,
    /// Operating system the browser is running on
    pub platform: String,
    /// IP address from which the authorization is performed, in human-readable format
    pub ip_address: String,
    /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
    pub location: String,
    /// True, if code matching dialog must be shown first and checkOauthRequestMatchCode must be called before acceptOauthRequest. Otherwise, checkOauthRequestMatchCode must not be called
    pub match_code_first: bool,
    /// The list of codes to match; may be empty if irrelevant
    pub match_codes: Vec<String>,
}

/// Classic light theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BuiltInThemeClassic {
}

/// Regular light theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BuiltInThemeDay {
}

/// Regular dark theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BuiltInThemeNight {
}

/// Tinted dark theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BuiltInThemeTinted {
}

/// Arctic light theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BuiltInThemeArctic {
}

/// Describes theme settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThemeSettings {
    /// Base theme for this theme
    pub base_theme: crate::enums::BuiltInTheme,
    /// Theme accent color in ARGB format
    pub accent_color: i32,
    /// The background to be used in chats; may be null
    pub background: Option<crate::types::Background>,
    /// The fill to be used as a background for outgoing messages; may be null if the fill from the base theme must be used instead
    pub outgoing_message_fill: Option<crate::enums::BackgroundFill>,
    /// If true, the freeform gradient fill needs to be animated on every sent message
    pub animate_outgoing_message_fill: bool,
    /// Accent color of outgoing messages in ARGB format
    pub outgoing_message_accent_color: i32,
}

/// Represents a button inside a rich message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineButton {
    /// Text of the button; only richTexts, richTextPlain, and richTextCustomEmoji are allowed
    pub text: crate::enums::RichText,
    /// Style of the button
    pub style: crate::enums::ButtonStyle,
    /// Type of the button; must be one of inlineKeyboardButtonTypeUrl, inlineKeyboardButtonTypeLoginUrl, inlineKeyboardButtonTypeWebApp, inlineKeyboardButtonTypeCallback,
    /// inlineKeyboardButtonTypeSwitchInline, inlineKeyboardButtonTypeUser, inlineKeyboardButtonTypeCopyText. Additionally,
    /// inlineKeyboardButtonTypeCallbackWithPassword and inlineKeyboardButtonTypeDisabled may be received in incoming messages. Regular users may use only inlineKeyboardButtonTypeUrl,
    /// inlineKeyboardButtonTypeUser and inlineKeyboardButtonTypeCopyText
    #[serde(rename = "type")]
    pub r#type: crate::enums::InlineKeyboardButtonType,
}

/// Describes an item of a list page block
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockListItem {
    /// Item label
    pub label: String,
    /// Item blocks
    pub blocks: Vec<crate::enums::PageBlock>,
    /// True, if the item has a checkbox
    pub has_checkbox: bool,
    /// True, if the item is checked
    pub is_checked: bool,
    /// Value of the item; 0 for unordered lists
    pub value: i32,
    /// Type of the item numbering type; must be one of "a" for lowercase letters, "A" for uppercase letters, "i" for lowercase Roman numerals, "I" for uppercase Roman numerals,
    /// "1" for decimal numbers, or empty for unordered lists
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Describes an item of a list page block to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockListItem {
    /// Item blocks
    pub blocks: Vec<crate::enums::InputPageBlock>,
    /// True, if the item has a checkbox
    pub has_checkbox: bool,
    /// True, if the item is checked
    pub is_checked: bool,
    /// Value of the item; pass 0 for unordered lists
    pub value: i32,
    /// Type of the item numbering type; must be one of "a" for a lowercase letter, "A" for an uppercase letter, "i" for lowercase Roman numerals, "I" for uppercase Roman numerals,
    /// "1" for decimal numbers, or empty for unordered lists
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The content must be left-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockHorizontalAlignmentLeft {
}

/// The content must be center-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockHorizontalAlignmentCenter {
}

/// The content must be right-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockHorizontalAlignmentRight {
}

/// The content must be top-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVerticalAlignmentTop {
}

/// The content must be middle-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVerticalAlignmentMiddle {
}

/// Represents a cell of a table
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTableCell {
    /// Cell text; may be null. If the text is null, then the cell must be invisible
    pub text: Option<crate::enums::RichText>,
    /// True, if it is a header cell
    pub is_header: bool,
    /// The number of columns the cell spans
    pub colspan: i32,
    /// The number of rows the cell spans
    pub rowspan: i32,
    /// Horizontal cell content alignment
    pub align: crate::enums::PageBlockHorizontalAlignment,
    /// Vertical cell content alignment
    pub valign: crate::enums::PageBlockVerticalAlignment,
}

/// Contains information about a related article
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockRelatedArticle {
    /// Related article URL
    pub url: String,
    /// Article title; may be empty
    pub title: String,
    /// Article description; may be empty
    pub description: String,
    /// Article photo; may be null
    pub photo: Option<crate::types::Photo>,
    /// Article author; may be empty
    pub author: String,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown
    pub publish_date: i32,
}

/// The title of a page; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTitle {
    /// Title
    pub title: crate::enums::RichText,
}

/// The subtitle of a page; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSubtitle {
    /// Subtitle
    pub subtitle: crate::enums::RichText,
}

/// The author and publishing date of a page; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAuthorDate {
    /// Author
    pub author: crate::enums::RichText,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown
    pub publish_date: i32,
}

/// A header; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockHeader {
    /// Header
    pub header: crate::enums::RichText,
}

/// A subheader; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSubheader {
    /// Subheader
    pub subheader: crate::enums::RichText,
}

/// A section heading
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSectionHeading {
    /// Text of the section heading
    pub text: crate::enums::RichText,
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: i32,
}

/// A kicker; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockKicker {
    /// Kicker
    pub kicker: crate::enums::RichText,
}

/// A text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockParagraph {
    /// Paragraph text
    pub text: crate::enums::RichText,
}

/// A preformatted text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockPreformatted {
    /// Paragraph text
    pub text: crate::enums::RichText,
    /// Programming language for which the text needs to be formatted
    pub language: String,
}

/// The footer of a page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockFooter {
    /// Footer
    pub footer: crate::enums::RichText,
}

/// A "Thinking..." placeholder; for pending rich messages only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockThinking {
    /// Text of the placeholder
    pub text: crate::enums::RichText,
}

/// An empty block separating a page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockDivider {
}

/// A mathematical expression
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockMathematicalExpression {
    /// The expression in LaTeX format
    pub expression: String,
}

/// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAnchor {
    /// Name of the anchor
    pub name: String,
}

/// A list of data blocks
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockList {
    /// The items of the list
    pub items: Vec<crate::types::PageBlockListItem>,
}

/// A block quote
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockBlockQuote {
    /// Quote blocks
    pub blocks: Vec<crate::enums::PageBlock>,
    /// Quote credit; may be null if none
    pub credit: Option<crate::enums::RichText>,
}

/// An expandable block quote
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockExpandableBlockQuote {
    /// Text of the quote
    pub text: crate::enums::RichText,
    /// Quote credit; may be null if none
    pub credit: Option<crate::enums::RichText>,
}

/// A pull quote
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockPullQuote {
    /// Quote text
    pub text: crate::enums::RichText,
    /// Quote credit; may be null if none
    pub credit: Option<crate::enums::RichText>,
}

/// A voice note
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVoiceNote {
    /// Voice note
    pub voice_note: crate::types::VoiceNote,
    /// Voice note caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A page cover; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCover {
    /// Cover
    pub cover: crate::enums::PageBlock,
}

/// An embedded web page; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockEmbedded {
    /// URL of the embedded page, if available
    pub url: String,
    /// HTML-markup of the embedded page
    pub html: String,
    /// Poster photo, if available; may be null
    pub poster_photo: Option<crate::types::Photo>,
    /// Block width; 0 if unknown
    pub width: i32,
    /// Block height; 0 if unknown
    pub height: i32,
    /// Block caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
    /// True, if the block must be full width
    pub is_full_width: bool,
    /// True, if scrolling needs to be allowed
    pub allow_scrolling: bool,
}

/// An embedded post; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockEmbeddedPost {
    /// URL of the embedded post
    pub url: String,
    /// Post author
    pub author: String,
    /// Post author photo; may be null
    pub author_photo: Option<crate::types::Photo>,
    /// Point in time (Unix timestamp) when the post was created; 0 if unknown
    pub date: i32,
    /// Post content
    pub blocks: Vec<crate::enums::PageBlock>,
    /// Post caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A collage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCollage {
    /// Collage item contents
    pub blocks: Vec<crate::enums::PageBlock>,
    /// Block caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A slideshow
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSlideshow {
    /// Slideshow item contents
    pub blocks: Vec<crate::enums::PageBlock>,
    /// Block caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A table
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTable {
    /// Table caption; may be null if none
    pub caption: Option<crate::enums::RichText>,
    /// Table cells
    pub cells: Vec<Vec<crate::types::PageBlockTableCell>>,
    /// True, if the table is bordered
    pub is_bordered: bool,
    /// True, if the table is striped
    pub is_striped: bool,
    /// True, if table cells must have smaller indents
    pub is_compact: bool,
}

/// A collapsible block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockDetails {
    /// Always visible heading for the block
    pub header: crate::enums::RichText,
    /// Block contents
    pub blocks: Vec<crate::enums::PageBlock>,
    /// True, if the block is open by default
    pub is_open: bool,
}

/// Related articles; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockRelatedArticles {
    /// Block header
    pub header: crate::enums::RichText,
    /// List of related articles
    pub articles: Vec<crate::types::PageBlockRelatedArticle>,
}

/// A map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockMap {
    /// Location of the map center
    pub location: crate::types::Location,
    /// Map zoom level
    pub zoom: i32,
    /// Map width
    pub width: i32,
    /// Map height
    pub height: i32,
    /// Block caption; may be null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A list of buttons shown in a row
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockButtonRow {
    /// The buttons
    pub buttons: Vec<crate::types::InlineButton>,
    /// Horizontal alignment of the buttons; may be null if the buttons must be shown full-width
    pub align: Option<crate::enums::PageBlockHorizontalAlignment>,
}

/// Represents a block unsupported by the current application version
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockUnsupported {
}

/// Describes an instant view page for a web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WebPageInstantView {
    /// Content of the instant view page
    pub blocks: Vec<crate::enums::PageBlock>,
    /// Number of the instant view views; 0 if unknown
    pub view_count: i32,
    /// Version of the instant view; currently, can be 1 or 2
    pub version: i32,
    /// True, if the instant view must be shown from right to left
    pub is_rtl: bool,
    /// True, if the instant view contains the full page. A network request might be needed to get the full instant view
    pub is_full: bool,
    /// An internal link to be opened to leave feedback about the instant view
    pub feedback_link: crate::enums::InternalLinkType,
}

/// The link is a link to a media album consisting of photos and videos
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeAlbum {
    /// The list of album media
    pub media: Vec<crate::enums::LinkPreviewAlbumMedia>,
    /// Album caption
    pub caption: String,
}

/// The link is a link to an app at App Store or Google Play
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeApp {
    /// Photo for the app
    pub photo: crate::types::Photo,
}

/// The link is a link to a website
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeArticle {
    /// Article's main photo; may be null
    pub photo: Option<crate::types::Photo>,
}

/// The link is a link to a background. Link preview title and description are available only for filled backgrounds
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeBackground {
    /// Document with the background; may be null for filled backgrounds
    pub document: Option<crate::types::Document>,
    /// Type of the background; may be null if unknown
    pub background_type: Option<crate::enums::BackgroundType>,
}

/// The link is a link to a gift auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeGiftAuction {
    /// The gift
    pub gift: crate::types::Gift,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub auction_end_date: i32,
}

/// The link is a link to a gift collection
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeGiftCollection {
    /// Icons for some gifts from the collection; may be empty
    pub icons: Vec<crate::types::Sticker>,
}

/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeTheme {
    /// The list of files with theme description
    pub documents: Vec<crate::types::Document>,
    /// Settings for the cloud theme; may be null if unknown
    pub settings: Option<crate::types::ThemeSettings>,
}

/// The link preview type is unsupported yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeUnsupported {
}

/// The link is a link to an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeUpgradedGift {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

/// The link is a link to a voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeVoiceNote {
    /// The voice note
    pub voice_note: crate::types::VoiceNote,
}

/// The link is a link to a Web App
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeWebApp {
    /// Web App photo; may be null if none
    pub photo: Option<crate::types::Photo>,
}

/// Describes a link preview
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreview {
    /// Original URL of the link
    pub url: String,
    /// URL to display
    pub display_url: String,
    /// Short name of the website (e.g., Google Docs, App Store)
    pub site_name: String,
    /// Title of the content
    pub title: String,
    /// Description of the content
    pub description: crate::types::FormattedText,
    /// Author of the content
    pub author: String,
    /// Type of the link preview
    #[serde(rename = "type")]
    pub r#type: crate::enums::LinkPreviewType,
    /// True, if size of media in the preview can be changed
    pub has_large_media: bool,
    /// True, if large media preview must be shown; otherwise, the media preview must be shown small and only the first frame must be shown for videos
    pub show_large_media: bool,
    /// True, if media must be shown above link preview description; otherwise, the media must be shown below the description
    pub show_media_above_description: bool,
    /// True, if there is no need to show an ordinary open URL confirmation, when opening the URL from the preview, because the URL is shown in the message text in clear
    pub skip_confirmation: bool,
    /// True, if the link preview must be shown above message text; otherwise, the link preview must be shown below the message text
    pub show_above_text: bool,
    /// Version of instant view (currently, can be 1 or 2) for the web page; 0 if none
    pub instant_view_version: i32,
}

/// Contains information about a country
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CountryInfo {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// Native name of the country
    pub name: String,
    /// English name of the country
    pub english_name: String,
    /// An emoji for the flag of the country; may be empty if unknown
    pub flag_emoji: String,
    /// True, if the country must be hidden from the list of all countries
    pub is_hidden: bool,
    /// List of country calling codes
    pub calling_codes: Vec<String>,
}

/// Contains information about countries
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Countries {
    /// The list of countries
    pub countries: Vec<crate::types::CountryInfo>,
}

/// Contains information about a phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberInfo {
    /// Information about the country to which the phone number belongs; may be null
    pub country: Option<crate::types::CountryInfo>,
    /// The part of the phone number denoting country calling code or its part
    pub country_calling_code: String,
    /// The phone number without country calling code formatted accordingly to local rules. Expected digits are returned as '-', but even more digits might be entered by the user
    pub formatted_phone_number: String,
    /// True, if the phone number was bought at https:fragment.com and isn't tied to a SIM card. Information about the phone number can be received using getCollectibleItemInfo
    pub is_anonymous: bool,
}

/// A phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemTypePhoneNumber {
    /// The phone number
    pub phone_number: String,
}

/// Contains information about a collectible item and its last purchase
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemInfo {
    /// Point in time (Unix timestamp) when the item was purchased
    pub purchase_date: i32,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the item
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Individual URL for the item on https:fragment.com
    pub url: String,
}

/// Describes an action associated with a bank card number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BankCardActionOpenUrl {
    /// Action text
    pub text: String,
    /// The URL to be opened
    pub url: String,
}

/// Information about a bank card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BankCardInfo {
    /// Title of the bank card description
    pub title: String,
    /// Actions that can be done with the bank card number
    pub actions: Vec<crate::types::BankCardActionOpenUrl>,
}

/// Describes an address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Address {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line of the address
    pub street_line1: String,
    /// Second line of the address
    pub street_line2: String,
    /// Address postal code
    pub postal_code: String,
}

/// Describes an address of a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LocationAddress {
    /// A two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable; empty if unknown
    pub state: String,
    /// City; empty if unknown
    pub city: String,
    /// The address; empty if unknown
    pub street: String,
}

/// Portion of the price of a product (e.g., "delivery cost", "tax amount")
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LabeledPricePart {
    /// Label for this portion of the product price
    pub label: String,
    /// Currency amount in the smallest units of the currency
    pub amount: i64,
}

/// Order information
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OrderInfo {
    /// Name of the user
    pub name: String,
    /// Phone number of the user
    pub phone_number: String,
    /// Email address of the user
    pub email_address: String,
    /// Shipping address for this order; may be null
    pub shipping_address: Option<crate::types::Address>,
}

/// Contains information about saved payment credentials
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedCredentials {
    /// Unique identifier of the saved credentials
    pub id: String,
    /// Title of the saved credentials
    pub title: String,
}

/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsSaved {
    /// Identifier of the saved credentials
    pub saved_credentials_id: String,
}

/// Applies if a user enters new credentials on a payment provider website
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsNew {
    /// JSON-encoded data with the credential identifier from the payment provider
    pub data: String,
    /// True, if the credential identifier can be saved on the server side
    pub allow_save: bool,
}

/// Applies if a user enters new credentials using Apple Pay
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsApplePay {
    /// JSON-encoded data with the credential identifier
    pub data: String,
}

/// Applies if a user enters new credentials using Google Pay
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsGooglePay {
    /// JSON-encoded data with the credential identifier
    pub data: String,
}

/// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ValidatedOrderInfo {
    /// Temporary identifier of the order information
    pub order_info_id: String,
    /// Available shipping options
    pub shipping_options: Vec<crate::types::ShippingOption>,
}

/// The media is hidden until the invoice is paid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidMediaPreview {
    /// Media width; 0 if unknown
    pub width: i32,
    /// Media height; 0 if unknown
    pub height: i32,
    /// Media duration, in seconds; 0 if unknown
    pub duration: i32,
    /// Media minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
}

/// The media is unsupported
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidMediaUnsupported {
}

/// Represents a date according to the Gregorian calendar
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Date {
    /// Day of the month; 1-31
    pub day: i32,
    /// Month; 1-12
    pub month: i32,
    /// Year; 1-9999
    pub year: i32,
}

/// Contains the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PersonalDetails {
    /// First name of the user written in English; 1-255 characters
    pub first_name: String,
    /// Middle name of the user written in English; 0-255 characters
    pub middle_name: String,
    /// Last name of the user written in English; 1-255 characters
    pub last_name: String,
    /// Native first name of the user; 1-255 characters
    pub native_first_name: String,
    /// Native middle name of the user; 0-255 characters
    pub native_middle_name: String,
    /// Native last name of the user; 1-255 characters
    pub native_last_name: String,
    /// Birthdate of the user
    pub birthdate: crate::types::Date,
    /// Gender of the user, "male" or "female"
    pub gender: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
    pub country_code: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's residence country
    pub residence_country_code: String,
}

/// Contains encrypted Telegram Passport data credentials
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EncryptedCredentials {
    /// The encrypted credentials
    pub data: String,
    /// The decrypted data hash
    pub hash: String,
    /// Secret for data decryption, encrypted with the service's public key
    pub secret: String,
}

/// Don't show the date or time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateTimePartPrecisionNone {
}

/// Show the date or time in a short way (17.03.22 or 22:45)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateTimePartPrecisionShort {
}

/// Show the date or time in a long way (March 17, 2022 or 22:45:00)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateTimePartPrecisionLong {
}

/// The time must be shown relative to the current time ([in ] X seconds, minutes, hours, days, months, years [ago])
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateTimeFormattingTypeRelative {
}

/// The date and time must be shown as absolute timestamps
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DateTimeFormattingTypeAbsolute {
    /// The precision with which hours, minutes and seconds are shown
    pub time_precision: crate::enums::DateTimePartPrecision,
    /// The precision with which the date is shown
    pub date_precision: crate::enums::DateTimePartPrecision,
    /// True, if the day of week must be shown
    pub show_day_of_week: bool,
}

/// Addition of some text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DiffEntityTypeInsert {
}

/// Change of some text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DiffEntityTypeReplace {
    /// The old text
    pub old_text: String,
}

/// Removal of some text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DiffEntityTypeDelete {
}

/// A video note to be sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputVoiceNote {
    /// Voice note file to be sent. The voice note must be encoded with the Opus codec and stored inside an OGG container with a single audio channel, or be in MP3 or M4A format as regular audio
    pub voice_note: crate::enums::InputFile,
    /// Duration of the voice note, in seconds
    pub duration: i32,
    /// Waveform representation of the voice note in 5-bit format
    pub waveform: String,
}

/// Describes a paid media to be sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPaidMedia {
    /// Type of the media
    #[serde(rename = "type")]
    pub r#type: crate::enums::InputPaidMediaType,
    /// Photo or video to be sent
    pub media: crate::enums::InputFile,
    /// Media thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// File identifiers of the stickers added to the media, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Media width
    pub width: i32,
    /// Media height
    pub height: i32,
}

/// A section heading
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockSectionHeading {
    /// Text of the section heading
    pub text: crate::enums::RichText,
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: i32,
}

/// A text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockParagraph {
    /// Paragraph text
    pub text: crate::enums::RichText,
}

/// A preformatted text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockPreformatted {
    /// Paragraph text
    pub text: crate::enums::RichText,
    /// Programming language for which the text needs to be formatted
    pub language: String,
}

/// The footer of the page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockFooter {
    /// Footer
    pub footer: crate::enums::RichText,
}

/// A "Thinking..." placeholder; for pending rich messages only; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockThinking {
    /// Text of the placeholder
    pub text: crate::enums::RichText,
}

/// An empty block separating the page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockDivider {
}

/// A mathematical expression
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockMathematicalExpression {
    /// The expression in LaTeX format
    pub expression: String,
}

/// An invisible anchor
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockAnchor {
    /// Name of the anchor
    pub name: String,
}

/// A list of data blocks
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockList {
    /// The items of the list
    pub items: Vec<crate::types::InputPageBlockListItem>,
}

/// A block quote
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockBlockQuote {
    /// Quote blocks
    pub blocks: Vec<crate::enums::InputPageBlock>,
    /// Quote credit; pass null if none
    pub credit: Option<crate::enums::RichText>,
}

/// An expandable block quote
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockExpandableBlockQuote {
    /// Quote text
    pub text: crate::enums::RichText,
    /// Quote credit; pass null if none
    pub credit: Option<crate::enums::RichText>,
}

/// A pull quote
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockPullQuote {
    /// Quote text
    pub text: crate::enums::RichText,
    /// Quote credit; pass null if none
    pub credit: Option<crate::enums::RichText>,
}

/// A voice note
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockVoiceNote {
    /// The voice note to be sent
    pub voice_note: crate::types::InputVoiceNote,
    /// Voice note caption; pass null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A collage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockCollage {
    /// Collage item contents
    pub blocks: Vec<crate::enums::InputPageBlock>,
    /// Block caption; pass null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A slideshow
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockSlideshow {
    /// Slideshow item contents
    pub blocks: Vec<crate::enums::InputPageBlock>,
    /// Block caption; pass null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A table
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockTable {
    /// Table caption
    pub caption: crate::enums::RichText,
    /// Table cells
    pub cells: Vec<Vec<crate::types::PageBlockTableCell>>,
    /// Pass true if the table is bordered
    pub is_bordered: bool,
    /// Pass true if the table is striped
    pub is_striped: bool,
    /// Pass true if table cells must have smaller indents
    pub is_compact: bool,
}

/// A collapsible block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockDetails {
    /// Always visible heading for the block
    pub header: crate::enums::RichText,
    /// Block contents
    pub blocks: Vec<crate::enums::InputPageBlock>,
    /// True, if the block is open by default
    pub is_open: bool,
}

/// A map. The map's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockMap {
    /// Location of the map center
    pub location: crate::types::Location,
    /// Map zoom level; 0-24
    pub zoom: i32,
    /// Map width; 0-10000
    pub width: i32,
    /// Map height; 0-10000
    pub height: i32,
    /// Block caption; pass null if none
    pub caption: Option<crate::types::PageBlockCaption>,
}

/// A list of buttons shown in a row
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPageBlockButtonRow {
    /// The buttons
    pub buttons: Vec<crate::types::InlineButton>,
    /// Horizontal alignment of the buttons; pass null if the buttons must be shown full-width
    pub align: Option<crate::enums::PageBlockHorizontalAlignment>,
}

/// Describes the current weather
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CurrentWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
}

/// Describes a shortcut that can be used for a quick reply
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyShortcut {
    /// Unique shortcut identifier
    pub id: i32,
    /// The name of the shortcut that can be used to use the shortcut
    pub name: String,
    /// The first shortcut message
    pub first_message: crate::types::QuickReplyMessage,
    /// The total number of messages in the shortcut
    pub message_count: i32,
}

/// Represents a list of public forwards and reposts as a story of a message or a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicForwards {
    /// Approximate total number of messages and stories found
    pub total_count: i32,
    /// List of found public forwards and reposts
    pub forwards: Vec<crate::enums::PublicForward>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// The code is re-sent, because device verification has failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResendCodeReasonVerificationFailed {
    /// Cause of the verification failure, for example, "PLAY_SERVICES_NOT_AVAILABLE", "APNS_RECEIVE_TIMEOUT", or "APNS_INIT_FAILED"
    pub error_message: String,
}

/// Represents an RTMP URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RtmpUrl {
    /// The URL
    pub url: String,
    /// Stream key
    pub stream_key: String,
}

/// Settings for Firebase Authentication in the official Android application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseAuthenticationSettingsAndroid {
}

/// Settings for Firebase Authentication in the official iOS application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseAuthenticationSettingsIos {
    /// Device token from Apple Push Notification service
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
}

/// Contains settings for the authentication of the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberAuthenticationSettings {
    /// Pass true if the authentication code may be sent via a flash call to the specified phone number
    pub allow_flash_call: bool,
    /// Pass true if the authentication code may be sent via a missed call to the specified phone number
    pub allow_missed_call: bool,
    /// Pass true if the authenticated phone number is used on the current device
    pub is_current_phone_number: bool,
    /// Pass true if there is a SIM card in the current device, but it is not possible to check whether phone number matches
    pub has_unknown_phone_number: bool,
    /// For official applications only. True, if the application can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https:developers.google.com/identity/sms-retriever/ for more details
    pub allow_sms_retriever_api: bool,
    /// For official Android and iOS applications only; pass null otherwise. Settings for Firebase Authentication
    pub firebase_authentication_settings: Option<crate::enums::FirebaseAuthenticationSettings>,
    /// List of up to 20 authentication tokens, recently received in updateOption("authentication_token") in previously logged out sessions; for setAuthenticationPhoneNumber only
    pub authentication_tokens: Vec<String>,
}

/// The speech recognition is ongoing
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultPending {
    /// Partially recognized text
    pub partial_text: String,
}

/// The speech recognition failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultError {
    /// Recognition error. An error with a message "MSG_VOICE_TOO_LONG" is returned when media duration is too big to be recognized
    pub error: crate::types::Error,
}

/// Contains an HTTP URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct HttpUrl {
    /// The URL
    pub url: String,
}

/// Represents a link to an article or web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultArticle {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the result, if it exists
    pub url: String,
    /// Title of the result
    pub title: String,
    /// A short description of the result
    pub description: String,
    /// URL of the result thumbnail, if it exists
    pub thumbnail_url: String,
    /// Thumbnail width, if known
    pub thumbnail_width: i32,
    /// Thumbnail height, if known
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultGame {
    /// Unique identifier of the query result
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

/// Represents a point on the map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultLocation {
    /// Unique identifier of the query result
    pub id: String,
    /// Location result
    pub location: crate::types::Location,
    /// Amount of time relative to the message sent time until the location can be updated, in seconds
    pub live_period: i32,
    /// Title of the result
    pub title: String,
    /// URL of the result thumbnail, if it exists
    pub thumbnail_url: String,
    /// Thumbnail width, if known
    pub thumbnail_width: i32,
    /// Thumbnail height, if known
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents information about a venue
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultVenue {
    /// Unique identifier of the query result
    pub id: String,
    /// Venue result
    pub venue: crate::types::Venue,
    /// URL of the result thumbnail, if it exists
    pub thumbnail_url: String,
    /// Thumbnail width, if known
    pub thumbnail_width: i32,
    /// Thumbnail height, if known
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultVoiceNote {
    /// Unique identifier of the query result
    pub id: String,
    /// Title of the voice note
    pub title: String,
    /// The URL of the voice note file
    pub voice_note_url: String,
    /// Duration of the voice note, in seconds
    pub voice_note_duration: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageVoiceNote, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents a link to an article or web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the result, if it exists
    pub url: String,
    /// Title of the result
    pub title: String,
    /// A short description of the result
    pub description: String,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

/// Represents a point on the map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier of the query result
    pub id: String,
    /// Location result
    pub location: crate::types::Location,
    /// Title of the result
    pub title: String,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

/// Represents information about a venue
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier of the query result
    pub id: String,
    /// Venue result
    pub venue: crate::types::Venue,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

/// Represents information about a game
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier of the query result
    pub id: String,
    /// Game result
    pub game: crate::types::Game,
}

/// Represents a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVoiceNote {
    /// Unique identifier of the query result
    pub id: String,
    /// Voice note
    pub voice_note: crate::types::VoiceNote,
    /// Title of the voice note
    pub title: String,
}

/// Describes the button that opens a Web App by calling getWebAppUrl
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButtonTypeWebApp {
    /// An HTTP URL to pass to getWebAppUrl
    pub url: String,
}

/// Represents a button to be shown above inline query results
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButton {
    /// The text of the button
    pub text: String,
    /// Type of the button
    #[serde(rename = "type")]
    pub r#type: crate::enums::InlineQueryResultsButtonType,
}

/// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResults {
    /// Unique identifier of the inline query
    #[serde_as(as = "DisplayFromStr")]
    pub inline_query_id: i64,
    /// Button to be shown above inline query results; may be null
    pub button: Option<crate::types::InlineQueryResultsButton>,
    /// Results of the query
    pub results: Vec<crate::enums::InlineQueryResult>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains the result of a custom request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomRequestResult {
    /// A JSON-serialized result
    pub result: String,
}

/// Contains one row of the game high score table
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GameHighScore {
    /// Position in the high score table
    pub position: i32,
    /// User identifier
    pub user_id: i64,
    /// User score
    pub score: i32,
}

/// Contains a list of game high scores
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GameHighScores {
    /// A list of game high scores
    pub scores: Vec<crate::types::GameHighScore>,
}

/// An ordinary language pack string
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStringValueOrdinary {
    /// String value
    pub value: String,
}

/// A language pack string which has different forms based on the number of some object it mentions. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStringValuePluralized {
    /// Value for zero objects
    pub zero_value: String,
    /// Value for one object
    pub one_value: String,
    /// Value for two objects
    pub two_value: String,
    /// Value for few objects
    pub few_value: String,
    /// Value for many objects
    pub many_value: String,
    /// Default value
    pub other_value: String,
}

/// A deleted language pack string, the value must be taken from the built-in English language pack
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStringValueDeleted {
}

/// Represents one language pack string
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackString {
    /// String key
    pub key: String,
    /// String value; pass null if the string needs to be taken from the built-in English language pack
    pub value: Option<crate::enums::LanguagePackStringValue>,
}

/// Contains a list of language pack strings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStrings {
    /// A list of language pack strings
    pub strings: Vec<crate::types::LanguagePackString>,
}

/// Contains information about a language pack
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackInfo {
    /// Unique language pack identifier
    pub id: String,
    /// Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it must be fetched from base language pack. Unsupported in custom language packs
    pub base_language_pack_id: String,
    /// Language name
    pub name: String,
    /// Name of the language in that language
    pub native_name: String,
    /// A language code to be used to apply plural forms. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
    pub plural_code: String,
    /// True, if the language pack is official
    pub is_official: bool,
    /// True, if the language pack strings are RTL
    pub is_rtl: bool,
    /// True, if the language pack is a beta language pack
    pub is_beta: bool,
    /// True, if the language pack is installed by the current user
    pub is_installed: bool,
    /// Total number of non-deleted strings from the language pack
    pub total_string_count: i32,
    /// Total number of translated strings from the language pack
    pub translated_string_count: i32,
    /// Total number of non-deleted strings from the language pack available locally
    pub local_string_count: i32,
    /// Link to language translation interface; empty for custom local language packs
    pub translation_url: String,
}

/// Contains information about the current localization target
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LocalizationTargetInfo {
    /// List of available language packs for this application
    pub language_packs: Vec<crate::types::LanguagePackInfo>,
}

/// A purchase through App Store
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoreTransactionAppStore {
    /// App Store receipt
    pub receipt: String,
}

/// A purchase through Google Play
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoreTransactionGooglePlay {
    /// Application package name
    pub package_name: String,
    /// Identifier of the purchased store product
    pub store_product_id: String,
    /// Google Play purchase token
    pub purchase_token: String,
}

/// A token for Firebase Cloud Messaging
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
    /// Device registration token; may be empty to deregister a device
    pub token: String,
    /// True, if push notifications must be additionally encrypted
    pub encrypt: bool,
}

/// A token for Apple Push Notification service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenApplePush {
    /// Device token; may be empty to deregister a device
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
}

/// A token for Apple Push Notification service VoIP notifications
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenApplePushVoIp {
    /// Device token; may be empty to deregister a device
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
    /// True, if push notifications must be additionally encrypted
    pub encrypt: bool,
}

/// A token for Windows Push Notification Services
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenWindowsPush {
    /// The access token that will be used to send notifications; may be empty to deregister a device
    pub access_token: String,
}

/// A token for Microsoft Push Notification Service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenMicrosoftPush {
    /// Push notification channel URI; may be empty to deregister a device
    pub channel_uri: String,
}

/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenMicrosoftPushVoIp {
    /// Push notification channel URI; may be empty to deregister a device
    pub channel_uri: String,
}

/// A token for web Push API
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenWebPush {
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
    pub endpoint: String,
    /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key
    pub p256dh_base64url: String,
    /// Base64url-encoded authentication secret
    pub auth_base64url: String,
}

/// A token for Simple Push API for Firefox OS
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenSimplePush {
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
    pub endpoint: String,
}

/// A token for Ubuntu Push Client service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenUbuntuPush {
    /// Token; may be empty to deregister a device
    pub token: String,
}

/// A token for BlackBerry Push Service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenBlackBerryPush {
    /// Token; may be empty to deregister a device
    pub token: String,
}

/// A token for Tizen Push Service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenTizenPush {
    /// Push service registration identifier; may be empty to deregister a device
    pub reg_id: String,
}

/// A token for HUAWEI Push Service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenHuaweiPush {
    /// Device registration token; may be empty to deregister a device
    pub token: String,
    /// True, if push notifications must be additionally encrypted
    pub encrypt: bool,
}

/// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushReceiverId {
    /// The globally unique identifier of push notification subscription
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
}

/// Describes a solid fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillSolid {
    /// A color of the background in the RGB format
    pub color: i32,
}

/// Describes a gradient fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillGradient {
    /// A top color of the background in the RGB format
    pub top_color: i32,
    /// A bottom color of the background in the RGB format
    pub bottom_color: i32,
    /// Clockwise rotation angle of the gradient, in degrees; 0-359. Must always be divisible by 45
    pub rotation_angle: i32,
}

/// Describes a freeform gradient fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of 3 or 4 colors of the freeform gradient in the RGB format
    pub colors: Vec<i32>,
}

/// A wallpaper in JPEG format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeWallpaper {
    /// True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12
    pub is_blurred: bool,
    /// True, if the background needs to be slightly moved when device is tilted
    pub is_moving: bool,
}

/// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypePattern {
    /// Fill of the background
    pub fill: crate::enums::BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: i32,
    /// True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    pub is_inverted: bool,
    /// True, if the background needs to be slightly moved when device is tilted
    pub is_moving: bool,
}

/// A filled background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: crate::enums::BackgroundFill,
}

/// A background from a local file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundLocal {
    /// Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns
    pub background: crate::enums::InputFile,
}

/// A background from the server
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundRemote {
    /// The background identifier
    #[serde_as(as = "DisplayFromStr")]
    pub background_id: i64,
}

/// A background previously set in the chat; for chat backgrounds only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBackgroundPrevious {
    /// Identifier of the message with the background
    pub message_id: i64,
}

/// Describes a time zone
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeZone {
    /// Unique time zone identifier
    pub id: String,
    /// Time zone name
    pub name: String,
    /// Current UTC time offset for the time zone
    pub utc_time_offset: i32,
}

/// Contains a list of time zones
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TimeZones {
    /// A list of time zones
    pub time_zones: Vec<crate::types::TimeZone>,
}

/// Contains a list of hashtags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Hashtags {
    /// A list of hashtags
    pub hashtags: Vec<String>,
}

/// The session can be used
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanTransferOwnershipResultOk {
}

/// The 2-step verification needs to be enabled first
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanTransferOwnershipResultPasswordNeeded {
}

/// The 2-step verification was enabled recently, user needs to wait
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanTransferOwnershipResultPasswordTooFresh {
    /// Time left before the session can be used to transfer ownership of a chat, in seconds
    pub retry_after: i32,
}

/// The session was created recently, user needs to wait
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanTransferOwnershipResultSessionTooFresh {
    /// Time left before the session can be used to transfer ownership of a chat, in seconds
    pub retry_after: i32,
}

/// The password was reset
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResetPasswordResultOk {
}

/// The password reset request is pending
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResetPasswordResultPending {
    /// Point in time (Unix timestamp) after which the password can be reset immediately using resetPassword
    pub pending_reset_date: i32,
}

/// The password reset request was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResetPasswordResultDeclined {
    /// Point in time (Unix timestamp) when the password reset can be retried
    pub retry_date: i32,
}

/// Describes a proxy server
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Proxy {
    /// Proxy server domain or IP address
    pub server: String,
    /// Proxy server port
    pub port: i32,
    /// Type of the proxy
    #[serde(rename = "type")]
    pub r#type: crate::enums::ProxyType,
}

/// Represents a boolean option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueBoolean {
    /// The value of the option
    pub value: bool,
}

/// Represents an unknown option or an option which has a default value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueEmpty {
}

/// Represents an integer option
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueInteger {
    /// The value of the option
    #[serde_as(as = "DisplayFromStr")]
    pub value: i64,
}

/// Represents a string option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueString {
    /// The value of the option
    pub value: String,
}

/// Represents one member of a JSON object
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JsonObjectMember {
    /// Member's key
    pub key: String,
    /// Member's value
    pub value: crate::enums::JsonValue,
}

/// Represents a null JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueNull {
}

/// Represents a boolean JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueBoolean {
    /// The value
    pub value: bool,
}

/// Represents a numeric JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueNumber {
    /// The value
    pub value: f64,
}

/// Represents a string JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueString {
    /// The value
    pub value: String,
}

/// Represents a JSON array
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueArray {
    /// The list of array elements
    pub values: Vec<crate::enums::JsonValue>,
}

/// Represents a JSON object
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueObject {
    /// The list of object members
    pub members: Vec<crate::types::JsonObjectMember>,
}

/// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReadDatePrivacySettings {
    /// True, if message read date is shown to other users in private chats. If false and the current user isn't a Telegram Premium user, then they will not be able to see other's message read date
    pub show_read_date: bool,
}

/// Contains information about the period of inactivity after which the current user's account will automatically be deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccountTtl {
    /// Number of days of inactivity before the account will be flagged for deletion; 30-730 days
    pub days: i32,
}

/// A regular session from a device
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionTypeDevice {
    /// Unique identifier of the session. Use terminateSession to terminate it or confirmSession to confirm it if it isn't confirmed yet
    #[serde_as(as = "DisplayFromStr")]
    pub session_id: i64,
}

/// The session is running on an Android device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeAndroid {
}

/// The session is running on a generic Apple device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeApple {
}

/// The session is running on the Brave browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeBrave {
}

/// The session is running on the Chrome browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeChrome {
}

/// The session is running on the Edge browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeEdge {
}

/// The session is running on the Firefox browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeFirefox {
}

/// The session is running on an iPad device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeIpad {
}

/// The session is running on an iPhone device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeIphone {
}

/// The session is running on a Linux device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeLinux {
}

/// The session is running on a Mac device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeMac {
}

/// The session is running on the Opera browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeOpera {
}

/// The session is running on the Safari browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeSafari {
}

/// The session is running on an Ubuntu device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeUbuntu {
}

/// The session is running on an unknown type of device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeUnknown {
}

/// The session is running on the Vivaldi browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeVivaldi {
}

/// The session is running on a Windows device
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeWindows {
}

/// The session is running on an Xbox console
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionDeviceTypeXbox {
}

/// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Session {
    /// Session identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// True, if this session is the current session
    pub is_current: bool,
    /// True, if a 2-step verification password is needed to complete authorization of the session
    pub is_password_pending: bool,
    /// True, if the session wasn't confirmed from another session
    pub is_unconfirmed: bool,
    /// True, if incoming secret chats can be accepted by the session
    pub can_accept_secret_chats: bool,
    /// True, if incoming calls can be accepted by the session
    pub can_accept_calls: bool,
    /// Session device type based on the system and application version, which can be used to display a corresponding icon
    pub device_type: crate::enums::SessionDeviceType,
    /// Telegram API identifier, as provided by the application
    pub api_id: i32,
    /// Name of the application, as provided by the application
    pub application_name: String,
    /// The version of the application, as provided by the application
    pub application_version: String,
    /// True, if the application is an official application or uses the api_id of an official application
    pub is_official_application: bool,
    /// Model of the device the application has been run or is running on, as provided by the application
    pub device_model: String,
    /// Operating system the application has been run or is running on, as provided by the application
    pub platform: String,
    /// Version of the operating system the application has been run or is running on, as provided by the application
    pub system_version: String,
    /// Point in time (Unix timestamp) when the user has logged in
    pub log_in_date: i32,
    /// Point in time (Unix timestamp) when the session was last used
    pub last_active_date: i32,
    /// IP address from which the session was created, in human-readable format
    pub ip_address: String,
    /// A human-readable description of the location from which the session was created, based on the IP address
    pub location: String,
}

/// Contains a list of sessions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Sessions {
    /// List of sessions
    pub sessions: Vec<crate::types::Session>,
    /// Number of days of inactivity before sessions will automatically be terminated; 1-366 days
    pub inactive_session_ttl_days: i32,
}

/// Contains information about an unconfirmed session
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnconfirmedSession {
    /// Session type
    #[serde(rename = "type")]
    pub r#type: crate::enums::SessionType,
    /// Point in time (Unix timestamp) when the user has logged in or the business bot was connected
    pub date: i32,
    /// Model of the device that was used for the session creation, as provided by the application
    pub device_model: String,
    /// A human-readable description of the location from which the session was created, based on the IP address
    pub location: String,
}

/// Contains information about one website the current user is logged in with Telegram
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedWebsite {
    /// Website identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// The domain name of the website
    pub domain_name: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// The version of a browser used to log in
    pub browser: String,
    /// Operating system the browser is running on
    pub platform: String,
    /// Point in time (Unix timestamp) when the user was logged in
    pub log_in_date: i32,
    /// Point in time (Unix timestamp) when obtained authorization was last used
    pub last_active_date: i32,
    /// IP address from which the user was logged in, in human-readable format
    pub ip_address: String,
    /// Human-readable description of a country and a region from which the user was logged in, based on the IP address
    pub location: String,
}

/// Contains a list of websites the current user is logged in with Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedWebsites {
    /// List of connected websites
    pub websites: Vec<crate::types::ConnectedWebsite>,
}

/// The chat contains spam messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonSpam {
}

/// The chat promotes violence
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonViolence {
}

/// The chat contains pornographic messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonPornography {
}

/// The chat has child abuse related content
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonChildAbuse {
}

/// The chat contains copyrighted content
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonCopyright {
}

/// The location-based chat is unrelated to its stated location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonUnrelatedLocation {
}

/// The chat represents a fake account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonFake {
}

/// The chat has illegal drugs related content
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonIllegalDrugs {
}

/// The chat contains messages with personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonPersonalDetails {
}

/// A custom reason provided by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportReasonCustom {
}

/// The appearance section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionAppearance {
    /// Subsection of the section; may be one of
    /// "", "themes", "themes/edit", "themes/create", "wallpapers", "wallpapers/edit", "wallpapers/set",
    /// "wallpapers/choose-photo", "your-color/profile", "your-color/profile/add-icons", "your-color/profile/use-gift",
    /// "your-color/profile/reset", "your-color/name", "your-color/name/add-icons", "your-color/name/use-gift",
    /// "night-mode", "auto-night-mode", "text-size", "text-size/use-system", "message-corners", "animations",
    /// "stickers-and-emoji", "stickers-and-emoji/edit", "stickers-and-emoji/trending", "stickers-and-emoji/archived",
    /// "stickers-and-emoji/archived/edit", "stickers-and-emoji/emoji", "stickers-and-emoji/emoji/edit",
    /// "stickers-and-emoji/emoji/archived", "stickers-and-emoji/emoji/archived/edit", "stickers-and-emoji/emoji/suggest",
    /// "stickers-and-emoji/emoji/quick-reaction", "stickers-and-emoji/emoji/quick-reaction/choose",
    /// "stickers-and-emoji/suggest-by-emoji", "stickers-and-emoji/large-emoji", "stickers-and-emoji/dynamic-order",
    /// "stickers-and-emoji/emoji/show-more", "app-icon", "tap-for-next-media"
    pub subsection: String,
}

/// The "Ask a question" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionAskQuestion {
}

/// The data and storage settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionDataAndStorage {
    /// Subsection of the section; may be one of
    /// "", "storage", "storage/edit", "storage/auto-remove", "storage/clear-cache", "storage/max-cache", "usage",
    /// "usage/mobile", "usage/wifi", "usage/reset", "usage/roaming", "auto-download/mobile",
    /// "auto-download/mobile/enable", "auto-download/mobile/usage", "auto-download/mobile/photos",
    /// "auto-download/mobile/stories", "auto-download/mobile/videos", "auto-download/mobile/files", "auto-download/wifi",
    /// "auto-download/wifi/enable", "auto-download/wifi/usage", "auto-download/wifi/photos",
    /// "auto-download/wifi/stories", "auto-download/wifi/videos", "auto-download/wifi/files", "auto-download/roaming",
    /// "auto-download/roaming/enable", "auto-download/roaming/usage", "auto-download/roaming/photos",
    /// "auto-download/roaming/stories", "auto-download/roaming/videos", "auto-download/roaming/files",
    /// "auto-download/reset", "save-to-photos/chats", "save-to-photos/chats/max-video-size",
    /// "save-to-photos/chats/add-exception", "save-to-photos/chats/delete-all", "save-to-photos/groups",
    /// "save-to-photos/groups/max-video-size", "save-to-photos/groups/add-exception", "save-to-photos/groups/delete-all",
    /// "save-to-photos/channels", "save-to-photos/channels/max-video-size", "save-to-photos/channels/add-exception",
    /// "save-to-photos/channels/delete-all", "less-data-calls", "open-links", "share-sheet",
    /// "share-sheet/suggested-chats", "share-sheet/suggest-by", "share-sheet/reset", "saved-edited-photos",
    /// "pause-music", "raise-to-listen", "raise-to-speak", "show-18-content", "proxy", "proxy/edit", "proxy/use-proxy",
    /// "proxy/add-proxy", "proxy/share-list", "proxy/use-for-calls"
    pub subsection: String,
}

/// The Devices section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionDevices {
    /// Subsection of the section; may be one of
    /// "", "edit", "link-desktop", "terminate-sessions", "auto-terminate"
    pub subsection: String,
}

/// The FAQ section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionFaq {
}

/// The "Telegram Features" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionFeatures {
}

/// The in-app browser settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionInAppBrowser {
    /// Subsection of the section; may be one of
    /// "", "enable-browser", "clear-cookies", "clear-cache", "history", "clear-history", "never-open", "clear-list", "search"
    pub subsection: String,
}

/// The application language section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionLanguage {
    /// Subsection of the section; may be one of "", "show-button" for Show Translate Button toggle,
    /// "translate-chats" for Translate Entire Chats toggle, "do-not-translate" - for Do Not Translate language list
    pub subsection: String,
}

/// The TON Gram balance and transaction section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionMyGrams {
}

/// The power saving settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPowerSaving {
    /// Subsection of the section; may be one of
    /// "", "videos", "gifs", "stickers", "emoji", "effects", "preload", "background", "call-animations", "particles", "transitions"
    pub subsection: String,
}

/// The privacy and security section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPrivacyAndSecurity {
    /// Subsection of the section; may be one of
    /// "", "blocked", "blocked/edit", "blocked/block-user", "blocked/block-user/chats", "blocked/block-user/contacts",
    /// "active-websites", "active-websites/edit", "active-websites/disconnect-all", "passcode", "passcode/disable",
    /// "passcode/change", "passcode/auto-lock", "passcode/face-id", "passcode/fingerprint", "2sv", "2sv/change",
    /// "2sv/disable", "2sv/change-email", "passkey", "passkey/create", "auto-delete", "auto-delete/set-custom",
    /// "login-email", "phone-number", "phone-number/never", "phone-number/always", "last-seen", "last-seen/never",
    /// "last-seen/always", "last-seen/hide-read-time", "profile-photos", "profile-photos/never", "profile-photos/always",
    /// "profile-photos/set-public", "profile-photos/update-public", "profile-photos/remove-public", "bio", "bio/never",
    /// "bio/always", "gifts", "gifts/show-icon", "gifts/never", "gifts/always", "gifts/accepted-types", "birthday",
    /// "birthday/add", "birthday/never", "birthday/always", "saved-music", "saved-music/never", "saved-music/always",
    /// "forwards", "forwards/never", "forwards/always", "calls", "calls/never", "calls/always", "calls/p2p",
    /// "calls/p2p/never", "calls/p2p/always", "calls/ios-integration", "voice", "voice/never", "voice/always",
    /// "messages", "messages/set-price", "messages/exceptions", "invites", "invites/never", "invites/always",
    /// "self-destruct", "data-settings", "data-settings/sync-contacts", "data-settings/delete-synced",
    /// "data-settings/suggest-contacts", "data-settings/delete-cloud-drafts", "data-settings/clear-payment-info",
    /// "data-settings/link-previews", "data-settings/bot-settings", "data-settings/map-provider", "archive-and-mute"
    pub subsection: String,
}

/// The "Privacy Policy" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPrivacyPolicy {
}

/// The current user's QR code section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionQrCode {
    /// Subsection of the section; may be one of
    /// "", "share", "scan"
    pub subsection: String,
}

/// Search in Settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionSearch {
}

/// The "Send a gift" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionSendGift {
    /// Subsection of the section; may be one of
    /// "", "self"
    pub subsection: String,
}

/// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeAuthenticationCode {
    /// The authentication code
    pub code: String,
}

/// The link is a link to a background. Call searchBackground with the given background name to process the link.
/// If background is found and the user wants to apply it, then call setDefaultBackground
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBackground {
    /// Name of the background
    pub background_name: String,
}

/// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot,
/// ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGame {
    /// Username of the bot that owns the game
    pub bot_username: String,
    /// Short name of the game
    pub game_short_name: String,
}

/// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGiftAuction {
    /// Unique identifier of the auction
    pub auction_id: String,
}

/// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier
/// and the given collection identifier, then show the collection if received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGiftCollection {
    /// Username of the owner of the gift collection
    pub gift_owner_username: String,
    /// Gift collection identifier
    pub collection_id: i32,
}

/// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link.
/// If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeInstantView {
    /// URL to be passed to getWebPageInstantView
    pub url: String,
    /// An URL to open if getWebPageInstantView fails
    pub fallback_url: String,
}

/// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link.
/// If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeLanguagePack {
    /// Language pack identifier
    pub language_pack_id: String,
}

/// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App.
/// If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu,
/// show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu,
/// then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot.
/// Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMainWebApp {
    /// Username of the bot
    pub bot_username: String,
    /// Start parameter to be passed to getMainWebApp
    pub start_parameter: String,
    /// The mode to be passed to getMainWebApp
    pub mode: crate::enums::WebAppOpenMode,
}

/// The link is an OAuth link. Call getOauthLinkInfo with the given URL to process the link if the link was received from outside of the application; otherwise, ignore it.
/// After getOauthLinkInfo, show the user confirmation dialog and process it with checkOauthRequestMatchCode, acceptOauthRequest or declineOauthRequest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeOauth {
    /// URL to be passed to getOauthLinkInfo
    pub url: String,
}

/// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link.
/// If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePhoneNumberConfirmation {
    /// Hash value from the link
    pub hash: String,
    /// Phone number value from the link
    pub phone_number: String,
}

/// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeProxy {
    /// The proxy; may be null if the proxy is unsupported, in which case an alert can be shown to the user
    pub proxy: Option<crate::types::Proxy>,
}

/// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to
/// "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeQrCodeAuthentication {
}

/// The link forces restore of App Store purchases when opened. For official iOS application only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeRestorePurchases {
}

/// The link is a link to the global chat and messages search field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeSearch {
}

/// The link is a link to application settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeSettings {
    /// Section of the application settings to open; may be null if none
    pub section: Option<crate::enums::SettingsSection>,
}

/// The link is a link to the Telegram Star purchase section of the application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStarPurchase {
    /// The number of Telegram Stars that must be owned by the user
    pub star_count: i64,
    /// Purpose of Telegram Star purchase. Arbitrary string specified by the server, for example, "subs" if the Telegram Stars are required to extend channel subscriptions
    pub purpose: String,
}

/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeTheme {
    /// Name of the theme
    pub theme_name: String,
}

/// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUnknownDeepLink {
    /// Link to be passed to getDeepLinkInfo
    pub link: String,
}

/// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUpgradedGift {
    /// Name of the unique gift
    pub name: String,
}

/// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message.
/// Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed.
/// If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog
/// and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
/// Then, call getWebAppLinkUrl and open the returned URL as a Web App
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeWebApp {
    /// Username of the bot that owns the Web App
    pub bot_username: String,
    /// Short name of the Web App
    pub web_app_short_name: String,
    /// Start parameter to be passed to getWebAppLinkUrl
    pub start_parameter: String,
    /// The mode in which the Web App must be opened
    pub mode: crate::enums::WebAppOpenMode,
}

/// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BlockListMain {
}

/// Contains the exact storage usage statistics split by chats and file type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatistics {
    /// Total size of files, in bytes
    pub size: i64,
    /// Total number of files
    pub count: i32,
    /// Statistics split by chats
    pub by_chat: Vec<crate::types::StorageStatisticsByChat>,
}

/// Contains approximate storage usage statistics, excluding files of unknown file type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatisticsFast {
    /// Approximate total size of files, in bytes
    pub files_size: i64,
    /// Approximate number of files
    pub file_count: i32,
    /// Size of the database
    pub database_size: i64,
    /// Size of the language pack database
    pub language_pack_database_size: i64,
    /// Size of the TDLib internal log
    pub log_size: i64,
}

/// Contains database statistics
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DatabaseStatistics {
    /// Database statistics in an unspecified human-readable format
    pub statistics: String,
}

/// The network is not available
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkTypeNone {
}

/// A mobile network
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkTypeMobile {
}

/// A mobile roaming network
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkTypeMobileRoaming {
}

/// A Wi-Fi network
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkTypeWiFi {
}

/// A different network type (e.g., Ethernet network)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkTypeOther {
}

/// A full list of available network statistic entries
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NetworkStatistics {
    /// Point in time (Unix timestamp) from which the statistics are collected
    pub since_date: i32,
    /// Network statistics entries
    pub entries: Vec<crate::enums::NetworkStatisticsEntry>,
}

/// Contains auto-download settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoDownloadSettings {
    /// True, if the auto-download is enabled
    pub is_auto_download_enabled: bool,
    /// The maximum size of a photo file to be auto-downloaded, in bytes
    pub max_photo_file_size: i32,
    /// The maximum size of a video file to be auto-downloaded, in bytes
    pub max_video_file_size: i64,
    /// The maximum size of other file types to be auto-downloaded, in bytes
    pub max_other_file_size: i64,
    /// The maximum suggested bitrate for uploaded videos, in kbit/s
    pub video_upload_bitrate: i32,
    /// True, if the beginning of video files needs to be preloaded for instant playback
    pub preload_large_videos: bool,
    /// True, if the next audio track needs to be preloaded while the user is listening to an audio file
    pub preload_next_audio: bool,
    /// True, if stories need to be preloaded
    pub preload_stories: bool,
    /// True, if "use less data for calls" option needs to be enabled
    pub use_less_data_for_calls: bool,
}

/// Contains auto-download settings presets for the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoDownloadSettingsPresets {
    /// Preset with lowest settings; expected to be used by default when roaming
    pub low: crate::types::AutoDownloadSettings,
    /// Preset with medium settings; expected to be used by default when using mobile data
    pub medium: crate::types::AutoDownloadSettings,
    /// Preset with highest settings; expected to be used by default when connected on Wi-Fi
    pub high: crate::types::AutoDownloadSettings,
}

/// Contains autosave settings for an autosave settings scope
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ScopeAutosaveSettings {
    /// True, if photo autosave is enabled
    pub autosave_photos: bool,
    /// True, if video autosave is enabled
    pub autosave_videos: bool,
    /// The maximum size of a video file to be autosaved, in bytes; 512 KB - 4000 MB
    pub max_video_file_size: i64,
}

/// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsException {
    /// Chat identifier
    pub chat_id: i64,
    /// Autosave settings for the chat
    pub settings: crate::types::ScopeAutosaveSettings,
}

/// Describes autosave settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettings {
    /// Default autosave settings for private chats
    pub private_chat_settings: crate::types::ScopeAutosaveSettings,
    /// Default autosave settings for basic group and supergroup chats
    pub group_settings: crate::types::ScopeAutosaveSettings,
    /// Default autosave settings for channel chats
    pub channel_settings: crate::types::ScopeAutosaveSettings,
    /// Autosave settings for specific chats
    pub exceptions: Vec<crate::types::AutosaveSettingsException>,
}

/// Describes an exception for built-in browser usage
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebDomainException {
    /// URL for which the exception is done
    pub url: String,
    /// Domain of the URL. All URLs on the domain and subdomains of the domain are subject to the exception
    pub domain: String,
    /// Title of the website
    pub title: String,
    /// Identifier of the custom emoji with favicon of the website; may be 0 if unknown, in which case the first letter of the domain must be used
    #[serde_as(as = "DisplayFromStr")]
    pub favicon_custom_emoji_id: i64,
}

/// Describes web browser settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebBrowserSettings {
    /// True, if links are opened in an external browser by default
    pub open_external_browser: bool,
    /// The list of websites which must always be opened in an external browser
    pub external_exceptions: Vec<crate::types::WebDomainException>,
    /// The list of websites which must always be opened in the in-app browser
    pub in_app_exceptions: Vec<crate::types::WebDomainException>,
    /// True, if a close button must be shown in the in-app browser; for Android app only
    pub display_close_button: bool,
}

/// An external web browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebBrowserTypeExternal {
}

/// The in-app browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebBrowserTypeInApp {
}

/// Waiting for the network to become available. Use setNetworkType to change the available network type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectionStateWaitingForNetwork {
}

/// Establishing a connection with a proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectionStateConnectingToProxy {
}

/// Establishing a connection to the Telegram servers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectionStateConnecting {
}

/// Downloading data expected to be received while the application was offline
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectionStateUpdating {
}

/// There is a working connection to the Telegram servers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectionStateReady {
}

/// Describes parameters for age verification of the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AgeVerificationParameters {
    /// The minimum age required to view restricted content
    pub min_age: i32,
    /// Username of the bot which main Web App may be used to verify age of the user
    pub verification_bot_username: String,
    /// Unique name for the country or region, which legislation required age verification. May be used to get the corresponding localization key
    pub country: String,
}

/// Contains 0-based match position
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPosition {
    /// The position of the match
    pub position: i32,
}

/// Contains 0-based positions of matched objects
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPositions {
    /// Total number of matched objects
    pub total_count: i32,
    /// The positions of the matched objects
    pub positions: Vec<i32>,
}

/// Represents a URL linking to an internal Telegram entity
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TmeUrl {
    /// URL
    pub url: String,
    /// Type of the URL
    #[serde(rename = "type")]
    pub r#type: crate::enums::TmeUrlType,
}

/// Contains a list of t.me URLs
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrls {
    /// List of URLs
    pub urls: Vec<crate::types::TmeUrl>,
}

/// Suggests the user to check whether they still remember their 2-step verification password
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionCheckPassword {
}

/// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionCheckPhoneNumber {
}

/// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionViewChecksHint {
}

/// Suggests the user to convert specified supergroup to a broadcast group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionConvertToBroadcastGroup {
    /// Supergroup identifier
    pub supergroup_id: i64,
}

/// Suggests the user to set a 2-step verification password to be able to log in again
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionSetPassword {
    /// The number of days to pass between consecutive authorizations if the user declines to set password; if 0, then the user is advised to set the password for security reasons
    pub authorization_delay: i32,
}

/// Suggests the user to set birthdate
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionSetBirthdate {
}

/// A custom suggestion to be shown at the top of the chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionCustom {
    /// Unique name of the suggestion
    pub name: String,
    /// Title of the suggestion
    pub title: crate::types::FormattedText,
    /// Description of the suggestion
    pub description: crate::types::FormattedText,
    /// The link to open when the suggestion is clicked
    pub url: String,
}

/// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionSetLoginEmailAddress {
    /// True, if the suggested action can be hidden using hideSuggestedAction. Otherwise, the user must not be able to use the application without setting up the email address
    pub can_be_hidden: bool,
}

/// Suggests the user to add a passkey for login using addLoginPasskey
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionAddLoginPasskey {
}

/// Contains a counter
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Count {
    /// Count
    pub count: i32,
}

/// Contains some binary data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Data {
    /// Data
    pub data: String,
}

/// Contains a value representing a number of seconds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Seconds {
    /// Number of seconds
    pub seconds: f64,
}

/// Contains a number of Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarCount {
    /// Number of Telegram Stars
    pub star_count: i64,
}

/// Contains information about a tg: deep link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeepLinkInfo {
    /// Text to be shown to the user
    pub text: crate::types::FormattedText,
    /// True, if the user must be asked to update the application
    pub need_update_application: bool,
}

/// A SOCKS5 proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeSocks5 {
    /// Username for logging in; may be empty
    pub username: String,
    /// Password for logging in; may be empty
    pub password: String,
}

/// A HTTP transparent proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeHttp {
    /// Username for logging in; may be empty
    pub username: String,
    /// Password for logging in; may be empty
    pub password: String,
    /// Pass true if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method
    pub http_only: bool,
}

/// An MTProto proxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProxyTypeMtproto {
    /// The proxy's secret in hexadecimal encoding
    pub secret: String,
}

/// Contains information about a proxy server added to the list of proxies
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedProxy {
    /// Unique identifier of the proxy
    pub id: i32,
    /// Point in time (Unix timestamp) when the proxy was last used; 0 if never
    pub last_used_date: i32,
    /// True, if the proxy is enabled now
    pub is_enabled: bool,
    /// Comment for the proxy added by the user
    pub comment: String,
    /// The proxy
    pub proxy: crate::types::Proxy,
}

/// Represents a list of added proxy servers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddedProxies {
    /// List of proxy servers
    pub proxies: Vec<crate::types::AddedProxy>,
}

/// Represents a date range
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DateRange {
    /// Point in time (Unix timestamp) at which the date range begins
    pub start_date: i32,
    /// Point in time (Unix timestamp) at which the date range ends
    pub end_date: i32,
}

/// A value with information about its recent changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalValue {
    /// The current value
    pub value: f64,
    /// The value for the previous day
    pub previous_value: f64,
    /// The growth rate of the value, as a percentage
    pub growth_rate_percentage: f64,
}

/// A graph data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphData {
    /// Graph data in JSON format
    pub json_data: String,
    /// If non-empty, a token which can be used to receive a zoomed in graph
    pub zoom_token: String,
}

/// The graph data to be asynchronously loaded through getStatisticalGraph
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphAsync {
    /// The token to use for data loading
    pub token: String,
}

/// An error message to be shown to the user instead of the graph
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StatisticalGraphError {
    /// The error message
    pub error_message: String,
}

/// A point on a Cartesian plane
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Point {
    /// The point's first coordinate
    pub x: f64,
    /// The point's second coordinate
    pub y: f64,
}

/// A straight line to a given point
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VectorPathCommandLine {
    /// The end point of the straight line
    pub end_point: crate::types::Point,
}

/// A cubic Bézier curve to a given point
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VectorPathCommandCubicBezierCurve {
    /// The start control point of the curve
    pub start_control_point: crate::types::Point,
    /// The end control point of the curve
    pub end_control_point: crate::types::Point,
    /// The end point of the curve
    pub end_point: crate::types::Point,
}

/// Checks ownership of a new phone number to change the user's authentication phone number; for official Android and iOS applications only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberCodeTypeChange {
}

/// Verifies ownership of a phone number to be added to the user's Telegram Passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberCodeTypeVerify {
}

/// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberCodeTypeConfirmOwnership {
    /// Hash value from the link
    pub hash: String,
}

/// The user authorization state has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAuthorizationState {
    /// New authorization state
    pub authorization_state: crate::enums::AuthorizationState,
}

/// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcut {
    /// New data about the shortcut
    pub shortcut: crate::types::QuickReplyShortcut,
}

/// A quick reply shortcut and all its messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcutDeleted {
    /// The identifier of the deleted shortcut
    pub shortcut_id: i32,
}

/// The list of quick reply shortcuts has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcuts {
    /// The new list of identifiers of quick reply shortcuts
    pub shortcut_ids: Vec<i32>,
}

/// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateCommunity {
    /// New data about the community
    pub community: crate::types::Community,
}

/// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBasicGroup {
    /// New data about the group
    pub basic_group: crate::types::BasicGroup,
}

/// Some data in basicGroupFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBasicGroupFullInfo {
    /// Identifier of a basic group
    pub basic_group_id: i64,
    /// New full information about the group
    pub basic_group_full_info: crate::types::BasicGroupFullInfo,
}

/// Some data in communityFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateCommunityFullInfo {
    /// Identifier of the community
    pub community_id: i64,
    /// New full information about the community
    pub community_full_info: crate::types::CommunityFullInfo,
}

/// An OAuth authorization request was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewOauthRequest {
    /// A domain of the URL where the user authorizes
    pub domain: String,
    /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
    pub location: String,
    /// The URL to pass to getOauthLinkInfo; the link is valid for 60 seconds
    pub url: String,
}

/// A request can't be completed unless application verification is performed; for official mobile applications only.
/// The method setApplicationVerificationToken must be called once the verification is completed or failed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateApplicationVerificationRequired {
    /// Unique identifier for the verification process
    pub verification_id: i64,
    /// Unique base64url-encoded nonce for the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic) for Android,
    /// or a unique string to compare with verify_nonce field from a push notification for iOS
    pub nonce: String,
    /// Cloud project number to pass to the Play Integrity API on Android
    #[serde_as(as = "DisplayFromStr")]
    pub cloud_project_number: i64,
}

/// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only.
/// The method setApplicationVerificationToken must be called once the verification is completed or failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateApplicationRecaptchaVerificationRequired {
    /// Unique identifier for the verification process
    pub verification_id: i64,
    /// The action for the check
    pub action: String,
    /// Identifier of the reCAPTCHA key
    pub recaptcha_key_id: String,
}

/// State of a gift auction was updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateGiftAuctionState {
    /// New state of the auction
    pub state: crate::types::GiftAuctionState,
}

/// The list of auctions in which the current user participates has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveGiftAuctions {
    /// New states of the auctions
    pub states: Vec<crate::types::GiftAuctionState>,
}

/// An option changed its value
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateOption {
    /// The option name
    pub name: String,
    /// The new option value
    pub value: crate::enums::OptionValue,
}

/// The default background has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultBackground {
    /// True, if default background for dark theme has changed
    pub for_dark_theme: bool,
    /// The new default background; may be null
    pub background: Option<crate::types::Background>,
}

/// The list of supported accent colors has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAccentColors {
    /// Information about supported colors; colors with identifiers 0 (red), 1 (orange), 2 (purple/violet), 3 (green), 4 (cyan), 5 (blue), 6 (pink) must always be supported
    /// and aren't included in the list. The exact colors for the accent colors with identifiers 0-6 must be taken from the application theme
    pub colors: Vec<crate::types::AccentColor>,
    /// The list of accent color identifiers, which can be set through setAccentColor and setChatAccentColor. The colors must be shown in the specified order
    pub available_accent_color_ids: Vec<i32>,
}

/// Web browser settings have been updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateWebBrowserSettings {
    /// New settings
    pub settings: crate::types::WebBrowserSettings,
}

/// Some language pack strings have been updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateLanguagePackStrings {
    /// Localization target to which the language pack belongs
    pub localization_target: String,
    /// Identifier of the updated language pack
    pub language_pack_id: String,
    /// List of changed language pack strings; empty if all strings have changed
    pub strings: Vec<crate::types::LanguagePackString>,
}

/// The connection state has changed. This update must be used only to show a human-readable description of the connection state
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateConnectionState {
    /// The new connection state
    pub state: crate::enums::ConnectionState,
}

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

/// The parameters for age verification of the current user's account have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAgeVerificationParameters {
    /// Parameters for the age verification; may be null if age verification isn't needed
    pub parameters: Option<crate::types::AgeVerificationParameters>,
}

/// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTermsOfService {
    /// Identifier of the terms of service
    pub terms_of_service_id: String,
    /// The new terms of service
    pub terms_of_service: crate::types::TermsOfService,
}

/// The first unconfirmed session has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnconfirmedSession {
    /// The unconfirmed session; may be null if none
    pub session: Option<crate::types::UnconfirmedSession>,
    /// The total number of unconfirmed sessions
    pub unconfirmed_session_count: i32,
}

/// The number of Telegram Stars owned by the current user has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateOwnedStarCount {
    /// The new amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
}

/// The number of TON Grams owned by the current user has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateOwnedGramCount {
    /// The new amount of owned Grams; in the smallest units of the cryptocurrency
    pub gram_amount: i64,
}

/// The parameters of speech recognition without Telegram Premium subscription have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSpeechRecognitionTrial {
    /// The maximum allowed duration of media for speech recognition without Telegram Premium subscription, in seconds
    pub max_media_duration: i32,
    /// The total number of allowed speech recognitions per week; 0 if none
    pub weekly_count: i32,
    /// Number of left speech recognition attempts this week
    pub left_count: i32,
    /// Point in time (Unix timestamp) when the weekly number of tries will reset; 0 if unknown
    pub next_reset_date: i32,
}

/// The list of suggested to the user actions has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSuggestedActions {
    /// Added suggested actions
    pub added_actions: Vec<crate::enums::SuggestedAction>,
    /// Removed suggested actions
    pub removed_actions: Vec<crate::enums::SuggestedAction>,
}

/// Autosave settings for some type of chats were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAutosaveSettings {
    /// Type of chats for which autosave settings were updated
    pub scope: crate::enums::AutosaveSettingsScope,
    /// The new autosave settings; may be null if the settings are reset to default
    pub settings: Option<crate::types::ScopeAutosaveSettings>,
}

/// A new incoming inline query; for bots only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewInlineQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// User location; may be null
    pub user_location: Option<crate::types::Location>,
    /// The type of the chat from which the query originated; may be null if unknown
    pub chat_type: Option<crate::enums::ChatType>,
    /// Text of the query
    pub query: String,
    /// Offset of the first entry to return
    pub offset: String,
}

/// The user has chosen a result of an inline query; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChosenInlineResult {
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// User location; may be null
    pub user_location: Option<crate::types::Location>,
    /// Text of the query
    pub query: String,
    /// Identifier of the chosen result
    pub result_id: String,
    /// Identifier of the sent inline message, if known
    pub inline_message_id: String,
}

/// A new incoming guest query; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewGuestQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// The message with the query
    pub message: crate::types::Message,
    /// The list of reference messages
    pub reference_messages: Vec<crate::types::Message>,
}

/// A new incoming event; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCustomEvent {
    /// A JSON-serialized event
    pub event: String,
}

/// A new incoming query; for bots only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCustomQuery {
    /// The query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// JSON-serialized query data
    pub data: String,
    /// Query timeout
    pub timeout: i32,
}

/// Paid media were purchased by a user; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdatePaidMediaPurchased {
    /// User identifier
    pub user_id: i64,
    /// Bot-specified payload for the paid media
    pub payload: String,
}

/// Contains a list of updates
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Updates {
    /// List of updates
    pub updates: Vec<crate::enums::Update>,
}

/// The log is written to stderr or an OS specific log
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogStreamDefault {
}

/// The log is written nowhere
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogStreamEmpty {
}

/// Contains a TDLib internal log verbosity level
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogVerbosityLevel {
    /// Log verbosity level
    pub verbosity_level: i32,
}

/// Contains a list of available TDLib internal log tags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogTags {
    /// List of log tags
    pub tags: Vec<String>,
}

/// A simple object containing a number; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestInt {
    /// Number
    pub value: i32,
}

/// A simple object containing a string; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestString {
    /// String
    pub value: String,
}

/// A simple object containing a sequence of bytes; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestBytes {
    /// Bytes
    pub value: String,
}

/// A simple object containing a vector of numbers; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorInt {
    /// Vector of numbers
    pub value: Vec<i32>,
}

/// A simple object containing a vector of objects that hold a number; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorIntObject {
    /// Vector of objects
    pub value: Vec<crate::types::TestInt>,
}

/// A simple object containing a vector of strings; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorString {
    /// Vector of strings
    pub value: Vec<String>,
}

/// A simple object containing a vector of objects that hold a string; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorStringObject {
    /// Vector of objects
    pub value: Vec<crate::types::TestString>,
}

