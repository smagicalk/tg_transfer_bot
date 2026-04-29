#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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
    /// Session type based on the system and application version, which can be used to display a corresponding icon
    pub r#type: crate::enums::SessionType,
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
