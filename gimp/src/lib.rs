#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {};
}

mod auto;
mod procedure;
mod image_procedure;
mod macros;
pub mod prelude;
pub mod subclass;

pub mod functions {
    pub use super::auto::functions::*;
}

use ffi;
pub use auto::*;
pub use prelude::*;
pub use cairo;
pub use gdk_pixbuf;
pub use gegl;
pub use babl;
pub use gio;
pub use glib;
pub use pango;
