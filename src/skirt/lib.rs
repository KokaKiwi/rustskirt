#[crate_id = "skirt#0.1.0"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];
#[license = "MIT"];

#[feature(globs)];
#[feature(macro_rules)];

pub use md = markdown;
pub use md::Markdown;
pub use callbacks::Callbacks;
pub use buffer::Buffer;

#[allow(non_camel_case_types)]
pub mod ffi;
pub mod callbacks;
pub mod markdown;
pub mod buffer;

pub enum Autolink {
    NotAutolink,
    Normal,
    Email,
}

impl Autolink {
    pub fn from_ffi(ty: ffi::mkd_autolink) -> Autolink {
        match ty {
            ffi::MKDA_NOT_AUTOLINK => NotAutolink,
            ffi::MKDA_NORMAL => Normal,
            ffi::MKDA_EMAIL => Email,
        }
    }
}

