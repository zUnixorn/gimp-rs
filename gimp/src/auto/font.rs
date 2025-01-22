// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Resource};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "GimpFont")]
    pub struct Font(Object<ffi::GimpFont, ffi::GimpFontClass>) @extends Resource;

    match fn {
        type_ => || ffi::gimp_font_get_type(),
    }
}

impl Font {
    #[doc(alias = "gimp_font_get_pango_font_description")]
    #[doc(alias = "get_pango_font_description")]
    pub fn pango_font_description(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gimp_font_get_pango_font_description(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_font_get_by_name")]
    #[doc(alias = "get_by_name")]
    pub fn by_name(name: &str) -> Option<Font> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_font_get_by_name(name.to_glib_none().0))
        }
    }
}
