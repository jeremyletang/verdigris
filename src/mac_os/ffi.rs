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

#![allow(raw_pointer_deriving)]

use libc::{c_void, c_int, c_char, c_uint};

pub type BOOL = i8;
pub static YES: BOOL = 1;
pub static NO: BOOL = 0;

pub fn to_bool(value: BOOL) -> bool {
    value == YES
}

#[allow(non_snake_case_functions)]
pub fn to_BOOL(value: bool) -> BOOL {
    match value {
        true => YES,
        false => NO
    }
}

#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct id {
    pub ptr: *mut c_void
}

#[deriving(Clone, PartialEq, PartialOrd, Show, Default)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize
}

#[deriving(Clone, PartialEq, PartialOrd, Show, Default)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64
}

#[deriving(Clone, PartialEq, PartialOrd, Show, Default)]
pub struct NSSize {
    pub width: f64,
    pub height: f64
}

pub fn ve_windowhandler_new(size: NSSize, style: c_int, context: *const u32) -> id {
    unsafe { glue::ve_windowhandler_new(size, style, context) }
}

pub fn ve_windowhandler_set_title(window_handler: id, title: *const c_char) {
    unsafe { glue::ve_windowhandler_set_title(window_handler, title) }
}

pub fn ve_windowhandler_fetch_events(window_handler: id) {
    unsafe { glue::ve_windowhandler_fetch_events(window_handler) }
}

pub fn ve_windowhandler_show(window_handler: id) {
    unsafe { glue::ve_windowhandler_show(window_handler) }
}

pub fn ve_windowhandler_should_close(window_handler: id) -> BOOL {
    unsafe { glue::ve_windowhandler_should_close(window_handler) }
}

pub fn ve_cursor_show() {
    unsafe { glue::ve_cursor_show() }
}

pub fn ve_cursor_hide() {
    unsafe { glue::ve_cursor_hide() }
}

pub fn ve_cursor_set(cursor: c_uint) {
    unsafe { glue::ve_cursor_set(cursor) }
}

pub fn ve_mouse_get_location(window_handler: id) -> NSPoint {
    unsafe { glue::ve_mouse_get_location(window_handler) }
}

pub fn ve_mouse_get_global_location() -> NSPoint {
    unsafe { glue::ve_mouse_get_global_location() }
}

pub fn ve_get_proc_address(proc_name: *const c_char) -> *const c_void {
    unsafe { glue::ve_get_proc_address(proc_name) }
}

pub fn ve_windowhandler_swap_buffers(window_handler: id) {
    unsafe {  glue::ve_windowhandler_swap_buffers(window_handler) }
}


mod glue {

    use super::{NSSize, NSPoint, BOOL, id};
    use libc::{c_int, c_char, c_uint, c_void};

    #[link(name = "verdigrisglue")]
    extern {
        // window handler
        pub fn ve_windowhandler_new(size: NSSize, style: c_int, context: *const u32) -> id;
        pub fn ve_windowhandler_set_title(window_handler: id, title: *const c_char);
        pub fn ve_windowhandler_fetch_events(window_handler: id);
        pub fn ve_windowhandler_show(window_handler: id);
        pub fn ve_windowhandler_should_close(window_handler: id) -> BOOL;
        pub fn ve_windowhandler_swap_buffers(window_handler: id);

        // cursor
        pub fn ve_cursor_show();
        pub fn ve_cursor_hide();
        pub fn ve_cursor_set(cursor: c_uint);

        // mouse
        pub fn ve_mouse_get_global_location() -> NSPoint;
        pub fn ve_mouse_get_location(window_handler: id) -> NSPoint;

        // gl
        pub fn ve_get_proc_address(proc_name: *const c_char) -> *const c_void;
    }
}

#[allow(non_snake_case_functions)]
pub fn __CFStringMakeConstantString(c: *const c_char) -> *const c_void {
    unsafe { cf::__CFStringMakeConstantString(c) }
}

#[allow(non_snake_case_functions)]
pub fn CFBundleGetBundleWithIdentifier(ptr: *const c_void) -> *const c_void {
    unsafe { cf::CFBundleGetBundleWithIdentifier(ptr) }
}

#[allow(non_snake_case_functions)]
pub fn CFBundleGetFunctionPointerForName(p1: *const c_void, p2: *const c_void) -> *const c_void {
    unsafe { cf::CFBundleGetFunctionPointerForName(p1, p2) }
}

#[allow(non_snake_case_functions)]
pub fn CFRelease(cfstr: *const c_void) {
    unsafe { cf::CFRelease(cfstr) }
}

mod cf {

    use libc::{c_char, c_void};

    #[link(name = "CoreFoundation", kind = "framework")]
    #[link(name = "Cocoa", kind = "framework")]
    extern "C" {
        pub fn __CFStringMakeConstantString(c: *const c_char) -> *const c_void;
        pub fn CFBundleGetBundleWithIdentifier(ptr: *const c_void) -> *const c_void;
        pub fn CFBundleGetFunctionPointerForName(p1: *const c_void, p2: *const c_void) -> *const c_void;
        pub fn CFRelease(cfstr: *const c_void);
    }
}

// vsync
//     GLint swapInterval = enabled ? 1 : 0;
//     [m_context setValues:&swapInterval forParameter:NSOpenGLCPSwapInterval];
