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

use keyboard::{mod, Key};

pub fn from_keycode(code: u32) -> Key {
    match code {
        0x00 => keyboard::A,
        0x0B => keyboard::B,
        0x08 => keyboard::C,
        0x02 => keyboard::D,
        0x0E => keyboard::E,
        0x03 => keyboard::F,
        0x05 => keyboard::G,
        0x04 => keyboard::H,
        0x22 => keyboard::I,
        0x26 => keyboard::J,
        0x28 => keyboard::K,
        0x25 => keyboard::L,
        0x2E => keyboard::M,
        0x2D => keyboard::N,
        0x1F => keyboard::O,
        0x23 => keyboard::P,
        0x0C => keyboard::Q,
        0x0F => keyboard::R,
        0x01 => keyboard::S,
        0x11 => keyboard::T,
        0x20 => keyboard::U,
        0x09 => keyboard::V,
        0x0D => keyboard::W,
        0x07 => keyboard::X,
        0x10 => keyboard::Y,
        0x06 => keyboard::Z,
        0x12 => keyboard::Num1,
        0x13 => keyboard::Num2,
        0x14 => keyboard::Num3,
        0x15 => keyboard::Num4,
        0x17 => keyboard::Num5,
        0x16 => keyboard::Num6,
        0x1A => keyboard::Num7,
        0x1C => keyboard::Num8,
        0x19 => keyboard::Num9,
        0x1D => keyboard::Num0,
        0x53 => keyboard::Keypad1,
        0x54 => keyboard::Keypad2,
        0x55 => keyboard::Keypad3,
        0x56 => keyboard::Keypad4,
        0x57 => keyboard::Keypad5,
        0x58 => keyboard::Keypad6,
        0x59 => keyboard::Keypad7,
        0x5B => keyboard::Keypad8,
        0x5C => keyboard::Keypad9,
        0x52 => keyboard::Keypad0,
        0x41 => keyboard::KeypadDecimal,
        0x43 => keyboard::KeypadMultiply,
        0x45 => keyboard::KeypadPlus,
        0x47 => keyboard::KeypadClear,
        0x4B => keyboard::KeypadDivide,
        0x4C => keyboard::KeypadEnter,
        0x4E => keyboard::KeypadMinus,
        0x51 => keyboard::KeypadEquals,
        0x18 => keyboard::Equal,
        0x1B => keyboard::Minus,
        0x1E => keyboard::RightBracket,
        0x21 => keyboard::LeftBracket,
        0x27 => keyboard::Quote,
        0x29 => keyboard::Semicolon,
        0x2A => keyboard::Backslash,
        0x2C => keyboard::Slash,
        0x2B => keyboard::Comma,
        0x2F => keyboard::Period,
        0x32 => keyboard::Grave,
        0x24 => keyboard::Return,
        0x30 => keyboard::Tab,
        0x31 => keyboard::Space,
        0x35 => keyboard::Escape,
        0x33 => keyboard::Delete,
        0x37 => keyboard::Command,
        0x38 => keyboard::Shift,
        0x39 => keyboard::CapsLock,
        0x3A => keyboard::Option,
        0x3B => keyboard::Control,
        0x3C => keyboard::RightShift,
        0x3D => keyboard::RightOption,
        0x3E => keyboard::RightControl,
        0x3F => keyboard::Fn,
        0x7A => keyboard::F1,
        0x78 => keyboard::F2,
        0x63 => keyboard::F3,
        0x76 => keyboard::F4,
        0x60 => keyboard::F5,
        0x61 => keyboard::F6,
        0x62 => keyboard::F7,
        0x64 => keyboard::F8,
        0x65 => keyboard::F9,
        0x6D => keyboard::F10,
        0x67 => keyboard::F11,
        0x6F => keyboard::F12,
        0x69 => keyboard::F13,
        0x6B => keyboard::F14,
        0x71 => keyboard::F15,
        0x6A => keyboard::F16,
        0x40 => keyboard::F17,
        0x4F => keyboard::F18,
        0x50 => keyboard::F19,
        0x5A => keyboard::F20,
        0x72 => keyboard::Help,
        0x73 => keyboard::Home,
        0x74 => keyboard::PageUp,
        0x79 => keyboard::PageDown,
        0x77 => keyboard::End,
        0x7B => keyboard::LeftArrow,
        0x7C => keyboard::RightArrow,
        0x7E => keyboard::UpArrow,
        0x7D => keyboard::DownArrow,
        _ => keyboard::Unknown
    }
}
