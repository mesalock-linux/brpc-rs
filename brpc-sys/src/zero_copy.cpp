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

#include <butil/iobuf.h>
#include <butil/logging.h>

namespace butil {
class ZeroCopyBuf {
public:
  explicit ZeroCopyBuf(const IOBuf &buf)
      : _block_start(NULL), _block_end(NULL), _total_len(0), _buf(&buf),
        _stream(buf) {
    char *block_end = NULL;
    int block_len;
    if (_block_start == NULL) {
      const void *start_ptr = NULL;
      _stream.Next(reinterpret_cast<const void **>(&start_ptr), &block_len);
      _block_start = (char *)start_ptr;
      _block_end = _block_start + block_len;
    }
  }
  uint64_t Remaining() const;
  bool Bytes(const void **data, int *size);
  bool Advance(int count);

private:
  char *_block_start;
  char *_block_end;
  uint64_t _total_len;
  const IOBuf *_buf;
  IOBufAsZeroCopyInputStream _stream;
};

uint64_t ZeroCopyBuf::Remaining() const { return _buf->length() - _total_len; }

bool ZeroCopyBuf::Bytes(const void **data, int *size) {
  *data = _block_start;
  *size = _block_end - _block_start;
  return true;
}

bool ZeroCopyBuf::Advance(int count) {
  if (count == 0) {
    return true;
  }
  size_t nc = 0;
  while (nc < count && Remaining() != 0) {
    const size_t block_size = _block_end - _block_start;
    const size_t to_skip = std::min(block_size, count - nc);
    _block_start += to_skip;
    _total_len += to_skip;
    nc += to_skip;
    if (_block_start == _block_end) {
      int block_len;
      _stream.Next(reinterpret_cast<const void **>(_block_start), &block_len);
      _block_end = _block_start + block_len;
    }
  }
  return true;
}

class ZeroCopyBufMut {
public:
  explicit ZeroCopyBufMut(IOBuf &buf)
      : _block_start(NULL), _block_end(NULL), _total_len(0), _buf(&buf),
        _stream(&buf) {
    char *block_end = NULL;
    int block_len;
    if (_block_start == NULL) {
      _stream.Next(reinterpret_cast<void **>(&_block_start), &block_len);
      _block_end = _block_start + block_len;
    }
  }
  uint64_t RemainingMut() const;
  bool BytesMut(void **data, int *size);
  bool AdvanceMut(size_t count);

private:
  char *_block_start;
  char *_block_end;
  uint64_t _total_len;
  IOBuf *_buf;
  IOBufAsZeroCopyOutputStream _stream;
};

uint64_t ZeroCopyBufMut::RemainingMut() const {
  return UINT64_MAX - _total_len;
}

bool ZeroCopyBufMut::BytesMut(void **data, int *size) {
  *data = _block_start;
  *size = _block_end - _block_start;
  return true;
}

bool ZeroCopyBufMut::AdvanceMut(size_t count) {
  if (count == 0) {
    return true;
  }
  if (count > RemainingMut()) {
    return false;
  }
  size_t nc = 0;
  while (nc < count && RemainingMut() != 0) {
    const size_t block_size = _block_end - _block_start;
    const size_t to_skip = std::min(block_size, count - nc);
    _block_start += to_skip;
    _total_len += to_skip;
    nc += to_skip;
    if (_block_start == _block_end) {
      int block_len;
      _stream.Next(reinterpret_cast<void **>(_block_start), &block_len);
      _block_end = _block_start + block_len;
    }
  }
  return true;
}

} // namespace butil

// ZeroCopyBuf and ZeroCopyBufMut
extern "C" {

butil::ZeroCopyBuf *zero_copy_buf_new(butil::IOBuf &iobuf) {
  return new butil::ZeroCopyBuf(iobuf);
}

butil::ZeroCopyBufMut *zero_copy_buf_mut_new(butil::IOBuf &iobuf) {
  return new butil::ZeroCopyBufMut(iobuf);
}

uint64_t zero_copy_buf_remaining(butil::ZeroCopyBuf *zc) {
  return zc->Remaining();
}

bool zero_copy_buf_bytes(butil::ZeroCopyBuf *zc, const void **data, int *size) {
  return zc->Bytes(data, size);
}

bool zero_copy_buf_advance(butil::ZeroCopyBuf *zc, int count) {
  return zc->Advance(count);
}

uint64_t zero_copy_buf_mut_remaining(butil::ZeroCopyBufMut *zc) {
  return zc->RemainingMut();
}

bool zero_copy_buf_mut_bytes(butil::ZeroCopyBufMut *zc, void **data,
                             int *size) {
  return zc->BytesMut(data, size);
}

bool zero_copy_buf_mut_advance(butil::ZeroCopyBufMut *zc, int count) {
  return zc->AdvanceMut(count);
}
}
