extern crate msgbox;

use msgbox::IconType;

fn main() {
    msgbox::create("Hello Title", "Hello World!", IconType::Info);
    msgbox::create("Error", "Error occured at hello_world.rs:5.\nTerminating..", IconType::Error);
}
