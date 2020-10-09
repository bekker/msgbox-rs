extern crate msgbox;

use msgbox::IconType;
use std::error::Error;

fn main() {
    open_window("Hello Title", "Hello World!", IconType::Info);
    open_window("Error", "Error occured at hello_world.rs:5.\nTerminating..", IconType::Error);
}

fn open_window(title: &str, content: &str, icon_type: IconType) {
    match msgbox::create(title, content, icon_type) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{} (type: {}, title: \"{}\", content: \"{}\")",
                err, icon_type, title, content);
            if err.source().is_some() {
                eprintln!("cause: {}", err.source().unwrap())
            }
        }
    }
}
