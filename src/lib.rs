pub mod ui;
pub mod init;

use crate::ui::UpdateMessage;
use iced::Element;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ElementS = Element<'static , UpdateMessage>;

pub type ElementL<'a> = Element<'a , UpdateMessage>;
