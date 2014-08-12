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

//! Helper struct to create windows easily

use window::Window;
use video_mode::VideoMode;
use window_style::WindowStyle;
use context_settings::ContextSettings;

/// Helper struct to create windows easily
pub struct WindowBuilder {
    style: Option<Vec<WindowStyle>>,
    video_mode: Option<VideoMode>,
    title: Option<String>,
    settings: Option<ContextSettings>
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            style: None,
            video_mode: None,
            title: None,
            settings: None
        }
    }

    pub fn title(mut self, title: &str) -> WindowBuilder{
        self.title = Some(title.to_string());
        self
    }

    pub fn video_mode(mut self, video_mode: VideoMode) -> WindowBuilder {
        self.video_mode = Some(video_mode);
        self
    }

    pub fn style(mut self, style: &[WindowStyle]) -> WindowBuilder {
        self.style = Some(style.to_vec());
        self
    }

    pub fn settings(mut self, settings: ContextSettings) -> WindowBuilder {
        self.settings = Some(settings);
        self
    }

    pub fn create(self) -> Window {
        let style = self.style.as_ref().clone();
        let title = self.title.as_ref().clone();
        Window::new(self.video_mode.unwrap(),
                    style.unwrap().as_slice(),
                    title.unwrap().as_slice(),
                    self.settings.unwrap())
    }
}