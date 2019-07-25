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

use crate::BrpcError;
use brpc_sys::ffi::{self, BrpcController, BrpcIOBuf};

#[doc(hidden)]
pub struct Controller {
    pub inner: *mut BrpcController,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            inner: unsafe { ffi::brpc_controller_new() },
        }
    }

    pub fn failed(&self) -> bool {
        unsafe { 1 == ffi::brpc_controller_failed(self.inner) }
    }

    pub fn error(&self) -> BrpcError {
        let error_code = unsafe { ffi::brpc_controller_error_code(self.inner) };
        BrpcError::from(error_code)
    }

    pub fn request_attachment(&self) -> *mut BrpcIOBuf {
        unsafe { ffi::brpc_controller_get_request_attachment(self.inner) }
    }

    pub fn response_attachment(&self) -> *mut BrpcIOBuf {
        unsafe { ffi::brpc_controller_get_response_attachment(self.inner) }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            ffi::brpc_controller_destroy(self.inner);
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn controller_new() {
        let cntl = Controller::new();
        assert_ne!(cntl.inner, ptr::null_mut());
    }

    #[test]
    fn controller_failed() {
        let cntl = Controller::new();
        assert_eq!(false, cntl.failed());
    }

    #[test]
    fn controller_error_code() {
        let cntl = Controller::new();
        assert_eq!(BrpcError::NOERROR, cntl.error());
    }

    #[test]
    fn controller_get_request_attachment() {
        let cntl = Controller::new();
        assert_ne!(cntl.request_attachment(), ptr::null_mut());
    }

    #[test]
    fn controller_get_response_attachment() {
        let cntl = Controller::new();
        assert_ne!(cntl.response_attachment(), ptr::null_mut());
    }
}
