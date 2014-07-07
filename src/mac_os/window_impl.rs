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

#![allow(unused_variable)]

use native::NativeWindow;
use video_mode::VideoMode;

pub struct WindowImpl {
    window: ::objcruntime::id
}

impl NativeWindow for WindowImpl {
    fn create() -> WindowImpl {
        unimplemented!()
    }

    fn destroy(&mut self) {
        unimplemented!()
    }

    fn set_title(&mut self, title: &str) {
        unimplemented!()
    }

    fn get_title<'r>(&'r self) -> &'r str {
        unimplemented!()
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
        unimplemented!()
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

}