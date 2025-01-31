// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Image};
use glib::{translate::*};

glib::wrapper! {
    /// Functions to create, delete and flush displays (views) on an image.
    ///
    /// ## Properties
    ///
    ///
    /// #### `id`
    ///  Readable | Writeable | Construct Only
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpDisplay")]
    pub struct Display(Object<ffi::GimpDisplay, ffi::GimpDisplayClass>);

    match fn {
        type_ => || ffi::gimp_display_get_type(),
    }
}

impl Display {
    /// Create a new display for the specified image.
    ///
    /// Creates a new display for the specified image. If the image already
    /// has a display, another is added. Multiple displays are handled
    /// transparently by GIMP. The newly created display is returned and can
    /// be subsequently destroyed with a call to [`delete()`][Self::delete()]. This
    /// procedure only makes sense for use with the GIMP UI, and will result
    /// in an execution error if called when GIMP has no UI.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// The new display.
    #[doc(alias = "gimp_display_new")]
    pub fn new(image: &Image) -> Display {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_display_new(image.to_glib_none().0))
        }
    }

    /// Delete the specified display.
    ///
    /// This procedure removes the specified display. If this is the last
    /// remaining display for the underlying image, then the image is
    /// deleted also. Note that the display is closed no matter if the image
    /// is dirty or not. Better save the image before calling this
    /// procedure.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_display_delete")]
    pub fn delete(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_display_delete(self.to_glib_none().0))
        }
    }

    /// Note: in most use cases, you should not need a display's ID which is
    /// mostly internal data and not reusable across sessions.
    ///
    /// # Returns
    ///
    /// the display ID.
    #[doc(alias = "gimp_display_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> i32 {
        unsafe {
            ffi::gimp_display_get_id(self.to_glib_none().0)
        }
    }

    /// Get a handle to the native window for an image display.
    ///
    /// This procedure returns a handle to the native window for a given
    /// image display.
    /// It can be different types of data depending on the platform you are
    /// running on. For example in the X backend of GDK, a native window
    /// handle is an Xlib XID whereas on Wayland, it is a string handle. A
    /// value of NULL is returned for an invalid display or if this function
    /// is unimplemented for the windowing system that is being used.
    ///
    /// # Returns
    ///
    /// The native window handle or NULL.
    #[doc(alias = "gimp_display_get_window_handle")]
    #[doc(alias = "get_window_handle")]
    pub fn window_handle(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(ffi::gimp_display_get_window_handle(self.to_glib_none().0))
        }
    }

    /// Returns TRUE if the display is valid.
    ///
    /// This procedure checks if the given display is valid and refers to
    /// an existing display.
    ///
    /// # Returns
    ///
    /// Whether the display is valid.
    #[doc(alias = "gimp_display_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_display_is_valid(self.to_glib_none().0))
        }
    }

    /// Present the specified display.
    ///
    /// This procedure presents the specified display at the top of the
    /// display stack.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_display_present")]
    pub fn present(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_display_present(self.to_glib_none().0))
        }
    }

    /// Returns a [`Display`][crate::Display] representing `display_id`.
    ///
    /// Note: in most use cases, you should not need to retrieve a
    /// [`Display`][crate::Display] by its ID, which is mostly internal data and not
    /// reusable across sessions. Use the appropriate functions for your use
    /// case instead.
    /// ## `display_id`
    /// The display id.
    ///
    /// # Returns
    ///
    /// a [`Display`][crate::Display] for `display_id` or
    ///  [`None`] if `display_id` does not represent a valid display.
    ///  The object belongs to libgimp and you must not modify or
    ///  unref it.
    #[doc(alias = "gimp_display_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(display_id: i32) -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_display_get_by_id(display_id))
        }
    }

    /// Returns TRUE if the display ID is valid.
    ///
    /// This procedure checks if the given display ID is valid and refers to
    /// an existing display.
    ///
    /// *Note*: in most use cases, you should not use this function. If you
    /// got a [class`Gimp`] from the API, you should trust it is
    /// valid. This function is mostly for internal usage.
    /// ## `display_id`
    /// The display ID to check.
    ///
    /// # Returns
    ///
    /// Whether the display ID is valid.
    #[doc(alias = "gimp_display_id_is_valid")]
    pub fn id_is_valid(display_id: i32) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gimp_display_id_is_valid(display_id))
        }
    }

    /// Returns the display to be used for plug-in windows.
    ///
    /// This is a constant value given at plug-in configuration time.
    /// Will return [`None`] if GIMP has been started with no GUI, either
    /// via "--no-interface" flag, or a console build.
    ///
    /// # Returns
    ///
    /// the display name
    #[doc(alias = "gimp_display_name")]
    pub fn name() -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_display_name())
        }
    }
}
