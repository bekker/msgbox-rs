extern crate gtk;
use self::gtk::prelude::*;

pub fn create(title:&str, content:&str, btn_text:&str) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        println!("{}", content);
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title(title);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_size_request(350, 70);
    window.set_resizable(false);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let hbox = gtk::Box::new(gtk::Orientation::Vertical, 20);

    let label = gtk::Label::new(Some(content));
    hbox.add(&label);

    let button = gtk::Button::new_with_label(btn_text);

    button.connect_clicked(move |_| {
        gtk::main_quit();
        Inhibit(false);
    });
    hbox.add(&button);

    window.add(&hbox);
    window.show_all();
    gtk::main();
}