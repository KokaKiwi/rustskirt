use Autolink;

// Callbacks
#[allow(unused_variable)]
#[allow(unused_must_use)]
pub trait Callbacks {
    // block level callbacks
    fn blockcode(&mut self, w: &mut Writer, text: &str, lang: &str) {}
    fn blockquote(&mut self, w: &mut Writer, text: &str) {}
    fn blockhtml(&mut self, w: &mut Writer, text: &str) {}
    fn header(&mut self, w: &mut Writer, text: &str, level: int) {}
    fn hrule(&mut self, w: &mut Writer) {}
    fn list(&mut self, w: &mut Writer, text: &str, flags: int) {}
    fn listitem(&mut self, w: &mut Writer, text: &str, flags: int) {}
    fn paragraph(&mut self, w: &mut Writer, text: &str) {}
    fn table(&mut self, w: &mut Writer, header: &str, body: &str) {}
    fn table_row(&mut self, w: &mut Writer, text: &str) {}
    fn table_cell(&mut self, w: &mut Writer, text: &str, flags: int) {}

    // span level callbacks
    fn autolink(&mut self, w: &mut Writer, link: &str, _ty: Autolink) -> bool { false }
    fn codespan(&mut self, w: &mut Writer, text: &str) -> bool { false }
    fn double_emphasis(&mut self, w: &mut Writer, text: &str) -> bool { false }
    fn emphasis(&mut self, w: &mut Writer, text: &str) -> bool { false }
    fn image(&mut self, w: &mut Writer, link: &str, title: &str, alt: &str) -> bool { false }
    fn linebreak(&mut self, w: &mut Writer) -> bool { false }
    fn link(&mut self, w: &mut Writer, link: &str, title: &str, content: &str) -> bool { false }
    fn raw_html_tag(&mut self, w: &mut Writer, tag: &str) -> bool { false }
    fn triple_emphasis(&mut self, w: &mut Writer, text: &str) -> bool { false }
    fn strikethrough(&mut self, w: &mut Writer, text: &str) -> bool { false }
    fn superscript(&mut self, w: &mut Writer, text: &str) -> bool { false }

    // low level callbacks
    fn entity(&mut self, w: &mut Writer, entity: &str) {
        w.write_str(entity);
    }
    fn normal_text(&mut self, w: &mut Writer, text: &str) {
        w.write_str(text);
    }

    // header and footer
    fn doc_header(&mut self, w: &mut Writer) {}
    fn doc_footer(&mut self, w: &mut Writer) {}
}
