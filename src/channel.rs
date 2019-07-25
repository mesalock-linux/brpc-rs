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

use brpc_sys::ffi::{self, BrpcChannel, BrpcChannelOptions};
use libc::c_int;
use std::ffi::CString;
use std::net::SocketAddr;

/// A `Channel` provides a connection to a BRPC server on a specified host and
/// port and is used when creating a client stub
pub struct Channel {
    pub inner: *mut BrpcChannel, // brpc_channel_t in ffi.cpp
}

impl Channel {
    /// Make a `Channel` with the provided socker address and `ChannelOptions`.
    pub fn with_options(sockaddr: &SocketAddr, options: &ChannelOptions) -> Self {
        let inner = unsafe { ffi::brpc_channel_new() };
        // safe to unwrap() because format!(sockaddr) does NOT contains \0
        let server_addr_and_port = CString::new(format!("{}", sockaddr)).unwrap();
        let server_addr_and_port_ptr = server_addr_and_port.as_c_str().as_ptr();
        assert!(
            0 == unsafe { ffi::brpc_channel_init(inner, server_addr_and_port_ptr, options.inner) }
        );
        Channel { inner }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        unsafe {
            ffi::brpc_channel_destroy(self.inner);
        }
    }
}

/// Options for a `Channel`
pub struct ChannelOptions {
    #[doc(hidden)]
    pub inner: *mut BrpcChannelOptions,
}

impl Default for ChannelOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelOptions {
    /// Make a `ChannelOptions` with default values.
    pub fn new() -> Self {
        ChannelOptions {
            inner: unsafe { ffi::brpc_channel_options_new() },
        }
    }

    /// Set max duration of RPC in milliseconds over this Channel. -1 means wait
    /// indefinitely.
    pub fn set_timeout_ms(&mut self, timeout: i32) {
        unsafe { ffi::brpc_channel_options_set_timeout_ms(self.inner, timeout as c_int) }
    }

    /// Set retry limit for RPC over this channel. <=0 means no retry.
    pub fn set_max_retry(&mut self, timeout: i32) {
        unsafe { ffi::brpc_channel_options_set_max_retry(self.inner, timeout as c_int) }
    }
}

impl Drop for ChannelOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::brpc_channel_options_destroy(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn channel_options_new() {
        let opt = ChannelOptions::new();
        assert_ne!(opt.inner, ptr::null_mut());
    }

    #[test]
    fn channel_options_set_timeout() {
        let mut opt = ChannelOptions::new();
        opt.set_timeout_ms(0);
    }

    #[test]
    fn channel_options_set_max_retry() {
        let mut opt = ChannelOptions::new();
        opt.set_max_retry(0);
    }

    #[test]
    fn channel_new_with_options() {
        let opt = ChannelOptions::new();
        let addr = "127.0.0.1:50000".parse().unwrap();
        let ch = Channel::with_options(&addr, &opt);
        assert_ne!(opt.inner, ptr::null_mut());
        assert_ne!(ch.inner, ptr::null_mut());
    }
}
