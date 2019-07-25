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

#include <google/protobuf/compiler/code_generator.h>
#include <google/protobuf/compiler/command_line_interface.h>
#include <google/protobuf/compiler/plugin.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/io/printer.h>
#include <google/protobuf/io/zero_copy_stream.h>

inline bool HasSuffixString(const std::string &str, const std::string &suffix) {
  return str.size() >= suffix.size() &&
         str.compare(str.size() - suffix.size(), suffix.size(), suffix) == 0;
}

inline std::string StripSuffixString(const std::string &str,
                                     const std::string &suffix) {
  if (HasSuffixString(str, suffix)) {
    return str.substr(0, str.size() - suffix.size());
  } else {
    return str;
  }
}

inline std::string StripBeforeSlashFromRight(const std::string &str) {
  size_t pos = str.rfind('/', str.length());
  if (pos != std::string::npos) {
    return str.substr(pos + 1, str.size() - 1);
  }
  return str;
}

namespace brpc_rs {
class BrpcToProtobuf : public google::protobuf::compiler::CodeGenerator {
public:
  bool Generate(const google::protobuf::FileDescriptor *file,
                const std::string &parameter,
                google::protobuf::compiler::GeneratorContext *,
                std::string *error) const;
};

bool BrpcToProtobuf::Generate(const google::protobuf::FileDescriptor *file,
                              const std::string &parameter,
                              google::protobuf::compiler::GeneratorContext *ctx,
                              std::string *error) const {

  std::string base_name = StripSuffixString(file->name(), ".proto");
  std::string proto_file_name = file->name();
  std::string cc_file_name = base_name + ".brpc.cc";
  std::string rust_file_name = file->package() + ".brpc.rs";
  std::string include_file_name =
      StripBeforeSlashFromRight(base_name) + ".pb.h";

  /* Generate *.proto */

  google::protobuf::io::ZeroCopyOutputStream *proto_file =
      ctx->Open(proto_file_name);
  google::protobuf::io::Printer proto_printer(proto_file, '$');
  proto_printer.Print("syntax=\"proto2\";\n");
  proto_printer.Print("package $package_name$;\n\n", "package_name",
                      file->package());

  proto_printer.Print("option cc_generic_services = true;\n\n"
                      "message HttpRequest {};\n"
                      "message HttpResponse {};\n\n");

  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);
    proto_printer.Print("service $service_name$ {\n", "service_name",
                        service->name());
    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);
      proto_printer.Print(
          "  rpc $method_name$(HttpRequest) returns (HttpResponse);\n",
          "method_name", method->name());
    }
    proto_printer.Print("}\n");
  }

  /* Generate *.brpc.cc */

  google::protobuf::io::ZeroCopyOutputStream *ffi_file =
      ctx->Open(cc_file_name);
  google::protobuf::io::Printer cpp_printer(ffi_file, '$');
  cpp_printer.Print("#include \"$header_name$\"\n", "header_name",
                    include_file_name);
  cpp_printer.Print("#include <brpc/server.h>\n"
                    "#include <brpc/channel.h>\n"
                    "#include <butil/logging.h>\n"
                    "#include <butil/iobuf.h>\n\n"
                    "#include <brpc/restful.h>\n\n");

  cpp_printer.Print(
      "namespace butil {\n"
      "class ZeroCopyBuf {\n"
      "public:\n"
      "  explicit ZeroCopyBuf(const IOBuf &buf)\n"
      "  : _block_start(NULL), _block_end(NULL), _total_len(0), _buf(&buf),\n"
      "  _stream(buf) {\n"
      "    char *block_end = NULL;\n"
      "    int block_len;\n"
      "    if (_block_start == NULL) {\n"
      "    const void *start_ptr = NULL;\n"
      "    _stream.Next(reinterpret_cast<const void **>(&start_ptr), "
      "&block_len);\n"
      "    _block_start = (char *)start_ptr;\n"
      "    _block_end = _block_start + block_len;\n"
      "  }\n"
      "}\n"
      "  uint64_t Remaining() const;\n"
      "  bool Bytes(const void **data, int *size);\n"
      "  bool Advance(int count);\n"
      "\n"
      "private:\n"
      "  char *_block_start;\n"
      "  char *_block_end;\n"
      "  uint64_t _total_len;\n"
      "  const IOBuf *_buf;\n"
      "  IOBufAsZeroCopyInputStream _stream;\n"
      "};\n"
      "\n"
      "class ZeroCopyBufMut {\n"
      "public:\n"
      "  explicit ZeroCopyBufMut(IOBuf &buf)\n"
      "      : _block_start(NULL), _block_end(NULL), _total_len(0), "
      "_stream(&buf) {\n"
      "    char *block_end = NULL;\n"
      "    int block_len;\n"
      "    if (_block_start == NULL) {\n"
      "      _stream.Next(reinterpret_cast<void **>(&_block_start), "
      "&block_len);\n"
      "      _block_end = _block_start + block_len;\n"
      "    }\n"
      "  }\n"
      "  uint64_t RemainingMut() const;\n"
      "  bool BytesMut(void **data, int *size);\n"
      "  bool AdvanceMut(size_t count);\n\n"
      "private:\n"
      "  char *_block_start;\n"
      "  char *_block_end;\n"
      "  uint64_t _total_len;\n"
      "  IOBufAsZeroCopyOutputStream _stream;\n"
      "};\n"
      "\n"
      "} // namespace butil\n\n\n");

  // typedefs
  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);
    cpp_printer.Print("typedef void *brpc_$service_name$_service_t;\n"
                      "typedef void *brpc_$service_name$_service_handler_t;\n"
                      "typedef $package_name$::$service_name$_Stub "
                      "*brpc_$service_name$_stub_t;\n",
                      "service_name", service->name(), "package_name",
                      file->package());
  }

  cpp_printer.Print("\n\n");

  // namespace $package_name$
  cpp_printer.Print("namespace $package_name$ {\n\n", "package_name",
                    file->package());
  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);
    cpp_printer.Print("class $service_name$Impl: public $service_name$ {\n"
                      "public:\n"
                      "  $service_name$Impl() {\n",
                      "service_name", service->name());
    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);
      cpp_printer.Print("    $method_name$_trampoline = NULL;\n"
                        "    $method_name$_closure_ptr = NULL;\n",
                        "method_name", method->name());
    }
    cpp_printer.Print("  };\n"
                      "  virtual ~$service_name$Impl(){};\n",
                      "service_name", service->name());

    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);
      cpp_printer.Print(
          "  void $method_name$(google::protobuf::RpcController *cntl_base,\n"
          "                     const HttpRequest *,\n"
          "                     HttpResponse *, \n"
          "                     google::protobuf::Closure *done) {\n"
          "    brpc::ClosureGuard done_guard(done);\n"
          "    brpc::Controller *cntl = \n"
          "        static_cast<brpc::Controller *>(cntl_base);\n"
          "    cntl->http_response()\n"
          "        .set_content_type(\"application/octet-stream\");\n"
          "    butil::IOBuf &request_buf = cntl->request_attachment();\n"
          "    butil::IOBuf &response_buf = cntl->response_attachment();\n"
          "    butil::ZeroCopyBuf zc_request(request_buf);\n"
          "    butil::ZeroCopyBufMut zc_response(response_buf);\n"
          "    if (0 != $method_name$_trampoline(\n"
          "               $method_name$_closure_ptr,\n"
          "               zc_request,\n"
          "               zc_response)) {\n"
          "      cntl->SetFailed(brpc::EINTERNAL, \"brpc-rs controller "
          "failed\");\n"
          "    }\n"
          "  }\n"
          "\n"
          "  int (*$method_name$_trampoline)(\n"
          "                void *,\n"
          "                butil::ZeroCopyBuf &,\n"
          "                butil::ZeroCopyBufMut &);\n"
          "  void *$method_name$_closure_ptr;\n\n",
          "method_name", method->name());
    }
    cpp_printer.Print("};\n"); // Service class ends
  }

  cpp_printer.Print("} // namespace $package_name$\n\n", "package_name",
                    file->package());

  cpp_printer.Print("extern \"C\" {\n");

  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);
    cpp_printer.Print(
        "brpc_$service_name$_service_t brpc_$service_name$_new() {\n"
        "  return new $package_name$::$service_name$Impl;\n"
        "}\n"
        "\n"
        "void brpc_$service_name$_destroy(\n"
        "    brpc_$service_name$_service_t service\n"
        ") {\n"
        "  $package_name$::$service_name$Impl *service_ptr = \n"
        "    static_cast<$package_name$::$service_name$Impl *>(service);\n"
        "  delete service_ptr;\n"
        "}\n"
        "brpc_$service_name$_stub_t brpc_$service_name$Stub_with_channel(\n"
        "    brpc::Channel *ch"
        ") {\n"
        "  return new $package_name$::$service_name$_Stub(ch);\n"
        "}\n"
        "void brpc_$service_name$Stub_destroy(\n"
        "    brpc_$service_name$_stub_t stub\n"
        ") {\n"
        "  $package_name$::$service_name$_Stub *stub_ptr = \n"
        "    static_cast<$package_name$::$service_name$_Stub *>(stub);\n"
        "  delete stub_ptr;\n"
        "}\n"
        "\n",
        "package_name", file->package(), "service_name", service->name());

    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);

      cpp_printer.Print(
          "void brpc_$service_name$_$method_name$_set_handler(\n"
          "  brpc_$service_name$_service_t service,\n"
          "  void *rust_closure_ptr,\n"
          "  int (*trampoline)(void *, butil::ZeroCopyBuf &, \n"
          "                     butil::ZeroCopyBufMut &))\n"
          "{\n"
          "  $package_name$::$service_name$Impl *service_ptr = \n"
          "    static_cast<$package_name$::$service_name$Impl *>(service);\n"
          "  service_ptr->$method_name$_trampoline = trampoline;\n"
          "  service_ptr->$method_name$_closure_ptr = rust_closure_ptr;\n"
          "}\n",
          "method_name", method->name(), "service_name", service->name(),
          "package_name", file->package());

      cpp_printer.Print(
          "void brpc_$service_name$Stub_$method_name$(\n"
          "  brpc_$service_name$_stub_t stub,\n"
          "  brpc::Controller *cntl) {\n"
          "  $package_name$::$service_name$_Stub *stub_ptr = \n"
          "    static_cast<$package_name$::$service_name$_Stub *>(stub);\n"
          "  stub_ptr->$method_name$(cntl, NULL, NULL, NULL);\n"
          "}\n",
          "method_name", method->name(), "service_name", service->name(),
          "package_name", file->package());
    }
  }

  cpp_printer.Print("}\n"); // extern "C"

  /* Generate [proto_name].rs */
  google::protobuf::io::ZeroCopyOutputStream *rust_file =
      ctx->Open(rust_file_name);
  google::protobuf::io::Printer rs_printer(rust_file, '$');

  rs_printer.Print(
      "use std::os::raw::{c_int, c_void};\n\n"
      "use brpc_rs::{BrpcError, BrpcResult, Channel, Controller, Service};\n"
      "use brpc_rs::internal::ffi::{BrpcChannel, BrpcController};\n"
      "use brpc_rs::internal::zero_copy::{ZeroCopyBuf, ZeroCopyBufMut};\n"
      "use brpc_rs::internal::zero_copy::{BrpcZeroCopyBuf, "
      "BrpcZeroCopyBufMut};\n"
      "use prost::Message; // Trait\n\n");

  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);

    rs_printer.Print(
        "pub enum Brpc$service_name$ {}\n"
        "pub enum Brpc$service_name$Stub {}\n"
        "pub struct $service_name$ { inner: *mut Brpc$service_name$ }\n"
        "pub struct $service_name$Stub { inner: *mut Brpc$service_name$Stub }\n"
        "\n"
        "impl Service for $service_name$ {\n"
        "    fn get_service_ptr(&self) -> *mut c_void {\n"
        "        self.inner as *mut c_void\n"
        "    }\n"
        "}\n"
        "\n"
        "impl Default for $service_name$ {\n"
        "    fn default() -> Self {\n"
        "        Self::new()\n"
        "    }\n"
        "}\n"
        "\n"
        "impl Drop for $service_name$ {\n"
        "    fn drop(&mut self) {\n"
        "        unsafe {\n"
        "            brpc_$service_name$_destroy(self.inner);\n"
        "        }\n"
        "    }\n"
        "}\n"
        "\n"
        "impl $service_name$ {\n"
        "    pub fn new() -> $service_name$ {\n"
        "        $service_name$ { inner: unsafe { brpc_$service_name$_new() } "
        "}\n"
        "    }\n"
        "\n\n",
        "service_name", service->name());

    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);

      const google::protobuf::Descriptor *input = method->input_type();
      const google::protobuf::Descriptor *output = method->output_type();

      rs_printer.Print("    pub fn set_$method_name$_handler<F>("
                       "&mut self, rust_fn: &mut F)\n",
                       "method_name", method->name());
      rs_printer.Print(
          "    where\n"
          "        F: FnMut(&$input$, &mut $output$) -> "
          "BrpcResult<()> + Send + Sync + 'static {\n"
          "        unsafe extern \"C\" fn trampoline<F>(\n"
          "            data: *mut c_void,\n"
          "            zc_req: *mut c_void,  // function parameter 1\n"
          "            zc_resp: *mut c_void, // function parameter 2\n"
          "        ) -> c_int\n"
          "        where F: FnMut(&$input$, &mut $output$) -> "
          "BrpcResult<()> + Send + Sync + 'static {\n"
          "            let buf = ZeroCopyBuf::from_raw_ptr(\n"
          "                zc_req as *mut BrpcZeroCopyBuf);\n"
          "            let mut buf_mut = ZeroCopyBufMut::from_raw_ptr(\n"
          "                zc_resp as *mut BrpcZeroCopyBufMut);\n"
          "            let closure: &mut F = &mut *(data as *mut F);\n"
          "            let mut response = $output$::default();\n"
          "            let request = match "
          "$input$::decode_length_delimited(buf) {\n"
          "                Ok(r) => r,\n"
          "                Err(_e) => return -1, \n"
          "            };\n"
          "            match (*closure)(&request, &mut response) {\n"
          "                Ok(r) => r,\n"
          "                Err(_e) => return -1, \n"
          "            };\n"
          "            match response.encode_length_delimited(&mut buf_mut) {\n"
          "                Ok(r) => r,\n"
          "                Err(_e) => return -1, \n"
          "            };\n"
          "            0\n"
          "        }\n",
          "input", input->name(), "output", output->name());
      rs_printer.Print(
          "        let rust_fn_ptr = rust_fn as *mut F as *mut c_void;\n"
          "        unsafe { "
          "brpc_$service_name$_$method_name$_set_handler(self.inner, "
          "rust_fn_ptr, trampoline::<F>) };\n"
          "    }\n",
          "service_name", service->name(), "method_name", method->name());
    }
    rs_printer.Print("}\n\n");

    // ServiceStub functions
    rs_printer.Print(
        "impl Drop for $service_name$Stub {\n"
        "    fn drop(&mut self) {\n"
        "        unsafe {\n"
        "            brpc_$service_name$Stub_destroy(self.inner);\n"
        "        }\n"
        "    }\n"
        "}\n\n"
        "impl $service_name$Stub {\n"
        "    pub fn with_channel(ch: &Channel) -> $service_name$Stub {\n"
        "        $service_name$Stub { \n"
        "            inner: unsafe{ "
        "brpc_$service_name$Stub_with_channel(ch.inner) }\n"
        "        }\n"
        "    }\n"
        "\n\n",
        "service_name", service->name());

    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);
      const google::protobuf::Descriptor *input = method->input_type();
      const google::protobuf::Descriptor *output = method->output_type();

      rs_printer.Print("    pub fn $method_name$(&self, request: &$input$) -> "
                       "BrpcResult<$output$> {\n",
                       "method_name", method->name(), "input", input->name(),
                       "output", output->name());
      rs_printer.Print(
          "        let cntl = Controller::new();\n"
          "        let mut request_buf = unsafe { ZeroCopyBufMut::from_iobuf(\n"
          "            cntl.request_attachment()\n"
          "        ) };\n"
          "        request.encode_length_delimited(&mut "
          "request_buf).map_err(|_| BrpcError::ESERIALIZE)?;\n"
          "        unsafe { brpc_$service_name$Stub_$method_name$(self.inner, "
          "cntl.inner) };\n"
          "        if cntl.failed() { return Err(cntl.error()); }\n"
          "        let response_buf = unsafe { ZeroCopyBuf::from_iobuf(\n"
          "            cntl.response_attachment()\n"
          "        ) };\n"
          "        let response = "
          "$output$::decode_length_delimited(response_buf).map_err(|_| "
          "BrpcError::EDESERIALIZE)?;\n"
          "        Ok(response)\n"
          "    }\n",
          "service_name", service->name(), "method_name", method->name(),
          "output", output->name());
      rs_printer.Print("}\n\n");
    }
  }

  rs_printer.Print("type Trampoline = unsafe extern \"C\" fn(*mut c_void, *mut "
                   "c_void, *mut c_void) -> c_int;\n\n");

  rs_printer.Print("extern \"C\" {\n");
  for (int i = 0; i < file->service_count(); ++i) {
    const google::protobuf::ServiceDescriptor *service = file->service(i);

    rs_printer.Print(
        "fn brpc_$service_name$_new() -> *mut Brpc$service_name$;\n"
        "fn brpc_$service_name$_destroy(service: *mut Brpc$service_name$);\n"
        "fn brpc_$service_name$Stub_with_channel(ch: *mut BrpcChannel) -> *mut "
        "Brpc$service_name$Stub;\n"
        "fn brpc_$service_name$Stub_destroy(service: *mut "
        "Brpc$service_name$Stub);\n",
        "service_name", service->name());

    for (int j = 0; j < service->method_count(); ++j) {
      const google::protobuf::MethodDescriptor *method = service->method(j);
      rs_printer.Print("fn brpc_$service_name$_$method_name$_set_handler(\n"
                       "    service: *mut Brpc$service_name$,\n"
                       "    closure: *mut c_void,\n"
                       "    t: Trampoline\n"
                       ");\n"
                       "\n"
                       "fn brpc_$service_name$Stub_$method_name$(\n"
                       "    stub: *mut Brpc$service_name$Stub,\n"
                       "    cntl: *mut BrpcController\n"
                       ");\n"
                       "\n",
                       "service_name", service->name(), "method_name",
                       method->name());
    }
  }

  rs_printer.Print("}\n\n\n"); // extern "C"
  return true;
}

} // namespace brpc_rs

extern "C" {
int cpp_main(int argc, char *argv[]) {
  ::brpc_rs::BrpcToProtobuf brpc_generator;
  return google::protobuf::compiler::PluginMain(argc, argv, &brpc_generator);
}
}
