// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Resource};
use glib::{translate::*};

glib::wrapper! {
    /// Installable object used by fill and clone tools.
    ///
    /// # Implements
    ///
    /// [`ResourceExt`][trait@crate::prelude::ResourceExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpPattern")]
    pub struct Pattern(Object<ffi::GimpPattern, ffi::GimpPatternClass>) @extends Resource;

    match fn {
        type_ => || ffi::gimp_pattern_get_type(),
    }
}

impl Pattern {
    /// Gets pixel data of the pattern within the bounding box specified by `max_width`
    /// and `max_height`. The data will be scaled down so that it fits within this
    /// size without changing its ratio. If the pattern is smaller than this size to
    /// begin with, it will not be scaled up.
    ///
    /// If `max_width` or `max_height` are [`None`], the buffer is returned in the pattern's
    /// native size.
    ///
    /// Make sure you called [func`Gegl`] before calling any function using
    /// `GEGL`.
    /// ## `max_width`
    /// a maximum width for the returned buffer.
    /// ## `max_height`
    /// a maximum height for the returned buffer.
    /// ## `format`
    /// an optional Babl format.
    ///
    /// # Returns
    ///
    /// a [class`Gegl`].
    #[doc(alias = "gimp_pattern_get_buffer")]
    #[doc(alias = "get_buffer")]
    pub fn buffer(&self, max_width: i32, max_height: i32, format: &babl::Object) -> Option<gegl::Buffer> {
        unsafe {
            from_glib_full(ffi::gimp_pattern_get_buffer(self.to_glib_none().0, max_width, max_height, format.to_glib_none().0))
        }
    }

    /// Gets information about the pattern.
    ///
    /// Gets information about the pattern: the pattern extents (width and
    /// height) and bytes per pixel.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    ///
    /// ## `width`
    /// The pattern width.
    ///
    /// ## `height`
    /// The pattern height.
    ///
    /// ## `bpp`
    /// The pattern bpp.
    #[doc(alias = "gimp_pattern_get_info")]
    #[doc(alias = "get_info")]
    pub fn info(&self) -> Option<(i32, i32, i32)> {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            let mut bpp = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_pattern_get_info(self.to_glib_none().0, width.as_mut_ptr(), height.as_mut_ptr(), bpp.as_mut_ptr()));
            if ret { Some((width.assume_init(), height.assume_init(), bpp.assume_init())) } else { None }
        }
    }

    /// Returns the pattern with the given name.
    ///
    /// Returns an existing pattern having the given name. Returns [`None`]
    /// when no pattern exists of that name.
    /// ## `name`
    /// The name of the pattern.
    ///
    /// # Returns
    ///
    /// The pattern.
    #[doc(alias = "gimp_pattern_get_by_name")]
    #[doc(alias = "get_by_name")]
    pub fn by_name(name: &str) -> Option<Pattern> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_pattern_get_by_name(name.to_glib_none().0))
        }
    }
}
