pub mod init;
pub mod pane;
pub mod sidebar;
pub mod window;
pub mod winctl;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");



pub use init::Mastary;
