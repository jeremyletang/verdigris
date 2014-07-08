extern crate verdigris;

use verdigris::{Window, VideoMode, Titled, Closable};

fn main() {
    let mode = VideoMode { width: 400, height: 400 };
    let window = Window::new(mode, &[Titled, Closable]);
    loop {}
}