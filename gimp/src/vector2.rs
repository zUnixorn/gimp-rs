use crate::{ffi};
use glib::{translate::*};

// TODO check if this is right and implement methods

glib::wrapper! {
    /// A two dimensional vector.
    pub struct Vector2(Boxed<ffi::GimpVector2>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gimp_vector2_get_type(), ptr as *mut _) as *mut ffi::GimpVector2,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gimp_vector2_get_type(), ptr as *mut _),
        type_ => || ffi::gimp_vector2_get_type(),
    }
}

