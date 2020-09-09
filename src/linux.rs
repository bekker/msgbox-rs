use ::gtk;
use ::gtk::prelude::*;
use ::gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog};

use common::{IconType, MsgBoxCreationError};

pub fn create<'a>(title:&'a str, content:&'a str, icon_type:IconType) -> std::result::Result<(), MsgBoxCreationError<'a>> {
    if gtk::init().is_err() {
        return Err(MsgBoxCreationError {
            title,
            content,
            icon_type
        });
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

    Ok(())
}
