use std::libc;
use std::slice;
use std::str;

use ffi;

pub struct Buffer {
    buf: ffi::ob_t,
    priv owned: bool,
}

impl Buffer {
    pub fn new(size: u64) -> Buffer {
        let buf = unsafe { ffi::bufnew(size as libc::size_t) };
        Buffer {
            buf: buf,
            owned: true,
        }
    }

    pub fn new_from_existing(buf: ffi::ob_t) -> Buffer {
        Buffer {
            buf: buf,
            owned: false,
        }
    }

    pub fn data(&self) -> Vec<u8> {
        if self.buf.is_null() {
            Vec::new()
        } else {
            unsafe {
                Vec::from_slice(slice::from_buf((*self.buf).data, (*self.buf).size as uint))
            }
        }
    }

    pub fn as_str(&self) -> ~str {
        str::from_utf8(self.data().as_slice()).unwrap().to_owned()
    }
}

impl Writer for Buffer {
    fn write(&mut self, buf: &[u8]) -> ::std::io::IoResult<()> {
        unsafe {
            ffi::bufput(self.buf, buf.as_ptr() as *libc::c_void, buf.len() as libc::size_t);
        }
        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::bufrelease(self.buf);
                self.buf = ::std::ptr::mut_null();
            }
        }
    }
}
