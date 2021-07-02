use gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType};

use common::{IconType, MsgBoxError};

#[derive(thiserror::Error, Debug)]
pub enum GtkError {
    #[error("failed to initialize GTK")]
    Init,
}

pub fn create(
    title: &str,
    content: &str,
    icon_type: IconType,
) -> std::result::Result<(), MsgBoxError> {
    gtk::init()?;

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
        content,
    );
    dialog.set_title(title);
    dialog.set_modal(true);
    dialog.set_decorated(true);
    dialog.set_keep_above(true);
    dialog.show();
    dialog.connect_response(move |dialog, _| {
        unsafe { dialog.destroy() };
        gtk::main_quit();
        Inhibit(false);
    });
    gtk::main();

    Ok(())
}
