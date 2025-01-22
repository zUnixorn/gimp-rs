// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,ColorProfile,ColorRenderingIntent,ColorTransformFlags};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    /// Definitions and Functions relating to LCMS.
    ///
    /// ## Signals
    ///
    ///
    /// #### `progress`
    ///
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpColorTransform")]
    pub struct ColorTransform(Object<ffi::GimpColorTransform, ffi::GimpColorTransformClass>);

    match fn {
        type_ => || ffi::gimp_color_transform_get_type(),
    }
}

impl ColorTransform {
    /// This function creates an color transform.
    ///
    /// The color transform is determined exclusively by `src_profile` and
    /// `dest_profile`. The color spaces of `src_format` and `dest_format` are
    /// ignored, the formats are only used to decide between what pixel
    /// encodings to transform.
    ///
    /// Note: this function used to return [`None`] if
    /// `gimp_color_transform_can_gegl_copy()` returned [`true`] for
    /// `src_profile` and `dest_profile`. This is no longer the case because
    /// special care has to be taken not to perform multiple implicit color
    /// transforms caused by babl formats with color spaces. Now, it always
    /// returns a non-[`None`] transform and the code takes care of doing only
    /// exactly the requested color transform.
    /// ## `src_profile`
    /// the source [`ColorProfile`][crate::ColorProfile]
    /// ## `src_format`
    /// the source `Babl` format
    /// ## `dest_profile`
    /// the destination [`ColorProfile`][crate::ColorProfile]
    /// ## `dest_format`
    /// the destination `Babl` format
    /// ## `rendering_intent`
    /// the rendering intent
    /// ## `flags`
    /// transform flags
    ///
    /// # Returns
    ///
    /// the [`ColorTransform`][crate::ColorTransform], or [`None`] if there was an error.
    #[doc(alias = "gimp_color_transform_new")]
    pub fn new(src_profile: &ColorProfile, src_format: &babl::Object, dest_profile: &ColorProfile, dest_format: &babl::Object, rendering_intent: ColorRenderingIntent, flags: ColorTransformFlags) -> Option<ColorTransform> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gimp_color_transform_new(src_profile.to_glib_none().0, src_format.to_glib_none().0, dest_profile.to_glib_none().0, dest_format.to_glib_none().0, rendering_intent.into_glib(), flags.into_glib()))
        }
    }

    /// This function creates a simulation / proofing color transform.
    ///
    /// See [`new()`][Self::new()] about the color spaces to transform
    /// between.
    /// ## `src_profile`
    /// the source [`ColorProfile`][crate::ColorProfile]
    /// ## `src_format`
    /// the source `Babl` format
    /// ## `dest_profile`
    /// the destination [`ColorProfile`][crate::ColorProfile]
    /// ## `dest_format`
    /// the destination `Babl` format
    /// ## `proof_profile`
    /// the proof [`ColorProfile`][crate::ColorProfile]
    /// ## `proof_intent`
    /// the proof intent
    /// ## `display_intent`
    /// the display intent
    /// ## `flags`
    /// transform flags
    ///
    /// # Returns
    ///
    /// the [`ColorTransform`][crate::ColorTransform], or [`None`] if there was an error.
    #[doc(alias = "gimp_color_transform_new_proofing")]
    pub fn new_proofing(src_profile: &ColorProfile, src_format: &babl::Object, dest_profile: &ColorProfile, dest_format: &babl::Object, proof_profile: &ColorProfile, proof_intent: ColorRenderingIntent, display_intent: ColorRenderingIntent, flags: ColorTransformFlags) -> Option<ColorTransform> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gimp_color_transform_new_proofing(src_profile.to_glib_none().0, src_format.to_glib_none().0, dest_profile.to_glib_none().0, dest_format.to_glib_none().0, proof_profile.to_glib_none().0, proof_intent.into_glib(), display_intent.into_glib(), flags.into_glib()))
        }
    }

    /// This function transforms buffer into another buffer.
    ///
    /// See [`new()`][Self::new()]: only the pixel encoding of
    /// `src_buffer`'s and `dest_buffer`'s formats honored, their color
    /// spaces are ignored. The transform always takes place between the
    /// color spaces determined by `self`'s color profiles.
    /// ## `src_buffer`
    /// source [`gegl::Buffer`][crate::gegl::Buffer]
    /// ## `src_rect`
    /// rectangle in `src_buffer`
    /// ## `dest_buffer`
    /// destination [`gegl::Buffer`][crate::gegl::Buffer]
    /// ## `dest_rect`
    /// rectangle in `dest_buffer`
    #[doc(alias = "gimp_color_transform_process_buffer")]
    pub fn process_buffer(&self, src_buffer: &gegl::Buffer, src_rect: &gegl::Rectangle, dest_buffer: &gegl::Buffer, dest_rect: &gegl::Rectangle) {
        unsafe {
            ffi::gimp_color_transform_process_buffer(self.to_glib_none().0, src_buffer.to_glib_none().0, src_rect.to_glib_none().0, dest_buffer.to_glib_none().0, dest_rect.to_glib_none().0);
        }
    }

    //#[doc(alias = "gimp_color_transform_process_pixels")]
    //pub fn process_pixels(&self, src_format: &babl::Object, src_pixels: /*Unimplemented*/Option<Basic: Pointer>, dest_format: &babl::Object, dest_pixels: /*Unimplemented*/Option<Basic: Pointer>, length: usize) {
    //    unsafe { TODO: call ffi:gimp_color_transform_process_pixels() }
    //}

    #[doc(alias = "gimp_color_transform_can_gegl_copy")]
    pub fn can_gegl_copy(src_profile: &ColorProfile, dest_profile: &ColorProfile) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_color_transform_can_gegl_copy(src_profile.to_glib_none().0, dest_profile.to_glib_none().0))
        }
    }

    #[doc(alias = "progress")]
    pub fn connect_progress<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn progress_trampoline<F: Fn(&ColorTransform, f64) + 'static>(this: *mut ffi::GimpColorTransform, object: std::ffi::c_double, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"progress".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(progress_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
