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

use libc::c_void;

use imp::ffi;

local_data_key!(key_bundle: *const c_void)

// pub fn get_proc_address(proc_name: &str) -> *const c_void {
//     let _proc_name = "_".to_string() + proc_name;
//     proc_name.with_c_str(|c_str|{
//         ffi::ve_get_proc_address(c_str)
//     })
// }

pub fn get_proc_address(proc_name: &str) ->  *const c_void {
    // get mac os opengl bundle
    let bundle = match key_bundle.get() {
        Some(b) => *b,
        None => {
            let id = "com.apple.opengl".with_c_str(|c_str| ffi::__CFStringMakeConstantString(c_str));
            let bundle = ffi::CFBundleGetBundleWithIdentifier(id);
            if bundle.is_null() { return ::std::ptr::null(); } // bundle not found
            key_bundle.replace(Some(bundle));
            bundle
        }
    };

    // find the symbole in the bundle
    let mut symbol: *const c_void;
    let cf_proc_name = proc_name.with_c_str(|c_str| ffi::__CFStringMakeConstantString(c_str));
    symbol = ffi::CFBundleGetFunctionPointerForName(bundle, cf_proc_name);

    // release CFStringRef
    ffi::CFRelease(cf_proc_name);

    // the symbol or null if not found
    symbol
}