// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{GStr};

#[doc(alias = "GIMP_API_VERSION")]
pub static API_VERSION: &GStr = unsafe{GStr::from_utf8_with_nul_unchecked(ffi::GIMP_API_VERSION)};
/// The GIMP version as a string.
#[doc(alias = "GIMP_VERSION")]
pub static VERSION: &GStr = unsafe{GStr::from_utf8_with_nul_unchecked(ffi::GIMP_VERSION)};
