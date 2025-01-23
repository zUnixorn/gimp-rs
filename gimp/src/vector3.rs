use crate::{ffi};
use glib::{translate::*};

glib::wrapper! {
    /// A three dimensional vector.
    pub struct Vector3(Boxed<ffi::GimpVector3>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gimp_vector3_get_type(), ptr as *mut _) as *mut ffi::GimpVector3,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gimp_vector3_get_type(), ptr as *mut _),
        type_ => || ffi::gimp_vector3_get_type(),
    }
}

