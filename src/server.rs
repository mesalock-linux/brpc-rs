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

use crate::{BrpcError, BrpcResult};
use brpc_sys::ffi::{self, BrpcServer, BrpcServerOptions};
use libc::{c_int, c_void};

#[repr(C)]
/// Represent server's ownership of services.
pub enum ServiceOwnership {
    ServerOwnsService = 0,
    ServerDoesntOwnService = 1,
}

#[doc(hidden)]
pub trait Service {
    fn get_service_ptr(&self) -> *mut c_void;
}

/// A `Server` provides a BRPC server where multiple BRPC services can run.
pub struct Server {
    inner: *mut BrpcServer, // brpc_server_t in ffi.cpp
}

impl Server {
    /// Create a new `Server`
    pub fn new() -> Self {
        Server {
            inner: unsafe { ffi::brpc_server_new() },
        }
    }

    /// Add a `Service`. `ownership` represents server's ownership of services.
    /// If `ownership` is `SERVER_OWNS_SERVICE`, server deletes the service at
    /// destruction. To prevent the deletion, set ownership to
    /// `SERVER_DOESNT_OWN_SERVICE`.
    pub fn add_service<T: Service + Sized>(
        &mut self,
        service: &T,
        ownership: ServiceOwnership,
    ) -> BrpcResult<()> {
        let ret = unsafe {
            ffi::brpc_server_add_service(self.inner, service.get_service_ptr(), ownership as c_int)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(BrpcError::EINTERNAL)
        }
    }

    /// Config a `Server` with the provided TCP port and `ServerOptions`.
    pub fn start(&mut self, port: u16, opt: &ServerOptions) -> BrpcResult<()> {
        let ret = unsafe { ffi::brpc_server_start(self.inner, i32::from(port), opt.inner) };
        if ret == 0 {
            Ok(())
        } else {
            Err(BrpcError::EINTERNAL)
        }
    }

    /// Run a `Server` until asked to quit (e.g CTRL-C).
    pub fn run(&mut self) {
        unsafe { ffi::brpc_server_run_until_asked_to_quit(self.inner) };
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            ffi::brpc_server_destroy(self.inner);
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for a `Server`
pub struct ServerOptions {
    #[doc(hidden)]
    pub(crate) inner: *mut BrpcServerOptions, // brpc_server_options_t in ffi.cpp
}

impl ServerOptions {
    /// Make a `ServerOptions` with default values.
    pub fn new() -> Self {
        ServerOptions {
            inner: unsafe { ffi::brpc_server_options_new() },
        }
    }

    /// Notify user when there's no data for at least `idle_timeout_ms`
    /// milliseconds. The default value is -1.
    pub fn set_idle_timeout_ms(&mut self, timeout: i32) {
        unsafe { ffi::brpc_server_options_set_idle_timeout_ms(self.inner, timeout as c_int) }
    }
}

impl Drop for ServerOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::brpc_server_options_destroy(self.inner);
        }
    }
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    struct NullService {}
    impl Service for NullService {
        fn get_service_ptr(&self) -> *mut c_void {
            ptr::null_mut()
        }
    }

    #[test]
    fn server_options_new() {
        let opt = ServerOptions::new();
        assert_ne!(opt.inner, ptr::null_mut());
    }

    #[test]
    fn server_options_set_idle_timeout_ms() {
        let mut opt = ServerOptions::new();
        opt.set_idle_timeout_ms(0);
    }

    #[test]
    fn server_new() {
        let server = Server::new();
        assert_ne!(server.inner, ptr::null_mut());
    }

    #[test]
    fn server_add_null_service() {
        let service = NullService {};
        let mut server = Server::new();
        let ret = server.add_service(&service, ServiceOwnership::ServerDoesntOwnService);
        assert_eq!(false, ret.is_ok()); // NullService must fail to add
    }

    #[test]
    fn server_start_null_service() {
        let service = NullService {};
        let mut server = Server::new();
        let _ = server.add_service(&service, ServiceOwnership::ServerDoesntOwnService);

        let opt = ServerOptions::new();
        let ret = server.start(50000, &opt);
        assert_eq!(true, ret.is_ok()); // NullService must fail to add
    }
}
