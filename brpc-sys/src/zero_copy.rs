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

use crate::ffi::BrpcIOBuf;
use bytes::{Buf, BufMut};
use std::os::raw::{c_int, c_ulonglong, c_void}; // traits

pub enum BrpcZeroCopyBuf {}
pub enum BrpcZeroCopyBufMut {}

pub struct ZeroCopyBuf {
    inner: *mut BrpcZeroCopyBuf,
}

impl ZeroCopyBuf {
    pub unsafe fn from_iobuf(ptr: *mut BrpcIOBuf) -> Self {
        ZeroCopyBuf {
            inner: zero_copy_buf_new(ptr),
        }
    }

    pub unsafe fn from_raw_ptr(ptr: *mut BrpcZeroCopyBuf) -> Self {
        ZeroCopyBuf { inner: ptr }
    }
}

impl Buf for ZeroCopyBuf {
    fn remaining(&self) -> usize {
        unsafe { zero_copy_buf_remaining(self.inner) as usize }
    }

    fn bytes(&self) -> &[u8] {
        let mut buf_ptr: *const c_void = std::ptr::null_mut();
        let mut size: c_int = 0;
        unsafe {
            let _ = zero_copy_buf_bytes(
                self.inner,
                &mut buf_ptr as *mut *const c_void,
                &mut size as *mut c_int,
            );
            std::slice::from_raw_parts(buf_ptr as *const u8, size as usize)
        }
    }

    fn advance(&mut self, cnt: usize) {
        // Panic if zero_copy_buf_advance() failed.
        unsafe {
            assert_eq!(1, zero_copy_buf_advance(self.inner, cnt as c_int));
        }
    }
}

pub struct ZeroCopyBufMut {
    inner: *mut BrpcZeroCopyBufMut,
}

impl ZeroCopyBufMut {
    pub unsafe fn from_iobuf(ptr: *mut BrpcIOBuf) -> Self {
        ZeroCopyBufMut {
            inner: zero_copy_buf_mut_new(ptr),
        }
    }

    pub unsafe fn from_raw_ptr(ptr: *mut BrpcZeroCopyBufMut) -> Self {
        ZeroCopyBufMut { inner: ptr }
    }
}

impl BufMut for ZeroCopyBufMut {
    fn remaining_mut(&self) -> usize {
        unsafe { zero_copy_buf_mut_remaining(self.inner) as usize }
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        let mut buf_ptr: *mut c_void = std::ptr::null_mut();
        let mut size: c_int = 0;
        let _ = zero_copy_buf_mut_bytes(
            self.inner,
            &mut buf_ptr as *mut *mut c_void,
            &mut size as *mut c_int,
        );
        std::slice::from_raw_parts_mut(buf_ptr as *mut u8, size as usize)
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        // Panic if zero_copy_buf_advance() failed.
        assert_eq!(1, zero_copy_buf_mut_advance(self.inner, cnt as c_int));
    }
}

extern "C" {
    pub fn zero_copy_buf_new(iobuf: *mut BrpcIOBuf) -> *mut BrpcZeroCopyBuf;
    pub fn zero_copy_buf_mut_new(iobuf: *mut BrpcIOBuf) -> *mut BrpcZeroCopyBufMut;

    pub fn zero_copy_buf_remaining(zc: *mut BrpcZeroCopyBuf) -> c_ulonglong;
    pub fn zero_copy_buf_bytes(
        zc: *mut BrpcZeroCopyBuf,
        data: *mut *const c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn zero_copy_buf_advance(zc: *mut BrpcZeroCopyBuf, count: c_int) -> c_int;

    pub fn zero_copy_buf_mut_remaining(zc: *mut BrpcZeroCopyBufMut) -> c_ulonglong;
    pub fn zero_copy_buf_mut_bytes(
        zc: *mut BrpcZeroCopyBufMut,
        data: *mut *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn zero_copy_buf_mut_advance(zc: *mut BrpcZeroCopyBufMut, count: c_int) -> c_int;
}
