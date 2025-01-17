#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {};
}

use ffi;
pub use auto::*;
pub mod subclass;
mod auto;

pub use cairo;
pub use gdk_pixbuf;
pub use gegl;
pub use babl;
pub use gio;
pub use glib;
pub use pango;

pub mod functions {
    pub use super::auto::functions::*;
}

pub mod traits {
    pub use super::auto::traits::*;
}

