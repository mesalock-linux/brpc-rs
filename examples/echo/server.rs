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

use brpc_rs::{Server, ServerOptions, ServiceOwnership};

pub mod echo {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
    include!(concat!(env!("OUT_DIR"), "/example.brpc.rs"));
}

fn main() {
    let mut service = echo::EchoService::new();
    service.set_echo_handler(&mut move |request, mut response| {
        response.message = request.message.clone();
        Ok(())
    });

    let mut server = Server::new();
    let mut options = ServerOptions::new();
    options.set_idle_timeout_ms(1000);
    server
        .add_service(&service, ServiceOwnership::ServerDoesntOwnService)
        .expect("Failed to add service");
    server
        .start(50000, &options)
        .expect("Failed to start service");
    server.run(); // Run until CTRL-C
}
