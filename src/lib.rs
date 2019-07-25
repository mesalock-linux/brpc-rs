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

//! [Apache BRPC](https://github.com/apache/incubator-brpc) is an
//! industrial-grade RPC framework for building reliable and high-performance
//! services. `brpc-rs` enables BRPC clients and servers implemented in the Rust
//! programming language.
//!
//! ## Status
//! This project is currently a prototype under active development. Many APIs
//! are missing; the provided APIs are not guaranteed to be stable until 1.0.
//!
//! ## Prerequisites
//! These dependencies are required for `brpc-rs` and `brpc-build` to work properly.
//!
//! * Apache BRPC: shared library and headers
//! * libprotobuf-dev
//! * libprotoc-dev
//! * protobuf-compiler
//! * libssl-dev
//! * libgflags-dev
//! * libleveldb-dev
//!
//! ## Quickstart
//! Please refer to the latest
//! [README.md](https://github.com/mesalock-linux/brpc-rs/blob/master/README.md).

mod channel;
mod controller;
mod server;

mod error;
pub use error::BrpcError;

#[doc(hidden)]
pub type BrpcResult<T> = Result<T, BrpcError>;

// for user code
pub use channel::{Channel, ChannelOptions};
pub use controller::Controller;
pub use server::{Server, ServerOptions, Service, ServiceOwnership};

// for protoc-generated code
#[doc(hidden)]
pub use brpc_sys as internal;
