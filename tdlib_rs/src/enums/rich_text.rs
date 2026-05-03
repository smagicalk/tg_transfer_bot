#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RichText {
    /// A plain text
    #[serde(rename(serialize = "richTextPlain", deserialize = "richTextPlain"))]
    Plain(crate::types::RichTextPlain),
    /// A bold rich text
    #[serde(rename(serialize = "richTextBold", deserialize = "richTextBold"))]
    Bold(Box<crate::types::RichTextBold>),
    /// An italicized rich text
    #[serde(rename(serialize = "richTextItalic", deserialize = "richTextItalic"))]
    Italic(Box<crate::types::RichTextItalic>),
    /// An underlined rich text
    #[serde(rename(serialize = "richTextUnderline", deserialize = "richTextUnderline"))]
    Underline(Box<crate::types::RichTextUnderline>),
    /// A strikethrough rich text
    #[serde(rename(
        serialize = "richTextStrikethrough",
        deserialize = "richTextStrikethrough"
    ))]
    Strikethrough(Box<crate::types::RichTextStrikethrough>),
    /// A fixed-width rich text
    #[serde(rename(serialize = "richTextFixed", deserialize = "richTextFixed"))]
    Fixed(Box<crate::types::RichTextFixed>),
    /// A rich text URL link
    #[serde(rename(serialize = "richTextUrl", deserialize = "richTextUrl"))]
    Url(Box<crate::types::RichTextUrl>),
    /// A rich text email link
    #[serde(rename(
        serialize = "richTextEmailAddress",
        deserialize = "richTextEmailAddress"
    ))]
    EmailAddress(Box<crate::types::RichTextEmailAddress>),
    /// A subscript rich text
    #[serde(rename(serialize = "richTextSubscript", deserialize = "richTextSubscript"))]
    Subscript(Box<crate::types::RichTextSubscript>),
    /// A superscript rich text
    #[serde(rename(serialize = "richTextSuperscript", deserialize = "richTextSuperscript"))]
    Superscript(Box<crate::types::RichTextSuperscript>),
    /// A marked rich text
    #[serde(rename(serialize = "richTextMarked", deserialize = "richTextMarked"))]
    Marked(Box<crate::types::RichTextMarked>),
    /// A rich text phone number
    #[serde(rename(serialize = "richTextPhoneNumber", deserialize = "richTextPhoneNumber"))]
    PhoneNumber(Box<crate::types::RichTextPhoneNumber>),
    /// A small image inside the text
    #[serde(rename(serialize = "richTextIcon", deserialize = "richTextIcon"))]
    Icon(crate::types::RichTextIcon),
    /// A reference to a richTexts object on the same page
    #[serde(rename(serialize = "richTextReference", deserialize = "richTextReference"))]
    Reference(Box<crate::types::RichTextReference>),
    /// An anchor
    #[serde(rename(serialize = "richTextAnchor", deserialize = "richTextAnchor"))]
    Anchor(crate::types::RichTextAnchor),
    /// A link to an anchor on the same page
    #[serde(rename(serialize = "richTextAnchorLink", deserialize = "richTextAnchorLink"))]
    AnchorLink(Box<crate::types::RichTextAnchorLink>),
    /// A concatenation of rich texts
    #[serde(rename(serialize = "richTexts", deserialize = "richTexts"))]
    RichTexts(crate::types::RichTexts),
}
