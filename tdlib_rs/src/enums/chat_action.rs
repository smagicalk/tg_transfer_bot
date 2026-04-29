#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAction {
    /// The user is typing a message
    #[serde(rename(serialize = "chatActionTyping", deserialize = "chatActionTyping"))]
    Typing,
    /// The user is recording a video
    #[serde(rename(serialize = "chatActionRecordingVideo", deserialize = "chatActionRecordingVideo"))]
    RecordingVideo,
    /// The user is uploading a video
    #[serde(rename(serialize = "chatActionUploadingVideo", deserialize = "chatActionUploadingVideo"))]
    UploadingVideo(crate::types::ChatActionUploadingVideo),
    /// The user is recording a voice note
    #[serde(rename(serialize = "chatActionRecordingVoiceNote", deserialize = "chatActionRecordingVoiceNote"))]
    RecordingVoiceNote,
    /// The user is uploading a voice note
    #[serde(rename(serialize = "chatActionUploadingVoiceNote", deserialize = "chatActionUploadingVoiceNote"))]
    UploadingVoiceNote(crate::types::ChatActionUploadingVoiceNote),
    /// The user is uploading a photo
    #[serde(rename(serialize = "chatActionUploadingPhoto", deserialize = "chatActionUploadingPhoto"))]
    UploadingPhoto(crate::types::ChatActionUploadingPhoto),
    /// The user is uploading a document
    #[serde(rename(serialize = "chatActionUploadingDocument", deserialize = "chatActionUploadingDocument"))]
    UploadingDocument(crate::types::ChatActionUploadingDocument),
    /// The user is picking a sticker to send
    #[serde(rename(serialize = "chatActionChoosingSticker", deserialize = "chatActionChoosingSticker"))]
    ChoosingSticker,
    /// The user is picking a location or venue to send
    #[serde(rename(serialize = "chatActionChoosingLocation", deserialize = "chatActionChoosingLocation"))]
    ChoosingLocation,
    /// The user is picking a contact to send
    #[serde(rename(serialize = "chatActionChoosingContact", deserialize = "chatActionChoosingContact"))]
    ChoosingContact,
    /// The user has started to play a game
    #[serde(rename(serialize = "chatActionStartPlayingGame", deserialize = "chatActionStartPlayingGame"))]
    StartPlayingGame,
    /// The user is recording a video note
    #[serde(rename(serialize = "chatActionRecordingVideoNote", deserialize = "chatActionRecordingVideoNote"))]
    RecordingVideoNote,
    /// The user is uploading a video note
    #[serde(rename(serialize = "chatActionUploadingVideoNote", deserialize = "chatActionUploadingVideoNote"))]
    UploadingVideoNote(crate::types::ChatActionUploadingVideoNote),
    /// The user is watching animations sent by the other party by clicking on an animated emoji
    #[serde(rename(serialize = "chatActionWatchingAnimations", deserialize = "chatActionWatchingAnimations"))]
    WatchingAnimations(crate::types::ChatActionWatchingAnimations),
    /// The user has canceled the previous action
    #[serde(rename(serialize = "chatActionCancel", deserialize = "chatActionCancel"))]
    Cancel,
}
