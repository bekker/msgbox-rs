extern crate gtk;
use self::gtk::prelude::*;
use self::gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog};

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
    dialog.set_modal(true);
    dialog.set_decorated(true);
    dialog.set_keep_above(true);
    dialog.show();
    dialog.connect_response(move |ref dialog, response_id| {
        dialog.destroy();
        gtk::main_quit();
        Inhibit(false);
    });
    gtk::main();
}