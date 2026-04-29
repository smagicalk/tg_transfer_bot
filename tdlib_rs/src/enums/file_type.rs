#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FileType {
    /// The data is not a file
    #[serde(rename(serialize = "fileTypeNone", deserialize = "fileTypeNone"))]
    None,
    /// The file is an animation
    #[serde(rename(serialize = "fileTypeAnimation", deserialize = "fileTypeAnimation"))]
    Animation,
    /// The file is an audio file
    #[serde(rename(serialize = "fileTypeAudio", deserialize = "fileTypeAudio"))]
    Audio,
    /// The file is a document
    #[serde(rename(serialize = "fileTypeDocument", deserialize = "fileTypeDocument"))]
    Document,
    /// The file is a notification sound
    #[serde(rename(serialize = "fileTypeNotificationSound", deserialize = "fileTypeNotificationSound"))]
    NotificationSound,
    /// The file is a photo
    #[serde(rename(serialize = "fileTypePhoto", deserialize = "fileTypePhoto"))]
    Photo,
    /// The file is a photo published as a story
    #[serde(rename(serialize = "fileTypePhotoStory", deserialize = "fileTypePhotoStory"))]
    PhotoStory,
    /// The file is a profile photo
    #[serde(rename(serialize = "fileTypeProfilePhoto", deserialize = "fileTypeProfilePhoto"))]
    ProfilePhoto,
    /// The file was sent to a secret chat (the file type is not known to the server)
    #[serde(rename(serialize = "fileTypeSecret", deserialize = "fileTypeSecret"))]
    Secret,
    /// The file is a thumbnail of a file from a secret chat
    #[serde(rename(serialize = "fileTypeSecretThumbnail", deserialize = "fileTypeSecretThumbnail"))]
    SecretThumbnail,
    /// The file is a file from Secure storage used for storing Telegram Passport files
    #[serde(rename(serialize = "fileTypeSecure", deserialize = "fileTypeSecure"))]
    Secure,
    /// The file is a self-destructing photo in a private chat
    #[serde(rename(serialize = "fileTypeSelfDestructingPhoto", deserialize = "fileTypeSelfDestructingPhoto"))]
    SelfDestructingPhoto,
    /// The file is a self-destructing video in a private chat
    #[serde(rename(serialize = "fileTypeSelfDestructingVideo", deserialize = "fileTypeSelfDestructingVideo"))]
    SelfDestructingVideo,
    /// The file is a self-destructing video note in a private chat
    #[serde(rename(serialize = "fileTypeSelfDestructingVideoNote", deserialize = "fileTypeSelfDestructingVideoNote"))]
    SelfDestructingVideoNote,
    /// The file is a self-destructing voice note in a private chat
    #[serde(rename(serialize = "fileTypeSelfDestructingVoiceNote", deserialize = "fileTypeSelfDestructingVoiceNote"))]
    SelfDestructingVoiceNote,
    /// The file is a sticker
    #[serde(rename(serialize = "fileTypeSticker", deserialize = "fileTypeSticker"))]
    Sticker,
    /// The file is a thumbnail of another file
    #[serde(rename(serialize = "fileTypeThumbnail", deserialize = "fileTypeThumbnail"))]
    Thumbnail,
    /// The file type is not yet known
    #[serde(rename(serialize = "fileTypeUnknown", deserialize = "fileTypeUnknown"))]
    Unknown,
    /// The file is a video
    #[serde(rename(serialize = "fileTypeVideo", deserialize = "fileTypeVideo"))]
    Video,
    /// The file is a video note
    #[serde(rename(serialize = "fileTypeVideoNote", deserialize = "fileTypeVideoNote"))]
    VideoNote,
    /// The file is a video published as a story
    #[serde(rename(serialize = "fileTypeVideoStory", deserialize = "fileTypeVideoStory"))]
    VideoStory,
    /// The file is a voice note
    #[serde(rename(serialize = "fileTypeVoiceNote", deserialize = "fileTypeVoiceNote"))]
    VoiceNote,
    /// The file is a wallpaper or a background pattern
    #[serde(rename(serialize = "fileTypeWallpaper", deserialize = "fileTypeWallpaper"))]
    Wallpaper,
}
