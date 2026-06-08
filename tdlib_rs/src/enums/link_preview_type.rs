#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewType {
    /// The link is a link to a media album consisting of photos and videos
    #[serde(rename(
        serialize = "linkPreviewTypeAlbum",
        deserialize = "linkPreviewTypeAlbum"
    ))]
    Album(crate::types::LinkPreviewTypeAlbum),
    /// The link is a link to an animation
    #[serde(rename(
        serialize = "linkPreviewTypeAnimation",
        deserialize = "linkPreviewTypeAnimation"
    ))]
    Animation(crate::types::LinkPreviewTypeAnimation),
    /// The link is a link to an app at App Store or Google Play
    #[serde(rename(serialize = "linkPreviewTypeApp", deserialize = "linkPreviewTypeApp"))]
    App(crate::types::LinkPreviewTypeApp),
    /// The link is a link to a web site
    #[serde(rename(
        serialize = "linkPreviewTypeArticle",
        deserialize = "linkPreviewTypeArticle"
    ))]
    Article(crate::types::LinkPreviewTypeArticle),
    /// The link is a link to an audio
    #[serde(rename(
        serialize = "linkPreviewTypeAudio",
        deserialize = "linkPreviewTypeAudio"
    ))]
    Audio(crate::types::LinkPreviewTypeAudio),
    /// The link is a link to a background. Link preview title and description are available only for filled backgrounds
    #[serde(rename(
        serialize = "linkPreviewTypeBackground",
        deserialize = "linkPreviewTypeBackground"
    ))]
    Background(crate::types::LinkPreviewTypeBackground),
    /// The link is a link to boost a channel chat
    #[serde(rename(
        serialize = "linkPreviewTypeChannelBoost",
        deserialize = "linkPreviewTypeChannelBoost"
    ))]
    ChannelBoost(crate::types::LinkPreviewTypeChannelBoost),
    /// The link is a link to a chat
    #[serde(rename(serialize = "linkPreviewTypeChat", deserialize = "linkPreviewTypeChat"))]
    Chat(crate::types::LinkPreviewTypeChat),
    /// The link is a link to a direct messages chat of a channel
    #[serde(rename(
        serialize = "linkPreviewTypeDirectMessagesChat",
        deserialize = "linkPreviewTypeDirectMessagesChat"
    ))]
    DirectMessagesChat(crate::types::LinkPreviewTypeDirectMessagesChat),
    /// The link is a link to a general file
    #[serde(rename(
        serialize = "linkPreviewTypeDocument",
        deserialize = "linkPreviewTypeDocument"
    ))]
    Document(crate::types::LinkPreviewTypeDocument),
    /// The link is a link to an animation player
    #[serde(rename(
        serialize = "linkPreviewTypeEmbeddedAnimationPlayer",
        deserialize = "linkPreviewTypeEmbeddedAnimationPlayer"
    ))]
    EmbeddedAnimationPlayer(crate::types::LinkPreviewTypeEmbeddedAnimationPlayer),
    /// The link is a link to an audio player
    #[serde(rename(
        serialize = "linkPreviewTypeEmbeddedAudioPlayer",
        deserialize = "linkPreviewTypeEmbeddedAudioPlayer"
    ))]
    EmbeddedAudioPlayer(crate::types::LinkPreviewTypeEmbeddedAudioPlayer),
    /// The link is a link to a video player
    #[serde(rename(
        serialize = "linkPreviewTypeEmbeddedVideoPlayer",
        deserialize = "linkPreviewTypeEmbeddedVideoPlayer"
    ))]
    EmbeddedVideoPlayer(crate::types::LinkPreviewTypeEmbeddedVideoPlayer),
    /// The link is a link to an audio file
    #[serde(rename(
        serialize = "linkPreviewTypeExternalAudio",
        deserialize = "linkPreviewTypeExternalAudio"
    ))]
    ExternalAudio(crate::types::LinkPreviewTypeExternalAudio),
    /// The link is a link to a video file
    #[serde(rename(
        serialize = "linkPreviewTypeExternalVideo",
        deserialize = "linkPreviewTypeExternalVideo"
    ))]
    ExternalVideo(crate::types::LinkPreviewTypeExternalVideo),
    /// The link is a link to a gift auction
    #[serde(rename(
        serialize = "linkPreviewTypeGiftAuction",
        deserialize = "linkPreviewTypeGiftAuction"
    ))]
    GiftAuction(crate::types::LinkPreviewTypeGiftAuction),
    /// The link is a link to a gift collection
    #[serde(rename(
        serialize = "linkPreviewTypeGiftCollection",
        deserialize = "linkPreviewTypeGiftCollection"
    ))]
    GiftCollection(crate::types::LinkPreviewTypeGiftCollection),
    /// The link is a link to a group call that isn't bound to a chat
    #[serde(rename(
        serialize = "linkPreviewTypeGroupCall",
        deserialize = "linkPreviewTypeGroupCall"
    ))]
    GroupCall,
    /// The link is a link to an invoice
    #[serde(rename(
        serialize = "linkPreviewTypeInvoice",
        deserialize = "linkPreviewTypeInvoice"
    ))]
    Invoice,
    /// The link is a link to a live story group call
    #[serde(rename(
        serialize = "linkPreviewTypeLiveStory",
        deserialize = "linkPreviewTypeLiveStory"
    ))]
    LiveStory(crate::types::LinkPreviewTypeLiveStory),
    /// The link is a link to a text or a poll Telegram message
    #[serde(rename(
        serialize = "linkPreviewTypeMessage",
        deserialize = "linkPreviewTypeMessage"
    ))]
    Message,
    /// The link is a link to a photo
    #[serde(rename(
        serialize = "linkPreviewTypePhoto",
        deserialize = "linkPreviewTypePhoto"
    ))]
    Photo(crate::types::LinkPreviewTypePhoto),
    /// The link is a link to a Telegram Premium gift code
    #[serde(rename(
        serialize = "linkPreviewTypePremiumGiftCode",
        deserialize = "linkPreviewTypePremiumGiftCode"
    ))]
    PremiumGiftCode,
    /// The link is a link to a shareable chat folder
    #[serde(rename(
        serialize = "linkPreviewTypeShareableChatFolder",
        deserialize = "linkPreviewTypeShareableChatFolder"
    ))]
    ShareableChatFolder,
    /// The link is a link to a sticker
    #[serde(rename(
        serialize = "linkPreviewTypeSticker",
        deserialize = "linkPreviewTypeSticker"
    ))]
    Sticker(crate::types::LinkPreviewTypeSticker),
    /// The link is a link to a sticker set
    #[serde(rename(
        serialize = "linkPreviewTypeStickerSet",
        deserialize = "linkPreviewTypeStickerSet"
    ))]
    StickerSet(crate::types::LinkPreviewTypeStickerSet),
    /// The link is a link to a story. Link preview description is unavailable
    #[serde(rename(
        serialize = "linkPreviewTypeStory",
        deserialize = "linkPreviewTypeStory"
    ))]
    Story(crate::types::LinkPreviewTypeStory),
    /// The link is a link to an album of stories
    #[serde(rename(
        serialize = "linkPreviewTypeStoryAlbum",
        deserialize = "linkPreviewTypeStoryAlbum"
    ))]
    StoryAlbum(crate::types::LinkPreviewTypeStoryAlbum),
    /// The link is a link to boost a supergroup chat
    #[serde(rename(
        serialize = "linkPreviewTypeSupergroupBoost",
        deserialize = "linkPreviewTypeSupergroupBoost"
    ))]
    SupergroupBoost(crate::types::LinkPreviewTypeSupergroupBoost),
    /// The link is a link to a cloud theme. TDLib has no theme support yet
    #[serde(rename(
        serialize = "linkPreviewTypeTheme",
        deserialize = "linkPreviewTypeTheme"
    ))]
    Theme(crate::types::LinkPreviewTypeTheme),
    /// The link preview type is unsupported yet
    #[serde(rename(
        serialize = "linkPreviewTypeUnsupported",
        deserialize = "linkPreviewTypeUnsupported"
    ))]
    Unsupported,
    /// The link is a link to an upgraded gift
    #[serde(rename(
        serialize = "linkPreviewTypeUpgradedGift",
        deserialize = "linkPreviewTypeUpgradedGift"
    ))]
    UpgradedGift(crate::types::LinkPreviewTypeUpgradedGift),
    /// The link is a link to a user
    #[serde(rename(serialize = "linkPreviewTypeUser", deserialize = "linkPreviewTypeUser"))]
    User(crate::types::LinkPreviewTypeUser),
    /// The link is a link to a video
    #[serde(rename(
        serialize = "linkPreviewTypeVideo",
        deserialize = "linkPreviewTypeVideo"
    ))]
    Video(crate::types::LinkPreviewTypeVideo),
    /// The link is a link to a video chat
    #[serde(rename(
        serialize = "linkPreviewTypeVideoChat",
        deserialize = "linkPreviewTypeVideoChat"
    ))]
    VideoChat(crate::types::LinkPreviewTypeVideoChat),
    /// The link is a link to a video note message
    #[serde(rename(
        serialize = "linkPreviewTypeVideoNote",
        deserialize = "linkPreviewTypeVideoNote"
    ))]
    VideoNote(crate::types::LinkPreviewTypeVideoNote),
    /// The link is a link to a voice note message
    #[serde(rename(
        serialize = "linkPreviewTypeVoiceNote",
        deserialize = "linkPreviewTypeVoiceNote"
    ))]
    VoiceNote(crate::types::LinkPreviewTypeVoiceNote),
    /// The link is a link to a Web App
    #[serde(rename(
        serialize = "linkPreviewTypeWebApp",
        deserialize = "linkPreviewTypeWebApp"
    ))]
    WebApp(crate::types::LinkPreviewTypeWebApp),
}
