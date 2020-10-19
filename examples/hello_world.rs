extern crate msgbox;

use msgbox::common::MsgBoxError;
use msgbox::IconType;

fn main() {
    open_window("Hello Title", "Hello World!", IconType::Info);
    open_window(
        "Error",
        "Error occured at hello_world.rs:5.\nTerminating..",
        IconType::Error,
    );
}

fn open_window(title: &str, content: &str, icon_type: IconType) {
    match msgbox::create(title, content, icon_type) {
        Ok(()) => (),
        Err(MsgBoxError::Create(_)) => {
            eprintln!(
                "Creation error: (type: {}, title: \"{}\", content: \"{}\")",
                icon_type, title, content
            );
        }
    }
}
