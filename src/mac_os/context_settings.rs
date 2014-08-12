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

use context_settings::{ContextSettings, CoreProfile, CompatibilityProfile, ESProfile};


#[repr(u32)]
enum NativeProfile {
    NSOpenGLProfileVersionLegacy    = 0x1000,
    NSOpenGLProfileVersion3_2Core   = 0x3200
}

#[repr(u32)]
enum NativeSettings {
    NSOpenGLPFAAllRenderers       =   1u32,
    NSOpenGLPFATripleBuffer       =   3u32,
    NSOpenGLPFADoubleBuffer       =   5u32,
    NSOpenGLPFAStereo             =   6u32,
    NSOpenGLPFAAuxBuffers         =   7u32,
    NSOpenGLPFAColorSize          =   8u32,
    NSOpenGLPFAAlphaSize          =  11u32,
    NSOpenGLPFADepthSize          =  12u32,
    NSOpenGLPFAStencilSize        =  13u32,
    NSOpenGLPFAAccumSize          =  14u32,
    NSOpenGLPFAMinimumPolicy      =  51u32,
    NSOpenGLPFAMaximumPolicy      =  52u32,
    NSOpenGLPFAOffScreen          =  53u32,
    NSOpenGLPFAFullScreen         =  54u32,
    NSOpenGLPFASampleBuffers      =  55u32,
    NSOpenGLPFASamples            =  56u32,
    NSOpenGLPFAAuxDepthStencil    =  57u32,
    NSOpenGLPFAColorFloat         =  58u32,
    NSOpenGLPFAMultisample        =  59u32,
    NSOpenGLPFASupersample        =  60u32,
    NSOpenGLPFASampleAlpha        =  61u32,
    NSOpenGLPFARendererID         =  70u32,
    NSOpenGLPFASingleRenderer     =  71u32,
    NSOpenGLPFANoRecovery         =  72u32,
    NSOpenGLPFAAccelerated        =  73u32,
    NSOpenGLPFAClosestPolicy      =  74u32,
    NSOpenGLPFARobust             =  75u32,
    NSOpenGLPFABackingStore       =  76u32,
    NSOpenGLPFAMPSafe             =  78u32,
    NSOpenGLPFAWindow             =  80u32,
    NSOpenGLPFAMultiScreen        =  81u32,
    NSOpenGLPFACompliant          =  83u32,
    NSOpenGLPFAScreenMask         =  84u32,
    NSOpenGLPFAPixelBuffer        =  90u32,
    NSOpenGLPFARemotePixelBuffer  =  91u32,
    NSOpenGLPFAAllowOfflineRenderers = 96u32,
    NSOpenGLPFAAcceleratedCompute =  97u32,
    NSOpenGLPFAOpenGLProfile      =  99u32,
    NSOpenGLPFAVirtualScreenCount = 128u32
}

//     pub multisample_buffers: u32,
//     pub multisample_samples: u32,
//     pub accelerated_visual: bool,
//     pub high_dpi: bool


pub fn from_struct(settings: &ContextSettings) -> Vec<u32> {
    let mut attr: Vec<u32> = vec![];

    attr.push(NSOpenGLPFAClosestPolicy as u32);

    attr.push_all([NSOpenGLPFAColorSize as u32,
                   settings.green_size
                   + settings.red_size
                   + settings.blue_size]);

    if settings.double_buffer {
        attr.push(NSOpenGLPFADoubleBuffer as u32);
    }

    attr.push_all([NSOpenGLPFAAlphaSize as u32, settings.alpha_size]);
    attr.push_all([NSOpenGLPFADepthSize as u32, settings.depth_size]);
    attr.push_all([NSOpenGLPFAStencilSize as u32, settings.stencil_size]);

    attr.push_all([NSOpenGLPFAAccumSize as u32,
                   settings.accum_red_size
                   + settings.accum_green_size
                   + settings.accum_blue_size
                   + settings.accum_alpha_size]);

    if settings.stereo {
        attr.push(NSOpenGLPFAStereo as u32);
    }

    match settings.profile {
        CoreProfile =>
            attr.push_all([NSOpenGLPFAOpenGLProfile as u32, NSOpenGLProfileVersion3_2Core as u32]),
        CompatibilityProfile =>
            attr.push_all([NSOpenGLPFAOpenGLProfile as u32, NSOpenGLProfileVersionLegacy as u32]),
        ESProfile => fail!("incompatible OpenGL ES profile.")
    }

    if settings.multisample_samples != 0 {
        attr.push_all([NSOpenGLPFASampleBuffers as u32, settings.multisample_samples]);
        attr.push_all([NSOpenGLPFASamples as u32, settings.multisample_samples]);
    }

    attr.push(0u32);
    attr
}
