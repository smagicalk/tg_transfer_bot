#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DeviceToken {
    /// A token for Firebase Cloud Messaging
    #[serde(rename(serialize = "deviceTokenFirebaseCloudMessaging", deserialize = "deviceTokenFirebaseCloudMessaging"))]
    FirebaseCloudMessaging(crate::types::DeviceTokenFirebaseCloudMessaging),
    /// A token for Apple Push Notification service
    #[serde(rename(serialize = "deviceTokenApplePush", deserialize = "deviceTokenApplePush"))]
    ApplePush(crate::types::DeviceTokenApplePush),
    /// A token for Apple Push Notification service VoIP notifications
    #[serde(rename(serialize = "deviceTokenApplePushVoIP", deserialize = "deviceTokenApplePushVoIP"))]
    ApplePushVoIp(crate::types::DeviceTokenApplePushVoIp),
    /// A token for Windows Push Notification Services
    #[serde(rename(serialize = "deviceTokenWindowsPush", deserialize = "deviceTokenWindowsPush"))]
    WindowsPush(crate::types::DeviceTokenWindowsPush),
    /// A token for Microsoft Push Notification Service
    #[serde(rename(serialize = "deviceTokenMicrosoftPush", deserialize = "deviceTokenMicrosoftPush"))]
    MicrosoftPush(crate::types::DeviceTokenMicrosoftPush),
    /// A token for Microsoft Push Notification Service VoIP channel
    #[serde(rename(serialize = "deviceTokenMicrosoftPushVoIP", deserialize = "deviceTokenMicrosoftPushVoIP"))]
    MicrosoftPushVoIp(crate::types::DeviceTokenMicrosoftPushVoIp),
    /// A token for web Push API
    #[serde(rename(serialize = "deviceTokenWebPush", deserialize = "deviceTokenWebPush"))]
    WebPush(crate::types::DeviceTokenWebPush),
    /// A token for Simple Push API for Firefox OS
    #[serde(rename(serialize = "deviceTokenSimplePush", deserialize = "deviceTokenSimplePush"))]
    SimplePush(crate::types::DeviceTokenSimplePush),
    /// A token for Ubuntu Push Client service
    #[serde(rename(serialize = "deviceTokenUbuntuPush", deserialize = "deviceTokenUbuntuPush"))]
    UbuntuPush(crate::types::DeviceTokenUbuntuPush),
    /// A token for BlackBerry Push Service
    #[serde(rename(serialize = "deviceTokenBlackBerryPush", deserialize = "deviceTokenBlackBerryPush"))]
    BlackBerryPush(crate::types::DeviceTokenBlackBerryPush),
    /// A token for Tizen Push Service
    #[serde(rename(serialize = "deviceTokenTizenPush", deserialize = "deviceTokenTizenPush"))]
    TizenPush(crate::types::DeviceTokenTizenPush),
    /// A token for HUAWEI Push Service
    #[serde(rename(serialize = "deviceTokenHuaweiPush", deserialize = "deviceTokenHuaweiPush"))]
    HuaweiPush(crate::types::DeviceTokenHuaweiPush),
}
