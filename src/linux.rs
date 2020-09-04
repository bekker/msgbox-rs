use ::gtk;
use ::gtk::prelude::*;
use ::gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog};

use icon::IconType;

pub fn create(title:&str, content:&str, icon_type:IconType) {
    if gtk::init().is_err() {
        // TODO: return a result with some error type.
        return;
    }

    let message_type = match icon_type {
        IconType::Error => MessageType::Error,
        IconType::Info => MessageType::Info,
        IconType::None => MessageType::Other,
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
    dialog.connect_response(move |ref dialog, _| {
        dialog.destroy();
        gtk::main_quit();
        Inhibit(false);
    });
    gtk::main();
}
