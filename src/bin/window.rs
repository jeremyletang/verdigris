#[crate_name = "window"]

extern crate verdigris;

use verdigris::{Window, VideoMode, window_style};

fn main() {
    let mode = VideoMode { width: 400, height: 400 };
    let window = Window::new(mode, window_style::Titled);
    loop {}
}