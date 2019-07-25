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

#[repr(C)]
#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum BrpcError {
    NOERROR = 0, // No error_code

    ENOSERVICE = 1001, // Service not found
    ENOMETHOD = 1002,  // Method not found
    EREQUEST = 1003,   // Bad Request
    ERPCAUTH = 1004,   // Unauthorized, can't be called EAUTH
    // directly which is defined in MACOSX
    ETOOMANYFAILS = 1005,     // Too many sub calls failed
    EPCHANFINISH = 1006,      // [Internal] ParallelChannel finished
    EBACKUPREQUEST = 1007,    // Sending backup request
    ERPCTIMEDOUT = 1008,      // RPC call is timed out
    EFAILEDSOCKET = 1009,     // Broken socket
    EHTTP = 1010,             // Bad http call
    EOVERCROWDED = 1011,      // The server is overcrowded
    ERTMPPUBLISHABLE = 1012,  // RtmpRetryingClientStream is publishable
    ERTMPCREATESTREAM = 1013, // createStream was rejected by the RTMP server
    EEOF = 1014,              // Got EOF
    EUNUSED = 1015,           // The socket was not needed
    ESSL = 1016,              // SSL related error
    EH2RUNOUTSTREAMS = 1017,  // The H2 socket was run out of streams
    EREJECT = 1018,           // The Request is rejected

    // Errno caused by server
    EINTERNAL = 2001, // Internal Server Error
    ERESPONSE = 2002, // Bad Response
    ELOGOFF = 2003,   // Server is stopping
    ELIMIT = 2004,    // Reached server's limit on resources
    ECLOSE = 2005,    // Close socket initiatively
    EITP = 2006,      // Failed Itp response

    // Errno caused by brpc-rs
    ESERIALIZE = 3001,   // Prost serialization error
    EDESERIALIZE = 3002, // Prost deserialization error
    EFFI = 3003,         // FFI error

    UNKNOWN = 0xffff, // Unknown error,
}

#[doc(hidden)]
impl From<i32> for BrpcError {
    fn from(e: i32) -> BrpcError {
        match e {
            0 => BrpcError::NOERROR,
            1001 => BrpcError::ENOSERVICE,
            1002 => BrpcError::ENOMETHOD,
            1003 => BrpcError::EREQUEST,
            1004 => BrpcError::ERPCAUTH,
            1005 => BrpcError::ETOOMANYFAILS,
            1006 => BrpcError::EPCHANFINISH,
            1007 => BrpcError::EBACKUPREQUEST,
            1008 => BrpcError::ERPCTIMEDOUT,
            1009 => BrpcError::EFAILEDSOCKET,
            1010 => BrpcError::EHTTP,
            1011 => BrpcError::EOVERCROWDED,
            1012 => BrpcError::ERTMPPUBLISHABLE,
            1013 => BrpcError::ERTMPCREATESTREAM,
            1014 => BrpcError::EEOF,
            1015 => BrpcError::EUNUSED,
            1016 => BrpcError::ESSL,
            1017 => BrpcError::EH2RUNOUTSTREAMS,
            1018 => BrpcError::EREJECT,

            2001 => BrpcError::EINTERNAL,
            2002 => BrpcError::ERESPONSE,
            2003 => BrpcError::ELOGOFF,
            2004 => BrpcError::ELIMIT,
            2005 => BrpcError::ECLOSE,
            2006 => BrpcError::EITP,

            3001 => BrpcError::ESERIALIZE,
            3002 => BrpcError::EDESERIALIZE,
            3003 => BrpcError::EFFI,

            _ => BrpcError::UNKNOWN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_i32() {
        assert_eq!(BrpcError::from(0), BrpcError::NOERROR);
        assert_eq!(BrpcError::from(1001), BrpcError::ENOSERVICE);
        assert_eq!(BrpcError::from(1002), BrpcError::ENOMETHOD);
        assert_eq!(BrpcError::from(1003), BrpcError::EREQUEST);
        assert_eq!(BrpcError::from(1004), BrpcError::ERPCAUTH);
        assert_eq!(BrpcError::from(1005), BrpcError::ETOOMANYFAILS);
        assert_eq!(BrpcError::from(1006), BrpcError::EPCHANFINISH);
        assert_eq!(BrpcError::from(1007), BrpcError::EBACKUPREQUEST);
        assert_eq!(BrpcError::from(1008), BrpcError::ERPCTIMEDOUT);
        assert_eq!(BrpcError::from(1009), BrpcError::EFAILEDSOCKET);
        assert_eq!(BrpcError::from(1010), BrpcError::EHTTP);
        assert_eq!(BrpcError::from(1011), BrpcError::EOVERCROWDED);
        assert_eq!(BrpcError::from(1012), BrpcError::ERTMPPUBLISHABLE);
        assert_eq!(BrpcError::from(1013), BrpcError::ERTMPCREATESTREAM);
        assert_eq!(BrpcError::from(1014), BrpcError::EEOF);
        assert_eq!(BrpcError::from(1015), BrpcError::EUNUSED);
        assert_eq!(BrpcError::from(1016), BrpcError::ESSL);
        assert_eq!(BrpcError::from(1017), BrpcError::EH2RUNOUTSTREAMS);
        assert_eq!(BrpcError::from(1018), BrpcError::EREJECT);
        assert_eq!(BrpcError::from(2001), BrpcError::EINTERNAL);
        assert_eq!(BrpcError::from(2002), BrpcError::ERESPONSE);
        assert_eq!(BrpcError::from(2003), BrpcError::ELOGOFF);
        assert_eq!(BrpcError::from(2004), BrpcError::ELIMIT);
        assert_eq!(BrpcError::from(2005), BrpcError::ECLOSE);
        assert_eq!(BrpcError::from(2006), BrpcError::EITP);
        assert_eq!(BrpcError::from(3001), BrpcError::ESERIALIZE);
        assert_eq!(BrpcError::from(3002), BrpcError::EDESERIALIZE);
        assert_eq!(BrpcError::from(3003), BrpcError::EFFI);
        assert_eq!(BrpcError::from(5678), BrpcError::UNKNOWN);
    }
}
