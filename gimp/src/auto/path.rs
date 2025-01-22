// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Image,Item,Layer,OrientationType,PathStrokeType};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    /// Functions for querying and manipulating path.
    ///
    /// # Implements
    ///
    /// [`ItemExt`][trait@crate::prelude::ItemExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpPath")]
    pub struct Path(Object<ffi::GimpPath, ffi::GimpPathClass>) @extends Item;

    match fn {
        type_ => || ffi::gimp_path_get_type(),
    }
}

impl Path {
    /// Creates a new empty path object.
    ///
    /// Creates a new empty path object. The path object needs to be added
    /// to the image using [`Image::insert_path()`][crate::Image::insert_path()].
    /// ## `image`
    /// The image.
    /// ## `name`
    /// the name of the new path object.
    ///
    /// # Returns
    ///
    ///
    ///  the current path object, 0 if no path exists in the image.
    #[doc(alias = "gimp_path_new")]
    pub fn new(image: &Image, name: &str) -> Path {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_path_new(image.to_glib_none().0, name.to_glib_none().0))
        }
    }

    /// Creates a new path object from a text layer.
    ///
    /// Creates a new path object from a text layer. The path object needs
    /// to be added to the image using [`Image::insert_path()`][crate::Image::insert_path()].
    /// ## `image`
    /// The image.
    /// ## `layer`
    /// The text layer.
    ///
    /// # Returns
    ///
    /// The path of the text layer.
    #[doc(alias = "gimp_path_new_from_text_layer")]
    #[doc(alias = "new_from_text_layer")]
    pub fn from_text_layer(image: &Image, layer: &impl IsA<Layer>) -> Path {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_path_new_from_text_layer(image.to_glib_none().0, layer.as_ref().to_glib_none().0))
        }
    }

    /// Extends a bezier stroke with a conic bezier spline.
    ///
    /// Extends a bezier stroke with a conic bezier spline. Actually a cubic
    /// bezier spline gets added that realizes the shape of a conic bezier
    /// spline.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `x0`
    /// The x-coordinate of the control point.
    /// ## `y0`
    /// The y-coordinate of the control point.
    /// ## `x1`
    /// The x-coordinate of the end point.
    /// ## `y1`
    /// The y-coordinate of the end point.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_bezier_stroke_conicto")]
    pub fn bezier_stroke_conicto(&self, stroke_id: i32, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_bezier_stroke_conicto(self.to_glib_none().0, stroke_id, x0, y0, x1, y1))
        }
    }

    /// Extends a bezier stroke with a cubic bezier spline.
    ///
    /// Extends a bezier stroke with a cubic bezier spline.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `x0`
    /// The x-coordinate of the first control point.
    /// ## `y0`
    /// The y-coordinate of the first control point.
    /// ## `x1`
    /// The x-coordinate of the second control point.
    /// ## `y1`
    /// The y-coordinate of the second control point.
    /// ## `x2`
    /// The x-coordinate of the end point.
    /// ## `y2`
    /// The y-coordinate of the end point.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_bezier_stroke_cubicto")]
    pub fn bezier_stroke_cubicto(&self, stroke_id: i32, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_bezier_stroke_cubicto(self.to_glib_none().0, stroke_id, x0, y0, x1, y1, x2, y2))
        }
    }

    /// Extends a bezier stroke with a lineto.
    ///
    /// Extends a bezier stroke with a lineto.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `x0`
    /// The x-coordinate of the lineto.
    /// ## `y0`
    /// The y-coordinate of the lineto.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_bezier_stroke_lineto")]
    pub fn bezier_stroke_lineto(&self, stroke_id: i32, x0: f64, y0: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_bezier_stroke_lineto(self.to_glib_none().0, stroke_id, x0, y0))
        }
    }

    /// Adds a bezier stroke describing an ellipse the path object.
    ///
    /// Adds a bezier stroke describing an ellipse on the path object.
    /// ## `x0`
    /// The x-coordinate of the center.
    /// ## `y0`
    /// The y-coordinate of the center.
    /// ## `radius_x`
    /// The radius in x direction.
    /// ## `radius_y`
    /// The radius in y direction.
    /// ## `angle`
    /// The angle the x-axis of the ellipse (radians, counterclockwise).
    ///
    /// # Returns
    ///
    /// The resulting stroke.
    #[doc(alias = "gimp_path_bezier_stroke_new_ellipse")]
    pub fn bezier_stroke_new_ellipse(&self, x0: f64, y0: f64, radius_x: f64, radius_y: f64, angle: f64) -> i32 {
        unsafe {
            ffi::gimp_path_bezier_stroke_new_ellipse(self.to_glib_none().0, x0, y0, radius_x, radius_y, angle)
        }
    }

    /// Adds a bezier stroke with a single moveto to the path object.
    ///
    /// Adds a bezier stroke with a single moveto to the path object.
    /// ## `x0`
    /// The x-coordinate of the moveto.
    /// ## `y0`
    /// The y-coordinate of the moveto.
    ///
    /// # Returns
    ///
    /// The resulting stroke.
    #[doc(alias = "gimp_path_bezier_stroke_new_moveto")]
    pub fn bezier_stroke_new_moveto(&self, x0: f64, y0: f64) -> i32 {
        unsafe {
            ffi::gimp_path_bezier_stroke_new_moveto(self.to_glib_none().0, x0, y0)
        }
    }

    #[doc(alias = "gimp_path_copy")]
