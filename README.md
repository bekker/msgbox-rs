# msgbox-rs

[![](http://meritbadge.herokuapp.com/msgbox)](https://crates.io/crates/msgbox)

| OS | Build Status
| -- | -----
| Linux & OSX | [![Build Status](https://travis-ci.org/bekker/msgbox-rs.svg?branch=master)](https://travis-ci.org/bekker/msgbox-rs)
| Windows |

![Example for windows](examples/hello_world_windows.png?raw=true "Example for windows")
![Example for Linux](examples/hello_world_linux.png?raw=true "Example for linux")

Simple, cross-platform message box GUI library.

All it does is to show a message box modal with a OK button, which runs synchronously.

All the other jobs stop until the user responds.
It runs fine with OpenGL windows already open.

It supports multi-platform, and maintains separate dependencies per platform, thus light-weight.

I use this library when an error occurs in OpenGL applications with [glium](https://github.com/tomaka/glium).

 - Synchronous Message Modal
 - Multi-platform (Linux GTK3+, Windows and OS X so far)
 - Light-weight

```toml
[dependencies]
msgbox = "0.1.0"
```

## Platform support
* Linux with GTK 3+ (Tested on Ubuntu Gnome 16.04)
* Windows (Tested on Windows 8.1)
* OS X

As long as [gtk](https://github.com/gtk-rs/gtk) or [winapi](https://github.com/retep998/winapi-rs) library working, any platform with it would work.

## Dev Requirements

### Linux
* `libgtk-3-dev` for apt
* `gtk3-devel` for yum

### Windows
* Windows version compatible with [winapi](https://github.com/retep998/winapi-rs)

### OS X
* `cairo gtk+3` for brew

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
