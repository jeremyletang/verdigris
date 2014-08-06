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

//! Window, keyboard and mouse related events handling

use keyboard::Key;

#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Event {
    Close,
    Move {
        pub x: i32,
        pub y: i32
    },
    Resize {
        pub x: i32,
        pub y: i32
    },
    KeyUp(Key),
    KeyDown(Key),
    MouseMoved {
        pub x: i32,
        pub y: i32
    },
    LeftMouseDown,
    LeftMouseUp,
    RightMouseDown,
    RightMouseUp,
    ScrollWheelUp(f32),
    ScrollWheelDown(f32),
    MouseEntered,
    MouseExited
}

#[doc(hidden)]
pub mod raw {

    pub type event_type_t = ::libc::c_uint;
    pub static CLOSE: ::libc::c_uint = 0;
    pub static MOVE: ::libc::c_uint = 1;
    pub static RESIZE: ::libc::c_uint = 2;
    pub static KEY_UP: ::libc::c_uint = 3;
    pub static KEY_DOWN: ::libc::c_uint = 4;
    pub static MOUSE_MOVED: ::libc::c_uint = 5;
    pub static LEFT_MOUSE_DOWN: ::libc::c_uint = 6;
    pub static LEFT_MOUSE_UP: ::libc::c_uint = 7;
    pub static RIGHT_MOUSE_DOWN: ::libc::c_uint = 8;
    pub static RIGHT_MOUSE_UP: ::libc::c_uint = 9;
    pub static SCROLL_WHEEL_UP: ::libc::c_uint = 10;
    pub static SCROLL_WHEEL_DOWN: ::libc::c_uint = 11;
    pub static MOUSE_ENTERED: ::libc::c_uint = 12;
    pub static MOUSE_EXITED: ::libc::c_uint = 13;

    #[repr(C)]
    pub struct move_t {
        pub _type: event_type_t,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct resize_t {
        pub _type: event_type_t,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct key_up_t {
        pub _type: event_type_t,
        pub key: ::libc::c_int,
    }

    #[repr(C)]
    pub struct key_down_t {
        pub _type: event_type_t,
        pub key: ::libc::c_int,
    }

    #[repr(C)]
    pub struct scroll_wheel_up_t {
        pub _type: event_type_t,
        pub factor: ::libc::c_float,
    }

    #[repr(C)]
    pub struct scroll_wheel_down_t {
        pub _type: event_type_t,
        pub factor: ::libc::c_float,
    }

    #[repr(C)]
    pub struct mouse_moved_t {
        pub _type: event_type_t,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct event_t {
        pub data: [u32, ..3u],
    }

    impl event_t {
        pub fn _type(&mut self) -> *mut event_type_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn move(&mut self) -> *mut move_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn resize(&mut self) -> *mut resize_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn key_up(&mut self) -> *mut key_up_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn key_down(&mut self) -> *mut key_down_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn mouse_move(&mut self) -> *mut mouse_moved_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn scroll_wheel_down(&mut self) -> *mut scroll_wheel_down_t {
            unsafe { ::std::mem::transmute(self) }
        }
        pub fn scroll_wheel_up(&mut self) -> *mut scroll_wheel_up_t {
            unsafe { ::std::mem::transmute(self) }
        }
    }
}