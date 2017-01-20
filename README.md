# msgbox-rs

[![](http://meritbadge.herokuapp.com/msgbox)](https://crates.io/crates/msgbox)

Simple, cross-platform message box GUI library.

All it does is to show a message box modal with a OK button, which runs synchronously.

All the other jobs stop until the user responds.
It runs fine with OpenGL windows already open.

It supports multi-platform, and maintains separate dependencies per platform, thus light-weight.

I use this library when an error occurs in OpenGL applications with [glium](https://github.com/tomaka/glium).

 - Synchronous Message Modal
 - Multi-platform (Linux GTK3+ and Windows so far)
 - Light-weight

```toml
[dependencies]
msgbox = "0.1.0"
```

## Platform support
* Linux with GTK 3+ (Tested on Ubuntu Gnome 16.04)
* Windows (Tested on Windows 8.1)

Planning to support OS X, but I don't have any devices with it. Contributions are welcome.

## Dev Requirements
* libgtk-3-dev for Ubuntu
* gtk3-devel for CentOS
* nothing for Windows

## Usage

```rust
extern crate msgbox;

fn main() {
    msgbox::create("Hello Title", "Hello World!", msgbox::IconType::INFO);
    msgbox::create("Error", "Error occured at hello_world.rs:5.\nTerminating..", msgbox::IconType::ERROR);
}
```

## License
Distributed under MIT License
