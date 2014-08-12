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

#![allow(unused_variable, unused_unsafe)]

use native::NativeWindow;
use imp::{ffi, window_mask, context_settings};
use window_style::WindowStyle;
use video_mode::VideoMode;
use context_settings::ContextSettings;

pub struct WindowImpl {
    pub window_handler: ffi::id,
    pub title: String,
}

impl NativeWindow for WindowImpl {
    fn create(mode: VideoMode, 
             style: &[WindowStyle], 
             title: &str, 
             settings: ContextSettings) -> Option<WindowImpl> {
        let w_mask = window_mask::from_windowstyle(style);
        let w_size = ffi::NSSize { width: mode.width as f64, height: mode.height as f64 };
        let w_settings =context_settings::from_struct(&settings);
        let w_handler = ffi::ve_windowhandler_new(w_size, w_mask, w_settings.as_ptr());
        title.with_c_str(|c_str| ffi::ve_windowhandler_set_title(w_handler, c_str));

        if w_handler.ptr.is_null() {
            None
        } else {
            Some(WindowImpl {
                window_handler: w_handler,
                title: title.to_string()
            })
        }
    }

    fn destroy(&mut self) {
        unimplemented!()
    }

    fn set_title(&mut self, title: &str) {
        title.with_c_str(|c_str| ffi::ve_windowhandler_set_title(self.window_handler, c_str));
    }

    fn get_title<'r>(&'r self) -> &'r str {
        self.title.as_slice()
    }

    fn set_size(&mut self, width: i32, height: i32) {
        unimplemented!()
    }

    fn get_size(&self) -> (i32, i32) {
        unimplemented!()
    }

    fn set_position(&mut self, pos_x: i32, pos_y: i32) {
        unimplemented!()
    }

    fn get_position(&self) -> (i32, i32) {
        unimplemented!()
    }

    fn reduce(&mut self) {
        unimplemented!()
    }

    fn restore(&mut self) {
        unimplemented!()
    }

    fn show(&mut self) {
        ffi::ve_windowhandler_show(self.window_handler)
    }

    fn hide(&mut self) {
        unimplemented!()
    }

    fn set_video_mode(&mut self, video_mode: VideoMode) {
        unimplemented!()
    }

    fn get_video_mode(&mut self) -> VideoMode {
        unimplemented!()
    }

    fn should_close(&self) -> bool {
        let close: ffi::BOOL = ffi::ve_windowhandler_should_close(self.window_handler);
        ffi::to_bool(close)
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn poll_event(&mut self) {
        ffi::ve_windowhandler_fetch_events(self.window_handler)
    }

    fn swap_buffers(&mut self) {
        ffi::ve_windowhandler_swap_buffers(self.window_handler)
    }

}