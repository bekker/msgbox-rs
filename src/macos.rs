#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

use common::{IconType, MsgBoxError};

/**
 * cocoa-rs doesn't implement NSAlert yet (0.14.0)
 * Then implement it!
 * Someone would stub and implement all methods for NSAlert, and make it to the upstream?
 */

/**
 * NSAlert.Style
 * https://developer.apple.com/documentation/appkit/nsalert.style
 */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSAlertStyle {
    warning = 0, // Same visual as informational
    informational = 1,
    critical = 2,
}

#[allow(non_upper_case_globals)]
pub static NSModalPannelWindowLevel: i32 = 10;

/**
 * NSAlert
 * https://developer.apple.com/documentation/appkit/nsalert
 */
pub trait NSAlert: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSAlert), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn autorelease(self) -> id;

    unsafe fn setAlertStyle(self, style: NSAlertStyle);
    unsafe fn setMessageText(self, messageText: id);
    unsafe fn setInformativeText(self, informativeText: id);
    unsafe fn addButton(self, withTitle: id);
    unsafe fn window(self) -> id;
    unsafe fn setWindowLevel(self, level: i32);
    unsafe fn runModal(self) -> id;
}

impl NSAlert for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn autorelease(self) -> id {
        msg_send![self, autorelease]
    }

    unsafe fn setAlertStyle(self, alertStyle: NSAlertStyle) {
        msg_send![self, setAlertStyle: alertStyle]
    }

    unsafe fn setMessageText(self, messageText: id) {
        msg_send![self, setMessageText: messageText]
    }

    unsafe fn setInformativeText(self, informativeText: id) {
        msg_send![self, setInformativeText: informativeText]
    }

    unsafe fn addButton(self, withTitle: id) {
        msg_send![self, addButtonWithTitle: withTitle]
    }

    unsafe fn window(self) -> id {
        msg_send![self, window]
    }

    unsafe fn runModal(self) -> id {
        msg_send![self, runModal]
    }

    unsafe fn setWindowLevel(self, level: i32) {
        msg_send![self.window(), setLevel: level]
    }
}

pub fn create(
    title: &str,
    content: &str,
    icon_type: IconType,
) -> std::result::Result<(), MsgBoxError> {
    let alert_style = match icon_type {
        IconType::Error => NSAlertStyle::critical,
        IconType::Info => NSAlertStyle::informational,

        // AppKit doesn't support NSAlert without any icon
        IconType::None => NSAlertStyle::informational,
    };

    unsafe {
        let alert = NSAlert::alloc(nil).init().autorelease();
        alert.addButton(NSString::alloc(nil).init_str("OK"));
        alert.setMessageText(NSString::alloc(nil).init_str(title));
        alert.setInformativeText(NSString::alloc(nil).init_str(content));
        alert.setAlertStyle(alert_style);
        // Force the alert to appear on top of any other windows
        alert.setWindowLevel(NSModalPannelWindowLevel);
        alert.runModal();

        // TODO: Find a away to detect that NSAlert.runModal() failed?
        Ok(())
    }
}
