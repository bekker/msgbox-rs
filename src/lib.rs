mod icon;
pub use icon::IconType;

/**
 * GTK+3
 */
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
extern crate gtk;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
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
