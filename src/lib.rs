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

#![crate_name = "verdigris"]
#![desc = "Multi plateform opengl windowing for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![allow(dead_code, non_camel_case_types, missing_doc)]
#![feature(struct_variant)]
#![feature(globs)]
#![unstable]

extern crate libc;

pub use self::window::Window;
pub use self::video_mode::VideoMode;
pub use self::window_builder::WindowBuilder;
pub use self::context_settings::ContextSettings;

#[cfg(target_os = "macos")]
#[path = "mac_os/mod.rs"]
mod imp;

#[cfg(target_os = "wind32")]
#[path = "windows/mod.rs"]
mod imp;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod imp;

mod native;
mod window;
mod video_mode;
mod window_builder;
pub mod context_settings;
pub mod window_style;
pub mod event;
pub mod inputs;
pub mod gl;
