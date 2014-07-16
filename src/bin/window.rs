extern crate verdigris;

use verdigris::{Window, VideoMode, Titled, Closable, Resizable, Miniaturizable};

fn main() {
    let mode = VideoMode { width: 400, height: 400 };
    let mut window = Window::new(mode, &[Titled, Closable, Resizable, Miniaturizable], "Hey a new NSWindow !");
    window.set_title("hum this seems a better name...");
    window.show();
    while !window.should_close() {
        window.poll_event();
    }

    println!("Goodbye !")
}