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

//! Input handling (cursor, mouse, keyboard)

// pub mod keyboard;
// pub mod cursor;
// pub mod mouse;

/// Available mouse buttons
#[deriving(Clone, Show, PartialEq, PartialOrd)]
pub enum MouseButton {
    MouseButtonLeft,
    MouseButtonRight,
    MouseButtonMiddle,
    MouseButtonOther(u8)
}

pub mod mouse {
    use native::Wrapper;
    use window::Window;
    use imp;

    pub fn location(window: &Window) -> (i32, i32) {
        imp::mouse::location(window.unwrap())
    }

    pub fn screen_location() -> (i32, i32) {
        imp::mouse::screen_location()
    }
}

/// Available cursors
#[repr(u32)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Cursor {
    Arrow = 0u32,
    IBeam,
    CrossHair,
    ClosedHand,
    OpenHand,
    PointingHand,
    ResizeLeft,
    ResizeRight,
    ResizeLeftRight,
    ResizeUp,
    ResizeDown,
    ResizeUpDown,
    DisappearingItem,
    IBeamVertical
}

pub mod cursor {
    use super::Cursor;
    use imp;

    pub fn get_default() -> Cursor {
        imp::cursor::get_default()
    }

    pub fn show() {
        imp::cursor::show()
    }

    pub fn hide() {
        imp::cursor::hide()
    }

    pub fn set(cursor: Cursor) {
        imp::cursor::set(cursor)
    }
}

/// Keyboard keys
#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord, Show)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadDecimal,
    KeypadMultiply,
    KeypadPlus,
    KeypadClear,
    KeypadDivide,
    KeypadEnter,
    KeypadMinus,
    KeypadEquals,
    Equal,
    Minus,
    RightBracket,
    LeftBracket,
    Quote,
    Semicolon,
    Backslash,
    Slash,
    Comma,
    Period,
    Grave,
    Return,
    Tab,
    Space,
    Escape,
    Delete,
    Command,
    Shift,
    CapsLock,
    Option,
    Control,
    RightShift,
    RightOption,
    RightControl,
    Fn,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Help,
    Home,
    PageUp,
    PageDown,
    End,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Unknown
}
