extern crate verdigris;
extern crate gl;

use verdigris::{WindowBuilder, VideoMode, ContextSettings};
use verdigris::window_style::{Resizable, Titled, Closable, Miniaturizable};

fn main() {
    gl::load_with(|cstr| verdigris::gl::get_proc_address(cstr));

    let mut window = WindowBuilder::new()
        .video_mode(VideoMode { width: 800, height: 600 })
        .style(&[Titled, Closable, Resizable, Miniaturizable])
        .title("Hey a new NSWindow !")
        .settings(ContextSettings::new())
        .create();

    window.show();
    while !window.should_close() {
        window.poll_event();
        gl::ClearColor(0.9, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        window.display();
    }
    println!("Goodbye !")
}