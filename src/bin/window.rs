extern crate verdigris;
extern crate gl;

use verdigris::{Window, VideoMode, ContextSettings};
use verdigris::window_style::{Resizable, Titled, Closable, Miniaturizable};

fn main() {
    gl::load_with(|cstr| verdigris::gl::get_proc_address(cstr));
    let mode = VideoMode { width: 400, height: 400 };
    let mut window = Window::new(mode,
                                 &[Titled, Closable, Resizable, Miniaturizable],
                                 "Hey a new NSWindow !",
                                 ContextSettings::new());
    window.show();
    while !window.should_close() {
        window.poll_event();
        gl::ClearColor(0.9, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        window.display();
    }
    println!("Goodbye !")
}