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

use brpc_rs::{Channel, ChannelOptions};

pub mod echo {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
    include!(concat!(env!("OUT_DIR"), "/example.brpc.rs"));
}

fn main() {
    let mut options = ChannelOptions::new();
    options.set_timeout_ms(100);
    let addr = "127.0.0.1:50000".parse().expect("Invalid socket address");
    let ch = Channel::with_options(&addr, &options);
    let client = echo::EchoServiceStub::with_channel(&ch);
    let request = echo::EchoRequest {
        message: "hello".to_owned(),
    };
    match client.echo(&request) {
        Ok(r) => println!("Response: {:?}", r),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
