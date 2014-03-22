use std::cast;
use std::libc;

use ffi;
use Autolink;
use buffer::Buffer;
use callbacks::Callbacks;

// block level callbacks
extern "C" fn cb_blockcode(ob: ffi::ob_t, text: *ffi::buf, lang: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let lang = Buffer::new_from_existing(lang as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).blockcode(&mut buf as &mut Writer, text, lang);
    }
}

extern "C" fn cb_blockquote(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).blockquote(&mut buf as &mut Writer, text);
    }
}

extern "C" fn cb_blockhtml(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).blockhtml(&mut buf as &mut Writer, text);
    }
}

extern "C" fn cb_header(ob: ffi::ob_t, text: *ffi::buf, level: libc::c_int, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let level = level as int;
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).header(&mut buf as &mut Writer, text, level);
    }
}

extern "C" fn cb_hrule(ob: ffi::ob_t, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).hrule(&mut buf as &mut Writer);
    }
}

extern "C" fn cb_list(ob: ffi::ob_t, text: *ffi::buf, flags: libc::c_int, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).list(&mut buf as &mut Writer, text, flags as int);
    }
}

extern "C" fn cb_listitem(ob: ffi::ob_t, text: *ffi::buf, flags: libc::c_int, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).listitem(&mut buf as &mut Writer, text, flags as int);
    }
}

extern "C" fn cb_paragraph(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).paragraph(&mut buf as &mut Writer, text);
    }
}

extern "C" fn cb_table(ob: ffi::ob_t, body: *ffi::buf, header: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let body = Buffer::new_from_existing(body as ffi::ob_t).as_str();
    let header = Buffer::new_from_existing(header as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).table(&mut buf as &mut Writer, body, header);
    }
}

extern "C" fn cb_table_row(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).table_row(&mut buf as &mut Writer, text);
    }
}

extern "C" fn cb_table_cell(ob: ffi::ob_t, text: *ffi::buf, flags: libc::c_int, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).table_cell(&mut buf as &mut Writer, text, flags as int);
    }
}

// span level callbacks
extern "C" fn cb_autolink(ob: ffi::ob_t, link: *ffi::buf, ty: ffi::mkd_autolink, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let link = Buffer::new_from_existing(link as ffi::ob_t).as_str();
    let ty = Autolink::from_ffi(ty);
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).autolink(&mut buf as &mut Writer, link, ty) as libc::c_int
    }
}

extern "C" fn cb_codespan(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).codespan(&mut buf as &mut Writer, text) as libc::c_int
    }
}

extern "C" fn cb_double_emphasis(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).double_emphasis(&mut buf as &mut Writer, text) as libc::c_int
    }
}

extern "C" fn cb_emphasis(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).emphasis(&mut buf as &mut Writer, text) as libc::c_int
    }
}

extern "C" fn cb_image(ob: ffi::ob_t, link: *ffi::buf, title: *ffi::buf, alt: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let link = Buffer::new_from_existing(link as ffi::ob_t).as_str();
    let title = Buffer::new_from_existing(title as ffi::ob_t).as_str();
    let alt = Buffer::new_from_existing(alt as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).image(&mut buf as &mut Writer, link, title, alt) as libc::c_int
    }
}

extern "C" fn cb_linebreak(ob: ffi::ob_t, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).linebreak(&mut buf as &mut Writer) as libc::c_int
    }
}

extern "C" fn cb_link(ob: ffi::ob_t, link: *ffi::buf, title: *ffi::buf, content: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let link = Buffer::new_from_existing(link as ffi::ob_t).as_str();
    let title = Buffer::new_from_existing(title as ffi::ob_t).as_str();
    let content = Buffer::new_from_existing(content as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).link(&mut buf as &mut Writer, link, title, content) as libc::c_int
    }
}

extern "C" fn cb_raw_html_tag(ob: ffi::ob_t, tag: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let tag = Buffer::new_from_existing(tag as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).raw_html_tag(&mut buf as &mut Writer, tag) as libc::c_int
    }
}

extern "C" fn cb_triple_emphasis(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).triple_emphasis(&mut buf as &mut Writer, text) as libc::c_int
    }
}

extern "C" fn cb_strikethrough(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).strikethrough(&mut buf as &mut Writer, text) as libc::c_int
    }
}

extern "C" fn cb_superscript(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) -> libc::c_int {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).superscript(&mut buf as &mut Writer, text) as libc::c_int
    }
}

// low level callbacks
extern "C" fn cb_entity(ob: ffi::ob_t, entity: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let entity = Buffer::new_from_existing(entity as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).entity(&mut buf as &mut Writer, entity);
    }
}

extern "C" fn cb_normal_text(ob: ffi::ob_t, text: *ffi::buf, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let text = Buffer::new_from_existing(text as ffi::ob_t).as_str();
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).normal_text(&mut buf as &mut Writer, text);
    }
}

// header and footer
extern "C" fn cb_doc_header(ob: ffi::ob_t, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).doc_header(&mut buf as &mut Writer);
    }
}

extern "C" fn cb_doc_footer(ob: ffi::ob_t, opaque: ffi::opaque_t) {
    let mut buf = Buffer::new_from_existing(ob);
    let cb: *&mut Callbacks = unsafe { cast::transmute(opaque) };

    unsafe {
        (*cb).doc_footer(&mut buf as &mut Writer);
    }
}

pub static callbacks: ffi::sd_callbacks = ffi::sd_callbacks {
    blockcode: Some(cb_blockcode),
    blockquote: Some(cb_blockquote),
    blockhtml: Some(cb_blockhtml),
    header: Some(cb_header),
    hrule: Some(cb_hrule),
    list: Some(cb_list),
    listitem: Some(cb_listitem),
    paragraph: Some(cb_paragraph),
    table: Some(cb_table),
    table_row: Some(cb_table_row),
    table_cell: Some(cb_table_cell),
    autolink: Some(cb_autolink),
    codespan: Some(cb_codespan),
    double_emphasis: Some(cb_double_emphasis),
    emphasis: Some(cb_emphasis),
    image: Some(cb_image),
    linebreak: Some(cb_linebreak),
    link: Some(cb_link),
    raw_html_tag: Some(cb_raw_html_tag),
    triple_emphasis: Some(cb_triple_emphasis),
    strikethrough: Some(cb_strikethrough),
    superscript: Some(cb_superscript),
    entity: Some(cb_entity),
    normal_text: Some(cb_normal_text),
    doc_header: Some(cb_doc_header),
    doc_footer: Some(cb_doc_footer),
};
