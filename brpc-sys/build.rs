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

fn main() {
    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .file("src/ffi.cpp")
        .file("src/zero_copy.cpp")
        .flag("-std=c++11")
        .flag_if_supported("-Wno-everything")
        .warnings(false);

    builder.compile("brpc_ffi");
    println!("cargo:rustc-link-lib=static=brpc_ffi");

    println!("cargo:rustc-link-lib=brpc");
    println!("cargo:rustc-link-lib=protobuf");
    println!("cargo:rustc-link-lib=gflags");
    println!("cargo:rustc-link-lib=leveldb");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
}
