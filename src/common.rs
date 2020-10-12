#[cfg(target_family = "unix")]
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
    #[cfg(target_family = "unix")]
    #[error("failed to create a message box")]
    Create(#[from] BoolError),
    #[cfg(not(target_family = "unix"))]
    #[error("failed to create a message box")]
    Create,
}
