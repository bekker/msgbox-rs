extern crate gtk;
use self::gtk::prelude::*;
use self::gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

use icon::IconType;

pub fn create(title:&str, content:&str, icon_type:IconType) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        println!("{}", content);
        return;
    }

    let message_type = match icon_type {
        IconType::ERROR => MessageType::Error,
        IconType::INFO => MessageType::Info,
        IconType::NONE => MessageType::Other,
    };

    let dialog = MessageDialog::new(
            None::<&gtk::Window>,
            DialogFlags::empty(),
            message_type,
            ButtonsType::Ok,
            content);
    dialog.set_title(title);
    dialog.run();
    dialog.destroy();
}