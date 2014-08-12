extern crate verdigris;
extern crate gl;

use verdigris::{WindowBuilder, VideoMode, ContextSettings};
use verdigris::window_style::{Resizable, Titled, Closable, Miniaturizable};

fn main() {
    gl::load_with(|proc_name| verdigris::gl::get_proc_address(proc_name));

    let mut window = WindowBuilder::new()
        .video_mode(VideoMode { width: 800, height: 600 })
        .style(&[Titled, Closable, Resizable, Miniaturizable])
        .title("Verdigris Window !")
        .settings(ContextSettings::new())
        .create()
        .expect("Cannot create window !");

    window.show();
    while !window.should_close() {
        window.poll_event();

        gl::ClearColor(0.9, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        window.swap_buffers();
    }
    println!("Goodbye !")
}