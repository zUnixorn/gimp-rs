// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Channel,Drawable,Image,Item,Layer};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "GimpSelection")]
    pub struct Selection(Object<ffi::GimpSelection, ffi::GimpSelectionClass>) @extends Channel, Drawable, Item;

    match fn {
        type_ => || ffi::gimp_selection_get_type(),
    }
}

impl Selection {
    #[doc(alias = "gimp_selection_all")]
    pub fn all(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_all(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_border")]
    pub fn border(image: &Image, radius: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_border(image.to_glib_none().0, radius))
        }
    }

    #[doc(alias = "gimp_selection_bounds")]
    pub fn bounds(image: &Image) -> Option<(bool, i32, i32, i32, i32)> {
        skip_assert_initialized!();
        unsafe {
            let mut non_empty = std::mem::MaybeUninit::uninit();
            let mut x1 = std::mem::MaybeUninit::uninit();
            let mut y1 = std::mem::MaybeUninit::uninit();
            let mut x2 = std::mem::MaybeUninit::uninit();
            let mut y2 = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_selection_bounds(image.to_glib_none().0, non_empty.as_mut_ptr(), x1.as_mut_ptr(), y1.as_mut_ptr(), x2.as_mut_ptr(), y2.as_mut_ptr()));
            if ret { Some((from_glib(non_empty.assume_init()), x1.assume_init(), y1.assume_init(), x2.assume_init(), y2.assume_init())) } else { None }
        }
    }

    #[doc(alias = "gimp_selection_feather")]
    pub fn feather(image: &Image, radius: f64) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_feather(image.to_glib_none().0, radius))
        }
    }

    #[doc(alias = "gimp_selection_float")]
    pub fn float(image: &Image, drawables: &[Drawable], offx: i32, offy: i32) -> Option<Layer> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_selection_float(image.to_glib_none().0, drawables.to_glib_none().0, offx, offy))
        }
    }

    #[doc(alias = "gimp_selection_flood")]
    pub fn flood(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_flood(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(selection_id: i32) -> Option<Selection> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_selection_get_by_id(selection_id))
        }
    }

    #[doc(alias = "gimp_selection_grow")]
    pub fn grow(image: &Image, steps: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_grow(image.to_glib_none().0, steps))
        }
    }

    #[doc(alias = "gimp_selection_invert")]
    pub fn invert(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_invert(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_is_empty")]
    pub fn is_empty(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_is_empty(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_none")]
    pub fn none(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_none(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_save")]
    pub fn save(image: &Image) -> Option<Channel> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_selection_save(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_sharpen")]
    pub fn sharpen(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_sharpen(image.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_selection_shrink")]
    pub fn shrink(image: &Image, steps: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_shrink(image.to_glib_none().0, steps))
        }
    }

    #[doc(alias = "gimp_selection_translate")]
    pub fn translate(image: &Image, offx: i32, offy: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_translate(image.to_glib_none().0, offx, offy))
        }
    }

    #[doc(alias = "gimp_selection_value")]
    pub fn value(image: &Image, x: i32, y: i32) -> i32 {
        skip_assert_initialized!();
        unsafe {
            ffi::gimp_selection_value(image.to_glib_none().0, x, y)
        }
    }
}
