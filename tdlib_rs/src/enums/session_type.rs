#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SessionType {
    /// The session is running on an Android device
    #[serde(rename(serialize = "sessionTypeAndroid", deserialize = "sessionTypeAndroid"))]
    Android,
    /// The session is running on a generic Apple device
    #[serde(rename(serialize = "sessionTypeApple", deserialize = "sessionTypeApple"))]
    Apple,
    /// The session is running on the Brave browser
    #[serde(rename(serialize = "sessionTypeBrave", deserialize = "sessionTypeBrave"))]
    Brave,
    /// The session is running on the Chrome browser
    #[serde(rename(serialize = "sessionTypeChrome", deserialize = "sessionTypeChrome"))]
    Chrome,
    /// The session is running on the Edge browser
    #[serde(rename(serialize = "sessionTypeEdge", deserialize = "sessionTypeEdge"))]
    Edge,
    /// The session is running on the Firefox browser
    #[serde(rename(serialize = "sessionTypeFirefox", deserialize = "sessionTypeFirefox"))]
    Firefox,
    /// The session is running on an iPad device
    #[serde(rename(serialize = "sessionTypeIpad", deserialize = "sessionTypeIpad"))]
    Ipad,
    /// The session is running on an iPhone device
    #[serde(rename(serialize = "sessionTypeIphone", deserialize = "sessionTypeIphone"))]
    Iphone,
    /// The session is running on a Linux device
    #[serde(rename(serialize = "sessionTypeLinux", deserialize = "sessionTypeLinux"))]
    Linux,
    /// The session is running on a Mac device
    #[serde(rename(serialize = "sessionTypeMac", deserialize = "sessionTypeMac"))]
    Mac,
    /// The session is running on the Opera browser
    #[serde(rename(serialize = "sessionTypeOpera", deserialize = "sessionTypeOpera"))]
    Opera,
    /// The session is running on the Safari browser
    #[serde(rename(serialize = "sessionTypeSafari", deserialize = "sessionTypeSafari"))]
    Safari,
    /// The session is running on an Ubuntu device
    #[serde(rename(serialize = "sessionTypeUbuntu", deserialize = "sessionTypeUbuntu"))]
    Ubuntu,
    /// The session is running on an unknown type of device
    #[serde(rename(serialize = "sessionTypeUnknown", deserialize = "sessionTypeUnknown"))]
    Unknown,
    /// The session is running on the Vivaldi browser
    #[serde(rename(serialize = "sessionTypeVivaldi", deserialize = "sessionTypeVivaldi"))]
    Vivaldi,
    /// The session is running on a Windows device
    #[serde(rename(serialize = "sessionTypeWindows", deserialize = "sessionTypeWindows"))]
    Windows,
    /// The session is running on an Xbox console
    #[serde(rename(serialize = "sessionTypeXbox", deserialize = "sessionTypeXbox"))]
    Xbox,
}
