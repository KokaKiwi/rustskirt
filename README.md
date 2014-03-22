rustskirt
=========

rustskirt is an unofficial Sundown library bindings for Rust language.

## Installation

### Get the source

~~~sh
git clone --recursive git://github.com/KokaKiwi/rustskirt.git
~~~

### Compile

~~~sh
make
~~~

## Usage

### Basic usage for a custom Markdown-backend

~~~rust
extern crate skirt;

use std::io;

use skirt::Markdown;
use skirt::Callbacks;

struct MyCallbacks;

#[allow(unused_must_use)]
impl Callbacks for MyCallbacks {
    fn normal_text(&mut self, w: &mut Writer, text: &str) {
        writeln!(w, "normal_text: '{}'", text);
    }
}

fn main() {
    let mut cb = MyCallbacks;
    let md = Markdown::new(0, 16, &mut cb);

    let document = io::stdin().read_to_end().unwrap();
    md.render(&mut io::stdout() as &mut Writer, document);
}
~~~
