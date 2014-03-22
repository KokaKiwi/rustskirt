use std::libc::*;

pub mod callbacks;

pub enum mkd_autolink {
    MKDA_NOT_AUTOLINK,
    MKDA_NORMAL,
    MKDA_EMAIL,
}

pub static MKD_TABLE_ALIGN_L: c_int = 1;
pub static MKD_TABLE_ALIGN_R: c_int = 2;
pub static MKD_TABLE_ALIGN_CENTER: c_int = 3;
pub static MKD_TABLE_ALIGNMASK: c_int = 3;
pub static MKD_TABLE_HEADER: c_int = 4;

pub static MKDEXT_NO_INTRA_EMPHASIS: c_int = (1 << 0);
pub static MKDEXT_TABLES: c_int = (1 << 1);
pub static MKDEXT_FENCED_CODE: c_int = (1 << 2);
pub static MKDEXT_AUTOLINK: c_int = (1 << 3);
pub static MKDEXT_STRIKETHROUGH: c_int = (1 << 4);
pub static MKDEXT_SPACE_HEADERS: c_int = (1 << 6);
pub static MKDEXT_SUPERSCRIPT: c_int = (1 << 7);
pub static MKDEXT_LAX_SPACING: c_int = (1 << 8);

pub static BUF_OK: c_int = 0;
pub static BUF_ENOMEM: c_int = -1;

pub struct buf {
    data: *uint8_t,
    size: size_t,
    asize: size_t,
    unit: size_t,
}

pub type ob_t = *mut buf;
pub type opaque_t = *mut c_void;

pub struct sd_callbacks {
    // block level callbacks - NULL skips the block
    blockcode: Option<extern "C" fn(ob: ob_t, text: *buf, lang: *buf, opaque: opaque_t)>,
    blockquote: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t)>,
    blockhtml: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t)>,
    header: Option<extern "C" fn(ob: ob_t, text: *buf, level: c_int, opaque: opaque_t)>,
    hrule: Option<extern "C" fn(ob: ob_t, opaque: opaque_t)>,
    list: Option<extern "C" fn(ob: ob_t, text: *buf, flags: c_int, opaque: opaque_t)>,
    listitem: Option<extern "C" fn(ob: ob_t, text: *buf, flags: c_int, opaque: opaque_t)>,
    paragraph: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t)>,
    table: Option<extern "C" fn(ob: ob_t, header: *buf, body: *buf, opaque: opaque_t)>,
    table_row: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t)>,
    table_cell: Option<extern "C" fn(ob: ob_t, text: *buf, flags: c_int, opaque: opaque_t)>,

    // span level callbacks - NULL or return 0 prints the span verbatim
    autolink: Option<extern "C" fn(ob: ob_t, link: *buf, ty: mkd_autolink, opaque: opaque_t) -> c_int>,
    codespan: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,
    double_emphasis: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,
    emphasis: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,
    image: Option<extern "C" fn(ob: ob_t, link: *buf, title: *buf, alt: *buf, opaque: opaque_t) -> c_int>,
    linebreak: Option<extern "C" fn(ob: ob_t, opaque: opaque_t) -> c_int>,
    link: Option<extern "C" fn(ob: ob_t, link: *buf, title: *buf, content: *buf, opaque: opaque_t) -> c_int>,
    raw_html_tag: Option<extern "C" fn(ob: ob_t, tag: *buf, opaque: opaque_t) -> c_int>,
    triple_emphasis: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,
    strikethrough: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,
    superscript: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t) -> c_int>,

    // low level callbacks - NULL copies input directly into the output
    entity: Option<extern "C" fn(ob: ob_t, entity: *buf, opaque: opaque_t)>,
    normal_text: Option<extern "C" fn(ob: ob_t, text: *buf, opaque: opaque_t)>,

    // header and footer
    doc_header: Option<extern "C" fn(ob: ob_t, opaque: opaque_t)>,
    doc_footer: Option<extern "C" fn(ob: ob_t, opaque: opaque_t)>,
}

pub struct sd_markdown;

#[link(name = "sundown", kind = "static")]
extern "C" {
    // markdown.h
    pub fn sd_markdown_new(extensions: c_uint, max_nesting: size_t, callbacks: *sd_callbacks, opaque: opaque_t) -> *mut sd_markdown;
    pub fn sd_markdown_render(ob: ob_t, document: *uint8_t, doc_size: size_t, md: *mut sd_markdown);
    pub fn sd_markdown_free(md: *mut sd_markdown);
    pub fn sd_version(major: *c_int, minor: *c_int, revision: *c_int);

    // buffer.h
    pub fn bufnew(size: size_t) -> *mut buf;
    pub fn bufcstr(buf: *mut buf) -> *c_char;
    pub fn bufprefix(buf: *buf, prefix: *c_char) -> c_int;
    pub fn bufput(buf: *mut buf, data: *c_void, size: size_t);
    pub fn bufputs(buf: *mut buf, s: *c_char);
    pub fn bufputc(buf: *mut buf, c: c_int);
    pub fn bufrelease(buf: *mut buf);
    pub fn bufreset(buf: *mut buf);
    pub fn bufslurp(buf: *mut buf, size: size_t);
}