#[must_use]
    pub fn copy(&self) -> Option<Path> {
        unsafe {
            from_glib_none(ffi::gimp_path_copy(self.to_glib_none().0))
        }
    }

    /// List the strokes associated with the passed path.
    ///
    /// Returns an Array with the stroke-IDs associated with the passed
    /// path.
    ///
    /// # Returns
    ///
    ///
    ///  List of the strokes belonging to the path.
    ///  The returned value must be freed with `g_free()`.
    #[doc(alias = "gimp_path_get_strokes")]
    #[doc(alias = "get_strokes")]
    pub fn strokes(&self) -> Vec<i32> {
        unsafe {
            let mut num_strokes = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gimp_path_get_strokes(self.to_glib_none().0, num_strokes.as_mut_ptr()), num_strokes.assume_init() as _);
            ret
        }
    }

    /// remove the stroke from a path object.
    ///
    /// Remove the stroke from a path object.
    /// ## `stroke_id`
    /// The stroke ID.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_remove_stroke")]
    pub fn remove_stroke(&self, stroke_id: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_remove_stroke(self.to_glib_none().0, stroke_id))
        }
    }

    /// closes the specified stroke.
    ///
    /// Closes the specified stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_close")]
    pub fn stroke_close(&self, stroke_id: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_close(self.to_glib_none().0, stroke_id))
        }
    }

    /// flips the given stroke.
    ///
    /// Rotates the given stroke around given center by angle (in degrees).
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `flip_type`
    /// Flip orientation, either vertical or horizontal.
    /// ## `axis`
    /// axis coordinate about which to flip, in pixels.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_flip")]
    pub fn stroke_flip(&self, stroke_id: i32, flip_type: OrientationType, axis: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_flip(self.to_glib_none().0, stroke_id, flip_type.into_glib(), axis))
        }
    }

    #[doc(alias = "gimp_path_stroke_flip_free")]
    pub fn stroke_flip_free(&self, stroke_id: i32, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_flip_free(self.to_glib_none().0, stroke_id, x1, y1, x2, y2))
        }
    }

    /// Measure the length of the given stroke.
    ///
    /// Measure the length of the given stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `precision`
    /// The precision used for approximating straight portions of the stroke.
    ///
    /// # Returns
    ///
    /// The length (in pixels) of the given stroke.
    #[doc(alias = "gimp_path_stroke_get_length")]
    pub fn stroke_get_length(&self, stroke_id: i32, precision: f64) -> f64 {
        unsafe {
            ffi::gimp_path_stroke_get_length(self.to_glib_none().0, stroke_id, precision)
        }
    }

    /// Get point at a specified distance along the stroke.
    ///
    /// This will return the x,y position of a point at a given distance
    /// along the stroke. The distance will be obtained by first digitizing
    /// the curve internally and then walking along the curve. For a closed
    /// stroke the start of the path is the first point on the path that was
    /// created. This might not be obvious. If the stroke is not long
    /// enough, a \"valid\" flag will be FALSE.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `dist`
    /// The given distance.
    /// ## `precision`
    /// The precision used for the approximation.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    ///
    /// ## `x_point`
    /// The x position of the point.
    ///
    /// ## `y_point`
    /// The y position of the point.
    ///
    /// ## `slope`
    /// The slope (dy / dx) at the specified point.
    ///
    /// ## `valid`
    /// Indicator for the validity of the returned data.
    #[doc(alias = "gimp_path_stroke_get_point_at_dist")]
    pub fn stroke_get_point_at_dist(&self, stroke_id: i32, dist: f64, precision: f64) -> Option<(f64, f64, f64, bool)> {
        unsafe {
            let mut x_point = std::mem::MaybeUninit::uninit();
            let mut y_point = std::mem::MaybeUninit::uninit();
            let mut slope = std::mem::MaybeUninit::uninit();
            let mut valid = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_path_stroke_get_point_at_dist(self.to_glib_none().0, stroke_id, dist, precision, x_point.as_mut_ptr(), y_point.as_mut_ptr(), slope.as_mut_ptr(), valid.as_mut_ptr()));
            if ret { Some((x_point.assume_init(), y_point.assume_init(), slope.assume_init(), from_glib(valid.assume_init()))) } else { None }
        }
    }

    /// returns the control points of a stroke.
    ///
    /// returns the control points of a stroke. The interpretation of the
    /// coordinates returned depends on the type of the stroke. For Gimp 2.4
    /// this is always a bezier stroke, where the coordinates are the
    /// control points.
    /// ## `stroke_id`
    /// The stroke ID.
    ///
    /// # Returns
    ///
    /// type of the stroke (always GIMP_PATH_STROKE_TYPE_BEZIER for now).
    ///
    /// ## `controlpoints`
    /// List of the control points for the stroke (x0, y0, x1, y1, ...).
    ///
    /// ## `closed`
    /// Whether the stroke is closed or not.
    #[doc(alias = "gimp_path_stroke_get_points")]
    pub fn stroke_get_points(&self, stroke_id: i32) -> (PathStrokeType, Vec<f64>, bool) {
        unsafe {
            let mut num_points = std::mem::MaybeUninit::uninit();
            let mut controlpoints = std::ptr::null_mut();
            let mut closed = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_path_stroke_get_points(self.to_glib_none().0, stroke_id, num_points.as_mut_ptr(), &mut controlpoints, closed.as_mut_ptr()));
            (ret, FromGlibContainer::from_glib_full_num(controlpoints, num_points.assume_init() as _), from_glib(closed.assume_init()))
        }
    }

    /// returns polygonal approximation of the stroke.
    ///
    /// returns polygonal approximation of the stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `precision`
    /// The precision used for the approximation.
    ///
    /// # Returns
    ///
    ///
    ///  List of the coords along the path (x0, y0, x1, y1, ...).
    ///  The returned value must be freed with `g_free()`.
    ///
    /// ## `closed`
    /// Whether the stroke is closed or not.
    #[doc(alias = "gimp_path_stroke_interpolate")]
    pub fn stroke_interpolate(&self, stroke_id: i32, precision: f64) -> (Vec<f64>, bool) {
        unsafe {
            let mut num_coords = std::mem::MaybeUninit::uninit();
            let mut closed = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gimp_path_stroke_interpolate(self.to_glib_none().0, stroke_id, precision, num_coords.as_mut_ptr(), closed.as_mut_ptr()), num_coords.assume_init() as _);
            (ret, from_glib(closed.assume_init()))
        }
    }

    /// Adds a stroke of a given type to the path object.
    ///
    /// Adds a stroke of a given type to the path object. The coordinates of
    /// the control points can be specified. For now only strokes of the
    /// type GIMP_PATH_STROKE_TYPE_BEZIER are supported. The control points
    /// are specified as a pair of double values for the x- and
    /// y-coordinate. The Bezier stroke type needs a multiple of three
    /// control points. Each Bezier segment endpoint (anchor, A) has two
    /// additional control points (C) associated. They are specified in the
    /// order CACCACCAC...
    /// ## `type_`
    /// type of the stroke (always GIMP_PATH_STROKE_TYPE_BEZIER for now).
    /// ## `controlpoints`
    /// List of the x- and y-coordinates of the control points.
    /// ## `closed`
    /// Whether the stroke is to be closed or not.
    ///
    /// # Returns
    ///
    /// The stroke ID of the newly created stroke.
    #[doc(alias = "gimp_path_stroke_new_from_points")]
    pub fn stroke_new_from_points(&self, type_: PathStrokeType, controlpoints: &[f64], closed: bool) -> i32 {
        let num_points = controlpoints.len() as _;
        unsafe {
            ffi::gimp_path_stroke_new_from_points(self.to_glib_none().0, type_.into_glib(), num_points, controlpoints.to_glib_none().0, closed.into_glib())
        }
    }

    /// reverses the specified stroke.
    ///
    /// Reverses the specified stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_reverse")]
    pub fn stroke_reverse(&self, stroke_id: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_reverse(self.to_glib_none().0, stroke_id))
        }
    }

    /// rotates the given stroke.
    ///
    /// Rotates the given stroke around given center by angle (in degrees).
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `center_x`
    /// X coordinate of the rotation center.
    /// ## `center_y`
    /// Y coordinate of the rotation center.
    /// ## `angle`
    /// angle to rotate about.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_rotate")]
    pub fn stroke_rotate(&self, stroke_id: i32, center_x: f64, center_y: f64, angle: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_rotate(self.to_glib_none().0, stroke_id, center_x, center_y, angle))
        }
    }

    /// scales the given stroke.
    ///
    /// Scale the given stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `scale_x`
    /// Scale factor in x direction.
    /// ## `scale_y`
    /// Scale factor in y direction.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_scale")]
    pub fn stroke_scale(&self, stroke_id: i32, scale_x: f64, scale_y: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_scale(self.to_glib_none().0, stroke_id, scale_x, scale_y))
        }
    }

    /// translate the given stroke.
    ///
    /// Translate the given stroke.
    /// ## `stroke_id`
    /// The stroke ID.
    /// ## `off_x`
    /// Offset in x direction.
    /// ## `off_y`
    /// Offset in y direction.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_path_stroke_translate")]
    pub fn stroke_translate(&self, stroke_id: i32, off_x: f64, off_y: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_path_stroke_translate(self.to_glib_none().0, stroke_id, off_x, off_y))
        }
    }

    /// Returns a [`Path`][crate::Path] representing `path_id`. This function
    /// calls [`Item::by_id()`][crate::Item::by_id()] and returns the item if it is a path
    /// or [`None`] otherwise.
    /// ## `path_id`
    /// The path id.
    ///
    /// # Returns
    ///
    /// a [`Path`][crate::Path] for `path_id`
    ///  or [`None`] if `path_id` does not represent a valid
    ///  path. The object belongs to libgimp and you must not
    ///  modify or unref it.
    #[doc(alias = "gimp_path_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(path_id: i32) -> Option<Path> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_path_get_by_id(path_id))
        }
    }

    /// Note that you have to `g_free()` the returned string.
    /// ## `path`
    ///
    ///  A list of directories as returned by `gimp_path_parse()`.
    ///
    /// # Returns
    ///
    /// The first directory in `path` where the user has write permission.
    #[doc(alias = "gimp_path_get_user_writable_dir")]
    #[doc(alias = "get_user_writable_dir")]
    pub fn user_writable_dir(path: &[&std::path::Path]) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gimp_path_get_user_writable_dir(path.to_glib_none().0))
        }
    }

    //#[doc(alias = "gimp_path_parse")]
    //pub fn parse(path: &str, max_paths: i32, check: bool, check_failed: /*Unimplemented*/Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
    //    unsafe { TODO: call ffi:gimp_path_parse() }
    //}

    /// ## `path`
    ///
    ///  A list of directories as returned by `gimp_path_parse()`.
    ///
    /// # Returns
    ///
    ///
    ///  A searchpath string separated by `G_SEARCHPATH_SEPARATOR`.
    #[doc(alias = "gimp_path_to_str")]
    pub fn to_str(path: &[&std::path::Path]) -> Option<std::path::PathBuf> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gimp_path_to_str(path.to_glib_none().0))
        }
    }
}
