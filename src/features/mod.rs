pub mod install;
pub mod list;
pub mod outdated;
pub mod uninstall;
pub mod update;

pub type Result<T> = std::result::Result<T, String>;
