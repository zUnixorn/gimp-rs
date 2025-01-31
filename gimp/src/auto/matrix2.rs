// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{translate::*};

glib::wrapper! {
    /// A two by two matrix.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Matrix2(Boxed<ffi::GimpMatrix2>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gimp_matrix2_get_type(), ptr as *mut _) as *mut ffi::GimpMatrix2,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gimp_matrix2_get_type(), ptr as *mut _),
        type_ => || ffi::gimp_matrix2_get_type(),
    }
}

impl Matrix2 {
    /// Calculates the determinant of the given matrix.
    ///
    /// # Returns
    ///
    /// The determinant.
    #[doc(alias = "gimp_matrix2_determinant")]
    pub fn determinant(&self) -> f64 {
        unsafe {
            ffi::gimp_matrix2_determinant(self.to_glib_none().0)
        }
    }

    /// Sets the matrix to the identity matrix.
    #[doc(alias = "gimp_matrix2_identity")]
    pub fn identity(&mut self) {
        unsafe {
            ffi::gimp_matrix2_identity(self.to_glib_none_mut().0);
        }
    }

    /// Inverts the given matrix.
    #[doc(alias = "gimp_matrix2_invert")]
    pub fn invert(&mut self) {
        unsafe {
            ffi::gimp_matrix2_invert(self.to_glib_none_mut().0);
        }
    }

    /// Multiplies two matrices and puts the result into the second one.
    /// ## `right`
    /// The second input matrix which will be overwritten by the result.
    #[doc(alias = "gimp_matrix2_mult")]
    pub fn mult(&self, right: &mut Matrix2) {
        unsafe {
            ffi::gimp_matrix2_mult(self.to_glib_none().0, right.to_glib_none_mut().0);
        }
    }

    /// Transforms a point in 2D as specified by the transformation matrix.
    /// ## `x`
    /// The source X coordinate.
    /// ## `y`
    /// The source Y coordinate.
    ///
    /// # Returns
    ///
    ///
    /// ## `newx`
    /// The transformed X coordinate.
    ///
    /// ## `newy`
    /// The transformed Y coordinate.
    #[doc(alias = "gimp_matrix2_transform_point")]
    pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe {
            let mut newx = std::mem::MaybeUninit::uninit();
            let mut newy = std::mem::MaybeUninit::uninit();
            ffi::gimp_matrix2_transform_point(self.to_glib_none().0, x, y, newx.as_mut_ptr(), newy.as_mut_ptr());
            (newx.assume_init(), newy.assume_init())
        }
    }
}
