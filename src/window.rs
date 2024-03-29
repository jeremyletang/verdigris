// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A native OpenGL window

use native::{NativeWindow, Wrapper};
use imp::WindowImpl;
use video_mode::VideoMode;
use window_style::WindowStyle;
use context_settings::ContextSettings;

/// A native OpenGL window
pub struct Window {
    window_impl: WindowImpl,
    on_error: Option<|&str|: 'static>,
    video_mode: VideoMode
}

impl Window {
    pub fn new(mode: VideoMode,
               style: &[WindowStyle],
               title: &str,
               settings: ContextSettings) -> Option<Window> {
        let native_window = NativeWindow::create(mode.clone(), style, title, settings);

        match native_window {
            Some(nw) => {
                Some(Window {
                    window_impl: nw,
                    on_error: None,
                    video_mode: mode
                })
            }
            None => None
        }
    }

    pub fn on_error(&mut self, handler: |&str|: 'static) {
        self.on_error = Some(handler)
    }

    pub fn destroy(&mut self) {
        self.window_impl.destroy()
    }

    pub fn set_title(&mut self, title: &str) {
        self.window_impl.set_title(title)
    }

    pub fn get_title<'r>(&'r self) -> &'r str {
        self.window_impl.get_title()
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.window_impl.set_size(width, height)
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.window_impl.get_size()
    }

    pub fn set_position(&mut self, pos_x: i32, pos_y: i32) {
        self.window_impl.set_position(pos_x, pos_y)
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.window_impl.get_position()
    }

    pub fn reduce(&mut self) {
        self.window_impl.reduce()
    }

    pub fn restore(&mut self) {
        self.window_impl.restore()
    }

    pub fn show(&mut self) {
        self.window_impl.show()
    }

    pub fn hide(&mut self) {
        self.window_impl.hide()
    }

    pub fn set_video_mode(&mut self, video_mode: VideoMode) {
        self.video_mode = video_mode.clone();
        self.window_impl.set_video_mode(video_mode)
    }

    pub fn get_video_mode(&mut self) -> VideoMode {
        self.video_mode.clone()
    }

    pub fn should_close(&self) -> bool {
        self.window_impl.should_close()
    }

    pub fn close(&mut self) {
        self.window_impl.close()
    }

    pub fn poll_event(&mut self) {
        self.window_impl.poll_event()
    }

    pub fn swap_buffers(&mut self) {
        self.window_impl.swap_buffers()
    }
}

impl Wrapper<WindowImpl> for Window {
    fn unwrap(&self) -> &WindowImpl { &self.window_impl }
}
