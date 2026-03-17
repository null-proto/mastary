pub mod init;
pub mod pane;
pub mod sidebar;
pub mod window;
pub mod theme;
pub mod icon;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// VictorMono Nerd Font Mono,VictorMono NFM:style=Bold
pub const ICON_FONT: iced::Font = iced::font::Font::with_name("VictorMono Nerd Font Mono");



pub use theme::Theme;
pub use init::Mastary;
