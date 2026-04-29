#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateHavePendingNotifications {
    /// True, if there are some delayed notification updates, which will be sent soon
    pub have_delayed_notifications: bool,
    /// True, if there can be some yet unreceived notifications, which are being fetched from the server
    pub have_unreceived_notifications: bool,
}
