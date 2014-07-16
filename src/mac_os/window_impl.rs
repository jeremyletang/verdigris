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
use native_impl::window_mask;
use window_style::WindowStyle;
use video_mode::VideoMode;
use foundation::{NSString, NSSize};
use native_impl::ffi;
use objc = objcruntime;

pub struct WindowImpl {
    window_handler: objc::id,
    title: String,
}

impl NativeWindow for WindowImpl {
    fn create(mode: VideoMode, style: &[WindowStyle], title: &str) -> WindowImpl {
        let w_mask = window_mask::from_windowstyle(style);
        let w_title = NSString::from_str(title);
        let w_size = NSSize { width: mode.width as f64, height: mode.height as f64 };
        // let w_handler = m![m![cls!(VEWindowHandler) alloc] initWithSize: w_size
                                                               // AndWindowStyle: w_mask];
        let w_handler = unsafe { ffi::ve_windowhandler_new(w_size, w_mask) };
        m![w_handler setTitle: NSString::from_str(title)];
        WindowImpl {
            window_handler: w_handler,
            title: title.to_string()
        }
    }

    fn destroy(&mut self) {
        unimplemented!()
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        let w_title = NSString::from_str(title);
        m![self.window_handler setTitle: w_title];
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
        m![self.window_handler show];
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
        let should: uint = unsafe { ::std::mem::transmute(m![self.window_handler shouldClose]) };
        match should {
            0 => false,
            _ => true
        }
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn poll_event(&mut self) {
        m![self.window_handler fetchEvents];
    }

}