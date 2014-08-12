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

//! Settings for OpenGL context creation

#[deriving(Clone, Show, PartialEq)]
pub enum GlProfile {
    CoreProfile,
    CompatibilityProfile,
    ESProfile
}

/// Settings for OpenGL context creation
#[deriving(Clone, Show, PartialEq)]
pub struct ContextSettings {
    pub red_size: u32,
    pub green_size: u32,
    pub blue_size: u32,
    pub alpha_size: u32,
    pub buffer_size: u32,
    pub double_buffer: bool,
    pub depth_size: u32,
    pub stencil_size: u32,
    pub accum_red_size: u32,
    pub accum_green_size: u32,
    pub accum_blue_size: u32,
    pub accum_alpha_size: u32,
    pub stereo: bool,
    pub multisample_buffers: u32,
    pub multisample_samples: u32,
    pub accelerated_visual: bool,
    pub major_version: u32,
    pub minor_version: u32,
    pub profile: GlProfile,
    pub high_dpi: bool
}

impl ContextSettings {
    /// initialize with default value from sdl2
    pub fn new() -> ContextSettings {
        ContextSettings {
            red_size: 3u32,
            green_size: 3u32,
            blue_size: 2u32,
            alpha_size: 0u32,
            buffer_size: 0u32,
            double_buffer: true,
            depth_size: 16u32,
            stencil_size: 0u32,
            accum_red_size: 0u32,
            accum_green_size: 0u32,
            accum_blue_size: 0u32,
            accum_alpha_size: 0u32,
            stereo: false,
            multisample_buffers: 0u32,
            multisample_samples: 0u32,
            accelerated_visual: true,
            major_version: 3u32,
            minor_version: 2u32,
            profile: CoreProfile,
            high_dpi: true,
        }
    }
}
