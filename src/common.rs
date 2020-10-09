use std::{fmt, error};

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
    #[error("failed to create a message box")]
    Create{#[source] source: Option<Box<dyn error::Error>>},
}
