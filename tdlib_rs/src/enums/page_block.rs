#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlock {
    /// The title of a page
    #[serde(rename(serialize = "pageBlockTitle", deserialize = "pageBlockTitle"))]
    Title(crate::types::PageBlockTitle),
    /// The subtitle of a page
    #[serde(rename(serialize = "pageBlockSubtitle", deserialize = "pageBlockSubtitle"))]
    Subtitle(crate::types::PageBlockSubtitle),
    /// The author and publishing date of a page
    #[serde(rename(serialize = "pageBlockAuthorDate", deserialize = "pageBlockAuthorDate"))]
    AuthorDate(crate::types::PageBlockAuthorDate),
    /// A header
    #[serde(rename(serialize = "pageBlockHeader", deserialize = "pageBlockHeader"))]
    Header(crate::types::PageBlockHeader),
    /// A subheader
    #[serde(rename(serialize = "pageBlockSubheader", deserialize = "pageBlockSubheader"))]
    Subheader(crate::types::PageBlockSubheader),
    /// A kicker
    #[serde(rename(serialize = "pageBlockKicker", deserialize = "pageBlockKicker"))]
    Kicker(crate::types::PageBlockKicker),
    /// A text paragraph
    #[serde(rename(serialize = "pageBlockParagraph", deserialize = "pageBlockParagraph"))]
    Paragraph(crate::types::PageBlockParagraph),
    /// A preformatted text paragraph
    #[serde(rename(
        serialize = "pageBlockPreformatted",
        deserialize = "pageBlockPreformatted"
    ))]
    Preformatted(crate::types::PageBlockPreformatted),
    /// The footer of a page
    #[serde(rename(serialize = "pageBlockFooter", deserialize = "pageBlockFooter"))]
    Footer(crate::types::PageBlockFooter),
    /// An empty block separating a page
    #[serde(rename(serialize = "pageBlockDivider", deserialize = "pageBlockDivider"))]
    Divider,
    /// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
    #[serde(rename(serialize = "pageBlockAnchor", deserialize = "pageBlockAnchor"))]
    Anchor(crate::types::PageBlockAnchor),
    /// A list of data blocks
    #[serde(rename(serialize = "pageBlockList", deserialize = "pageBlockList"))]
    List(crate::types::PageBlockList),
    /// A block quote
    #[serde(rename(serialize = "pageBlockBlockQuote", deserialize = "pageBlockBlockQuote"))]
    BlockQuote(crate::types::PageBlockBlockQuote),
    /// A pull quote
    #[serde(rename(serialize = "pageBlockPullQuote", deserialize = "pageBlockPullQuote"))]
    PullQuote(crate::types::PageBlockPullQuote),
    /// An animation
    #[serde(rename(serialize = "pageBlockAnimation", deserialize = "pageBlockAnimation"))]
    Animation(crate::types::PageBlockAnimation),
    /// An audio file
    #[serde(rename(serialize = "pageBlockAudio", deserialize = "pageBlockAudio"))]
    Audio(crate::types::PageBlockAudio),
    /// A photo
    #[serde(rename(serialize = "pageBlockPhoto", deserialize = "pageBlockPhoto"))]
    Photo(crate::types::PageBlockPhoto),
    /// A video
    #[serde(rename(serialize = "pageBlockVideo", deserialize = "pageBlockVideo"))]
    Video(crate::types::PageBlockVideo),
    /// A voice note
    #[serde(rename(serialize = "pageBlockVoiceNote", deserialize = "pageBlockVoiceNote"))]
    VoiceNote(crate::types::PageBlockVoiceNote),
    /// A page cover
    #[serde(rename(serialize = "pageBlockCover", deserialize = "pageBlockCover"))]
    Cover(Box<crate::types::PageBlockCover>),
    /// An embedded web page
    #[serde(rename(serialize = "pageBlockEmbedded", deserialize = "pageBlockEmbedded"))]
    Embedded(crate::types::PageBlockEmbedded),
    /// An embedded post
    #[serde(rename(
        serialize = "pageBlockEmbeddedPost",
        deserialize = "pageBlockEmbeddedPost"
    ))]
    EmbeddedPost(crate::types::PageBlockEmbeddedPost),
    /// A collage
    #[serde(rename(serialize = "pageBlockCollage", deserialize = "pageBlockCollage"))]
    Collage(crate::types::PageBlockCollage),
    /// A slideshow
    #[serde(rename(serialize = "pageBlockSlideshow", deserialize = "pageBlockSlideshow"))]
    Slideshow(crate::types::PageBlockSlideshow),
    /// A link to a chat
    #[serde(rename(serialize = "pageBlockChatLink", deserialize = "pageBlockChatLink"))]
    ChatLink(crate::types::PageBlockChatLink),
    /// A table
    #[serde(rename(serialize = "pageBlockTable", deserialize = "pageBlockTable"))]
    Table(crate::types::PageBlockTable),
    /// A collapsible block
    #[serde(rename(serialize = "pageBlockDetails", deserialize = "pageBlockDetails"))]
    Details(crate::types::PageBlockDetails),
    /// Related articles
    #[serde(rename(
        serialize = "pageBlockRelatedArticles",
        deserialize = "pageBlockRelatedArticles"
    ))]
    RelatedArticles(crate::types::PageBlockRelatedArticles),
    /// A map
    #[serde(rename(serialize = "pageBlockMap", deserialize = "pageBlockMap"))]
    Map(crate::types::PageBlockMap),
}
