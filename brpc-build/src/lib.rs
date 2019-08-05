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

//! `brpc-build` compiles `.proto` files for `brpc-rs`.
//!
//! `brpc-build` is designed to be used for build-time code generation as part of
//! a Cargo build-script.

#![deny(warnings)]

use std::path::{Path, PathBuf};
use std::{env, io, path, process};

/// Compile .proto files into Rust files during a Cargo build.
///
/// The generated `.rs` files will be written to the Cargo `OUT_DIR` directory,
/// suitable for use with the `include!` macro.
///
/// This function should be called in a project's `build.rs`.
///
/// # Arguments
///
/// **`protos`** - Paths to `.proto` files to compile. Any transitively
/// [imported][3] `.proto` files will automatically be included.
///
/// **`includes`** - Paths to directories in which to search for imports.
/// Directories will be searched in order. The `.proto` files passed in
/// **`protos`** must be found in one of the provided include directories.
///
/// It's expected that this function call be `unwrap`ed in a `build.rs`; there
/// is typically no reason to gracefully recover from errors during a build.
///
/// # Example `build.rs`
///
/// ```norun
/// fn main() {
///     brpc_build::compile_protos(&["src/echo.proto",],
///                                 &["src"]).unwrap();
/// }
/// ```
pub fn compile_protos<P>(protos: &[P], includes: &[P]) -> io::Result<()>
where
    P: AsRef<path::Path>,
{
    let out_dir_path: path::PathBuf = env::var_os("OUT_DIR")
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "OUT_DIR env var is not set"))
        .map(Into::into)?;
    let out_dir = out_dir_path.as_os_str();

    let brpc_plugin_path = find_in_path("protoc-gen-brpc").ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "protoc-gen-brpc not found in PATH",
    ))?;

    // Step 0
    let _ = prost_build::compile_protos(protos, includes)?;

    // Step 1
    let mut cmd = process::Command::new("protoc");
    for include in includes {
        cmd.arg("-I").arg(include.as_ref());
    }
    for proto in protos {
        cmd.arg(proto.as_ref());
    }
    cmd.arg(&format!(
        "--plugin=protoc-gen=brpc={}",
        brpc_plugin_path.to_string_lossy()
    ));
    cmd.arg("--brpc_out").arg(&out_dir);

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "protoc failed in the first pass: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // Step 2
    let mut cmd = process::Command::new("protoc");
    let current_dir = out_dir_path.to_path_buf();
    cmd.arg("-I").arg(out_dir_path.to_path_buf());
    for proto in protos {
        let f = proto
            .as_ref()
            .file_name()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid file name"))?;
        cmd.arg(current_dir.join(f));
    }
    cmd.arg("--cpp_out").arg(out_dir_path.to_path_buf());
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "protoc failed in the second pass: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // Step 3
    let mut builder = cc::Build::new();
    for proto in protos {
        let mut cc_to_build = out_dir_path.to_path_buf();
        let f = proto
            .as_ref()
            .file_name()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid file name"))?;
        cc_to_build.push(f);
        cc_to_build.set_extension("brpc.cc");
        builder.file(&cc_to_build);

        let mut cc_to_build = out_dir_path.to_path_buf();
        let f = proto
            .as_ref()
            .file_name()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid file name"))?;
        cc_to_build.push(f);
        cc_to_build.set_extension("pb.cc");
        builder.file(&cc_to_build);
    }

    builder.cpp(true).flag("-std=c++11").warnings(false);
    builder.compile("brpc_service");
    println!("cargo:rustc-link-lib=static=brpc_service");

    println!("cargo:rustc-link-lib=brpc");
    println!("cargo:rustc-link-lib=protobuf");
    println!("cargo:rustc-link-lib=gflags");
    println!("cargo:rustc-link-lib=leveldb");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");

    Ok(())
}

// find executable file in $PATH (%PATH% in windows)
fn find_in_path<E: AsRef<Path>>(exe: E) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}
