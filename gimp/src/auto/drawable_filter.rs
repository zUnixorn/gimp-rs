// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Drawable,DrawableFilterConfig,LayerMode};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "GimpDrawableFilter")]
    pub struct DrawableFilter(Object<ffi::GimpDrawableFilter, ffi::GimpDrawableFilterClass>);

    match fn {
        type_ => || ffi::gimp_drawable_filter_get_type(),
    }
}

impl DrawableFilter {
    #[doc(alias = "gimp_drawable_filter_new")]
    pub fn new(drawable: &impl IsA<Drawable>, operation_name: &str, name: &str) -> DrawableFilter {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_drawable_filter_new(drawable.as_ref().to_glib_none().0, operation_name.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_delete")]
    pub fn delete(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_drawable_filter_delete(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_blend_mode")]
    #[doc(alias = "get_blend_mode")]
    pub fn blend_mode(&self) -> LayerMode {
        unsafe {
            from_glib(ffi::gimp_drawable_filter_get_blend_mode(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_config")]
    #[doc(alias = "get_config")]
    pub fn config(&self) -> Option<DrawableFilterConfig> {
        unsafe {
            from_glib_none(ffi::gimp_drawable_filter_get_config(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> i32 {
        unsafe {
            ffi::gimp_drawable_filter_get_id(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gimp_drawable_filter_get_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_opacity")]
    #[doc(alias = "get_opacity")]
    pub fn opacity(&self) -> f64 {
        unsafe {
            ffi::gimp_drawable_filter_get_opacity(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_operation_name")]
    #[doc(alias = "get_operation_name")]
    pub fn operation_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gimp_drawable_filter_get_operation_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_drawable_filter_get_visible(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_drawable_filter_is_valid(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_drawable_filter_set_aux_input")]
    pub fn set_aux_input(&self, input_pad_name: &str, input: &impl IsA<Drawable>) {
        unsafe {
            ffi::gimp_drawable_filter_set_aux_input(self.to_glib_none().0, input_pad_name.to_glib_none().0, input.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_drawable_filter_set_blend_mode")]
    pub fn set_blend_mode(&self, mode: LayerMode) {
        unsafe {
            ffi::gimp_drawable_filter_set_blend_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gimp_drawable_filter_set_opacity")]
    pub fn set_opacity(&self, opacity: f64) {
        unsafe {
            ffi::gimp_drawable_filter_set_opacity(self.to_glib_none().0, opacity);
        }
    }

    #[doc(alias = "gimp_drawable_filter_set_visible")]
    pub fn set_visible(&self, visible: bool) -> bool {
        unsafe {
            from_glib(ffi::gimp_drawable_filter_set_visible(self.to_glib_none().0, visible.into_glib()))
        }
    }

    #[doc(alias = "gimp_drawable_filter_update")]
    pub fn update(&self) {
        unsafe {
            ffi::gimp_drawable_filter_update(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_drawable_filter_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(filter_id: i32) -> Option<DrawableFilter> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_drawable_filter_get_by_id(filter_id))
        }
    }

    #[doc(alias = "gimp_drawable_filter_id_is_valid")]
    pub fn id_is_valid(filter_id: i32) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gimp_drawable_filter_id_is_valid(filter_id))
        }
    }
}
