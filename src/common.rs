#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use glib::error::BoolError;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum IconType {
    Error,
    Info,
    None,
}

impl fmt::Display for IconType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MsgBoxError {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    #[error("failed to create a message box")]
    Create(#[from] BoolError),
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    #[error("failed to create a message box")]
    Create(()),
}
