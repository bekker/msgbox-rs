#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
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
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    #[error("failed to create a message box")]
    Create(#[from] BoolError),
    #[cfg(not(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    #[error("failed to create a message box")]
    Create(()),
}
