// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,GradientSegmentColor,GradientSegmentType,Resource};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "GimpGradient")]
    pub struct Gradient(Object<ffi::GimpGradient, ffi::GimpGradientClass>) @extends Resource;

    match fn {
        type_ => || ffi::gimp_gradient_get_type(),
    }
}

impl Gradient {
    #[doc(alias = "gimp_gradient_new")]
    pub fn new(name: &str) -> Gradient {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_gradient_new(name.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_gradient_get_custom_samples")]
    #[doc(alias = "get_custom_samples")]
    pub fn custom_samples(&self, positions: &[f64], reverse: bool) -> Vec<gegl::Color> {
        let num_samples = positions.len() as _;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gimp_gradient_get_custom_samples(self.to_glib_none().0, num_samples, positions.to_glib_none().0, reverse.into_glib()))
        }
    }

    #[doc(alias = "gimp_gradient_get_number_of_segments")]
    #[doc(alias = "get_number_of_segments")]
    pub fn number_of_segments(&self) -> i32 {
        unsafe {
            ffi::gimp_gradient_get_number_of_segments(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gimp_gradient_get_uniform_samples")]
    #[doc(alias = "get_uniform_samples")]
    pub fn uniform_samples(&self, num_samples: i32, reverse: bool) -> Vec<gegl::Color> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gimp_gradient_get_uniform_samples(self.to_glib_none().0, num_samples, reverse.into_glib()))
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_blending_function")]
    pub fn segment_get_blending_function(&self, segment: i32) -> Option<GradientSegmentType> {
        unsafe {
            let mut blend_func = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_get_blending_function(self.to_glib_none().0, segment, blend_func.as_mut_ptr()));
            if ret { Some(from_glib(blend_func.assume_init())) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_coloring_type")]
    pub fn segment_get_coloring_type(&self, segment: i32) -> Option<GradientSegmentColor> {
        unsafe {
            let mut coloring_type = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_get_coloring_type(self.to_glib_none().0, segment, coloring_type.as_mut_ptr()));
            if ret { Some(from_glib(coloring_type.assume_init())) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_left_color")]
    pub fn segment_get_left_color(&self, segment: i32) -> Option<gegl::Color> {
        unsafe {
            from_glib_full(ffi::gimp_gradient_segment_get_left_color(self.to_glib_none().0, segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_left_pos")]
    pub fn segment_get_left_pos(&self, segment: i32) -> Option<f64> {
        unsafe {
            let mut pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_get_left_pos(self.to_glib_none().0, segment, pos.as_mut_ptr()));
            if ret { Some(pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_middle_pos")]
    pub fn segment_get_middle_pos(&self, segment: i32) -> Option<f64> {
        unsafe {
            let mut pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_get_middle_pos(self.to_glib_none().0, segment, pos.as_mut_ptr()));
            if ret { Some(pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_right_color")]
    pub fn segment_get_right_color(&self, segment: i32) -> Option<gegl::Color> {
        unsafe {
            from_glib_full(ffi::gimp_gradient_segment_get_right_color(self.to_glib_none().0, segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_get_right_pos")]
    pub fn segment_get_right_pos(&self, segment: i32) -> Option<f64> {
        unsafe {
            let mut pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_get_right_pos(self.to_glib_none().0, segment, pos.as_mut_ptr()));
            if ret { Some(pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_blend_colors")]
    pub fn segment_range_blend_colors(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_blend_colors(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_blend_opacity")]
    pub fn segment_range_blend_opacity(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_blend_opacity(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_delete")]
    pub fn segment_range_delete(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_delete(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_flip")]
    pub fn segment_range_flip(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_flip(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_move")]
    pub fn segment_range_move(&self, start_segment: i32, end_segment: i32, delta: f64, control_compress: bool) -> f64 {
        unsafe {
            ffi::gimp_gradient_segment_range_move(self.to_glib_none().0, start_segment, end_segment, delta, control_compress.into_glib())
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_redistribute_handles")]
    pub fn segment_range_redistribute_handles(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_redistribute_handles(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_replicate")]
    pub fn segment_range_replicate(&self, start_segment: i32, end_segment: i32, replicate_times: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_replicate(self.to_glib_none().0, start_segment, end_segment, replicate_times))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_set_blending_function")]
    pub fn segment_range_set_blending_function(&self, start_segment: i32, end_segment: i32, blending_function: GradientSegmentType) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_set_blending_function(self.to_glib_none().0, start_segment, end_segment, blending_function.into_glib()))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_set_coloring_type")]
    pub fn segment_range_set_coloring_type(&self, start_segment: i32, end_segment: i32, coloring_type: GradientSegmentColor) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_set_coloring_type(self.to_glib_none().0, start_segment, end_segment, coloring_type.into_glib()))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_split_midpoint")]
    pub fn segment_range_split_midpoint(&self, start_segment: i32, end_segment: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_split_midpoint(self.to_glib_none().0, start_segment, end_segment))
        }
    }

    #[doc(alias = "gimp_gradient_segment_range_split_uniform")]
    pub fn segment_range_split_uniform(&self, start_segment: i32, end_segment: i32, split_parts: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_range_split_uniform(self.to_glib_none().0, start_segment, end_segment, split_parts))
        }
    }

    #[doc(alias = "gimp_gradient_segment_set_left_color")]
    pub fn segment_set_left_color(&self, segment: i32, color: &impl IsA<gegl::Color>) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_set_left_color(self.to_glib_none().0, segment, color.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_gradient_segment_set_left_pos")]
    pub fn segment_set_left_pos(&self, segment: i32, pos: f64) -> Option<f64> {
        unsafe {
            let mut final_pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_set_left_pos(self.to_glib_none().0, segment, pos, final_pos.as_mut_ptr()));
            if ret { Some(final_pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_set_middle_pos")]
    pub fn segment_set_middle_pos(&self, segment: i32, pos: f64) -> Option<f64> {
        unsafe {
            let mut final_pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_set_middle_pos(self.to_glib_none().0, segment, pos, final_pos.as_mut_ptr()));
            if ret { Some(final_pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_segment_set_right_color")]
    pub fn segment_set_right_color(&self, segment: i32, color: &impl IsA<gegl::Color>) -> bool {
        unsafe {
            from_glib(ffi::gimp_gradient_segment_set_right_color(self.to_glib_none().0, segment, color.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_gradient_segment_set_right_pos")]
    pub fn segment_set_right_pos(&self, segment: i32, pos: f64) -> Option<f64> {
        unsafe {
            let mut final_pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_gradient_segment_set_right_pos(self.to_glib_none().0, segment, pos, final_pos.as_mut_ptr()));
            if ret { Some(final_pos.assume_init()) } else { None }
        }
    }

    #[doc(alias = "gimp_gradient_get_by_name")]
    #[doc(alias = "get_by_name")]
    pub fn by_name(name: &str) -> Option<Gradient> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_gradient_get_by_name(name.to_glib_none().0))
        }
    }
}
