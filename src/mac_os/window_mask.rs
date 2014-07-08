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

use window_style::{
    WindowStyle,
    Borderless,
    Titled,
    Closable,
    Miniaturizable,
    Resizable,
    TexturedBackground
};

pub type WindowMask = i32;
pub static NSBorderlessWindowMask: WindowMask = 0;
pub static NSTitledWindowMask: WindowMask = 1 << 0;
pub static NSClosableWindowMask: WindowMask = 1 << 1;
pub static NSMiniaturizableWindowMask: WindowMask = 1 << 2;
pub static NSResizableWindowMask: WindowMask = 1 << 3;
pub static NSTexturedBackgroundWindowMask: WindowMask = 1 << 8;

pub fn from_windowstyle(style: &[WindowStyle]) -> WindowMask {
    let mut mask: WindowMask = NSBorderlessWindowMask;
    for s in style.iter() {
        match *s {
            Borderless => mask = mask | NSBorderlessWindowMask,
            Titled => mask = mask | NSTitledWindowMask,
            Closable => mask = mask | NSClosableWindowMask,
            Miniaturizable => mask = mask | NSMiniaturizableWindowMask,
            Resizable => mask = mask | NSResizableWindowMask,
            TexturedBackground => mask = mask | NSTexturedBackgroundWindowMask
        }
    }

    mask
}