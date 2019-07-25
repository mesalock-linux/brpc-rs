// Copyright 2019 Baidu, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use libc::{c_char, c_int};
use std::{env, ffi};

extern "C" {
    fn cpp_main(argv: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    let args = env::args()
        .map(|arg| ffi::CString::new(arg).unwrap())
        .collect::<Vec<ffi::CString>>();
    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    unsafe {
        let _ = cpp_main(c_args.len() as c_int, c_args.as_ptr());
    }
}
