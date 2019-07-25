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

#include <brpc/channel.h>
#include <brpc/server.h>

extern "C" {
size_t iobuf_size(butil::IOBuf &buf) { return buf.size(); }
bool brpc_is_asked_to_quit(void) { return brpc::IsAskedToQuit(); }
}

// brpc::Server
extern "C" {
brpc::Server *brpc_server_new() { return new brpc::Server; }

void brpc_server_destroy(brpc::Server *server) { delete server; }

int brpc_server_add_service(brpc::Server *server,
                            ::google::protobuf::Service *service,
                            brpc::ServiceOwnership ownership) {
  return server->AddService(service, ownership);
}

int brpc_server_start(brpc::Server *server, int port,
                      brpc::ServerOptions *options) {
  return server->Start(port, options);
}

void brpc_server_run_until_asked_to_quit(brpc::Server *server) {
  return server->RunUntilAskedToQuit();
}

// brpc::ServerOptions
brpc::ServerOptions *brpc_server_options_new() {
  return new brpc::ServerOptions;
}

void brpc_server_options_destroy(brpc::ServerOptions *options) {
  delete options;
}

void brpc_server_options_set_idle_timeout_ms(brpc::ServerOptions *options,
                                             int timeout) {
  options->idle_timeout_sec = timeout;
}
} // extern "C" brpc::Server

// brpc::Channel
extern "C" {
brpc::Channel *brpc_channel_new() { return new brpc::Channel; }

void brpc_channel_destroy(brpc::Channel *channel) { delete channel; }

int brpc_channel_init(brpc::Channel *channel, const char *server_addr_and_port,
                      const brpc::ChannelOptions *options) {
  return channel->Init(server_addr_and_port, options);
}

// brpc::ChannelOptions
brpc::ChannelOptions *brpc_channel_options_new() {
  brpc::ChannelOptions *ptr = new brpc::ChannelOptions;
  ptr->protocol = "http";
  return ptr;
}

void brpc_channel_options_destroy(brpc::ChannelOptions *options) {
  delete options;
}

void brpc_channel_options_set_timeout_ms(brpc::ChannelOptions *options,
                                         int timeout) {
  options->timeout_ms = timeout;
}

void brpc_channel_options_set_max_retry(brpc::ChannelOptions *options,
                                        int max_retry) {
  options->max_retry = max_retry;
}

} // extern "C" brpc::Channel

// brpc::Controller
extern "C" {
brpc::Controller *brpc_controller_new() { return new brpc::Controller; }

void brpc_controller_destroy(brpc::Controller *cntl) { delete cntl; }

bool brpc_controller_failed(brpc::Controller *cntl) { return cntl->Failed(); }

int brpc_controller_error_code(brpc::Controller *cntl) {
  return cntl->ErrorCode();
}

void brpc_controller_set_failed(brpc::Controller *cntl, int code) {
  cntl->SetFailed(code, "brpc-rs controller failed");
}

butil::IOBuf &brpc_controller_get_request_attachment(brpc::Controller *cntl) {
  return cntl->request_attachment();
}

butil::IOBuf &brpc_controller_get_response_attachment(brpc::Controller *cntl) {
  return cntl->response_attachment();
}
}