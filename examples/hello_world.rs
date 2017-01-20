extern crate msgbox;

fn main() {
    msgbox::create("Hello Title", "Hello World!", msgbox::IconType::INFO);
    msgbox::create("Error", "Error occured at hello_world.rs:5.\nTerminating..", msgbox::IconType::ERROR);
}