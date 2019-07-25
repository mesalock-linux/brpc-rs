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

use std::os::raw::{c_char, c_int, c_void};

pub enum BrpcServer {} // brpc::Server
pub enum BrpcServerOptions {} // brpc::ServerOptions

pub enum BrpcChannel {} // brpc::Channel
pub enum BrpcChannelOptions {} // brpc::ChannelOptions

pub enum BrpcController {} // brpc::Controller
pub enum BrpcIOBuf {} // butil::IOBuf

#[allow(dead_code)]
extern "C" {
    pub fn brpc_is_asked_to_quit() -> c_int;
    pub fn brpc_server_new() -> *mut BrpcServer;
    pub fn brpc_server_destroy(server: *mut BrpcServer);
    pub fn brpc_server_add_service(
        server: *mut BrpcServer,
        service: *mut c_void,
        ownership: c_int,
    ) -> c_int;
    pub fn brpc_server_start(
        server: *mut BrpcServer,
        port: c_int,
        opt: *const BrpcServerOptions,
    ) -> c_int;
    pub fn brpc_server_run_until_asked_to_quit(server: *mut BrpcServer);

    pub fn brpc_server_options_new() -> *mut BrpcServerOptions;
    pub fn brpc_server_options_destroy(server_options: *mut BrpcServerOptions);
    pub fn brpc_server_options_set_idle_timeout_ms(
        server_options: *mut BrpcServerOptions,
        timeout: c_int,
    );

    pub fn brpc_channel_new() -> *mut BrpcChannel;
    pub fn brpc_channel_destroy(channel: *mut BrpcChannel);
    pub fn brpc_channel_init(
        channel: *mut BrpcChannel,
        server_addr_and_port: *const c_char,
        options: *const BrpcChannelOptions,
    ) -> c_int;

    pub fn brpc_channel_options_new() -> *mut BrpcChannelOptions;
    pub fn brpc_channel_options_destroy(channel_options: *mut BrpcChannelOptions);

    pub fn brpc_channel_options_set_timeout_ms(
        channel_options: *mut BrpcChannelOptions,
        timeout: c_int,
    );
    pub fn brpc_channel_options_set_max_retry(
        channel_options: *mut BrpcChannelOptions,
        max_retry: c_int,
    );
    pub fn brpc_controller_new() -> *mut BrpcController;
    pub fn brpc_controller_destroy(cntl: *mut BrpcController);
    pub fn brpc_controller_failed(cntl: *mut BrpcController) -> c_int;
    pub fn brpc_controller_error_code(cntl: *mut BrpcController) -> c_int;
    pub fn brpc_controller_set_failed(cntl: *mut BrpcController, code: c_int);
    pub fn brpc_controller_get_request_attachment(cntl: *mut BrpcController) -> *mut BrpcIOBuf;
    pub fn brpc_controller_get_response_attachment(cntl: *mut BrpcController) -> *mut BrpcIOBuf;
}
