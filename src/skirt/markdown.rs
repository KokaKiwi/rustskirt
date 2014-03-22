use std::libc;
use std::cast;
use std::str;
use std::io::MemWriter;
use std::io::IoResult;
use std::ty::Unsafe;

use ffi;
use callbacks::Callbacks;
use buffer::Buffer;

pub static ExtNoIntraEmphasis: uint = ffi::MKDEXT_NO_INTRA_EMPHASIS as uint;
pub static ExtTables: uint = ffi::MKDEXT_TABLES as uint;
pub static ExtFencedCode: uint = ffi::MKDEXT_FENCED_CODE as uint;
pub static ExtAutolink: uint = ffi::MKDEXT_AUTOLINK as uint;
pub static ExtStrikethrough: uint = ffi::MKDEXT_STRIKETHROUGH as uint;
pub static ExtSpaceHeaders: uint = ffi::MKDEXT_SPACE_HEADERS as uint;
pub static ExtSuperscript: uint = ffi::MKDEXT_SUPERSCRIPT as uint;
pub static ExtLaxSpacing: uint = ffi::MKDEXT_LAX_SPACING as uint;

pub struct Markdown<'a> {
    priv md: *mut ffi::sd_markdown,
    priv cb: ~Unsafe<&'a mut Callbacks>,
}

impl<'a> Markdown<'a> {
    pub fn new(extensions: uint, max_nesting: u64, cb: &'a mut Callbacks) -> Markdown {
        let ucb = ~Unsafe::new(cb);
        let ptr = unsafe { cast::transmute(ucb.get()) };
        let md = unsafe { ffi::sd_markdown_new(extensions as libc::c_uint, max_nesting as libc::size_t, &ffi::callbacks::callbacks, ptr) };

        Markdown {
            md: md,
            cb: ucb,
        }
    }

    pub fn render(&self, w: &mut Writer, document: &[u8]) -> IoResult<()> {
        let buf = Buffer::new(64);

        unsafe {
            ffi::sd_markdown_render(buf.buf, document.as_ptr(), document.len() as libc::size_t, self.md);
        }

        try!(w.write(buf.data().as_slice()));

        Ok(())
    }

    pub fn render_u8(&self, document: &[u8]) -> IoResult<~[u8]> {
        let mut w = MemWriter::new();
        try!(self.render(&mut w, document));
        Ok(w.unwrap())
    }

    pub fn render_str(&self, document: &str) -> IoResult<~str> {
        let data = try!(self.render_u8(document.as_bytes()));
        Ok(str::from_utf8(data).unwrap().to_owned())
    }
}

#[unsafe_destructor]
impl<'a> Drop for Markdown<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::sd_markdown_free(self.md);
            self.md = ::std::ptr::mut_null();
        }
    }
}
