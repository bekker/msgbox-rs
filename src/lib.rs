extern crate thiserror;

pub mod common;
pub use common::{IconType, MsgBoxError};

/**
 * GTK+3 (Default)
 */
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
extern crate gtk;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
extern crate glib;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod linux;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub use linux::*;

/**
 * Cocoa
 */
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
extern crate cocoa;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

/**
 * WinAPI
 */
#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
