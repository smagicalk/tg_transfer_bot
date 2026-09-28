//!
//! TDLib data structs, categorized by business domain.
//!
pub mod bot;
pub use bot::*;

pub mod call;
pub use call::*;

pub mod chat;
pub use chat::*;

pub mod file;
pub use file::*;

pub mod forum;
pub use forum::*;

pub mod message;
pub use message::*;

pub mod misc;
pub use misc::*;

pub mod notification;
pub use notification::*;

pub mod passport;
pub use passport::*;

pub mod payment;
pub use payment::*;

pub mod poll;
pub use poll::*;

pub mod premium;
pub use premium::*;

pub mod sticker;
pub use sticker::*;

pub mod story;
pub use story::*;

pub mod user;
pub use user::*;

