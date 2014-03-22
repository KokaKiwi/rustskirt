#[crate_id = "mddebug#0.1.0"];
#[crate_type = "bin"];
#[license = "MIT"];

#[allow(unused_must_use)];

extern crate skirt;

use std::io;

use skirt::md;
use skirt::Markdown;
use skirt::Callbacks;

struct DebugCallbacks;

impl Callbacks for DebugCallbacks {
    // block level callbacks
    fn blockcode(&mut self, _w: &mut Writer, text: &str, lang: &str) {
        println!("blockcode('{}', '{}')", text, lang);
    }

    fn blockquote(&mut self, _w: &mut Writer, text: &str) {
        println!("blockquote('{}')", text);
    }

    fn blockhtml(&mut self, _w: &mut Writer, text: &str) {
        println!("blockquote('{}')", text);
    }

    fn header(&mut self, _w: &mut Writer, text: &str, level: int) {
        println!("header('{}', {})", text, level);
    }

    fn hrule(&mut self, _w: &mut Writer) {
        println!("hrule()");
    }

    fn list(&mut self, _w: &mut Writer, text: &str, flags: int) {
        println!("list('{}', {})", text, flags);
    }

    fn listitem(&mut self, _w: &mut Writer, text: &str, flags: int) {
        println!("listitem('{}', {})", text, flags);
    }

    fn paragraph(&mut self, _w: &mut Writer, text: &str) {
        println!("paragraph('{}')", text);
    }

    fn table(&mut self, _w: &mut Writer, header: &str, body: &str) {
        println!("table('{}', '{}')", header, body);
    }

    fn table_row(&mut self, _w: &mut Writer, text: &str) {
        println!("table_row('{}')", text);
    }

    fn table_cell(&mut self, _w: &mut Writer, text: &str, flags: int) {
        println!("table_cell('{}', {})", text, flags);
    }

    // header and footer
    fn doc_header(&mut self, _w: &mut Writer) {
        println!("doc_header()");
    }

    fn doc_footer(&mut self, _w: &mut Writer) {
        println!("doc_footer()");
    }
}

fn main() {
    let mut cb = DebugCallbacks;
    let md = Markdown::new(md::ExtTables | md::ExtFencedCode, 16, &mut cb);

    let document = io::stdin().read_to_end().unwrap();
    md.render(&mut io::stdout() as &mut Writer, document);
}
