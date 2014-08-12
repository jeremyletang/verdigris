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

use cursor::Cursor;
use video_mode::VideoMode;
use window_style::WindowStyle;
use context_settings::ContextSettings;

pub trait NativeWindow {
    fn create(mode: VideoMode, style: &[WindowStyle], title: &str, settings: ContextSettings) -> Option<Self>;
    fn destroy(&mut self);
    fn set_title(&mut self, title: &str);
    fn get_title<'r>(&'r self) -> &'r str;
    fn set_size(&mut self, width: i32, height: i32);
    fn get_size(&self) -> (i32, i32);
    fn set_position(&mut self, pos_x: i32, pos_y: i32);
    fn get_position(&self) -> (i32, i32);
    fn reduce(&mut self);
    fn restore(&mut self);
    fn show(&mut self);
    fn hide(&mut self);
    fn set_video_mode(&mut self, video_mode: VideoMode);
    fn get_video_mode(&mut self) -> VideoMode;
    fn should_close(&self) -> bool;
    fn close(&mut self);
    fn poll_event(&mut self);
    fn swap_buffers(&mut self);
}

pub trait NativeCursor {
    fn show();
    fn hide();
    fn set(cursor: Cursor);
}

pub trait Wrapper<T> {
    fn unwrap(&self) -> &T;
    fn wrap(_: T) {}
}