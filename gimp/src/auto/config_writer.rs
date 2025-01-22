// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    /// Functions for writing config info to a file for libgimpconfig.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConfigWriter(Shared<ffi::GimpConfigWriter>);

    match fn {
        ref => |ptr| ffi::gimp_config_writer_ref(ptr),
        unref => |ptr| ffi::gimp_config_writer_unref(ptr),
        type_ => || ffi::gimp_config_writer_get_type(),
    }
}

impl ConfigWriter {
    ///
    /// # Returns
    ///
    /// a new [`ConfigWriter`][crate::ConfigWriter] or [`None`] in case of an error
    #[doc(alias = "gimp_config_writer_new_from_fd")]
    #[doc(alias = "new_from_fd")]
    pub fn from_fd(fd: i32) -> Option<ConfigWriter> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gimp_config_writer_new_from_fd(fd))
        }
    }

    /// Creates a new [`ConfigWriter`][crate::ConfigWriter] and sets it up to write to
    /// `file`. If `atomic` is [`true`], a temporary file is used to avoid
    /// possible race conditions. The temporary file is then moved to `file`
    /// when the writer is closed.
    /// ## `file`
    /// a [`gio::File`][crate::gio::File]
    /// ## `atomic`
    /// if [`true`] the file is written atomically
    /// ## `header`
    /// text to include as comment at the top of the file
    ///
    /// # Returns
    ///
    /// a new [`ConfigWriter`][crate::ConfigWriter] or [`None`] in case of an error
    #[doc(alias = "gimp_config_writer_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(file: &impl IsA<gio::File>, atomic: bool, header: &str) -> Result<Option<ConfigWriter>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::gimp_config_writer_new_from_file(file.as_ref().to_glib_none().0, atomic.into_glib(), header.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// Creates a new [`ConfigWriter`][crate::ConfigWriter] and sets it up to write to
    /// `output`.
    /// ## `output`
    /// a [`gio::OutputStream`][crate::gio::OutputStream]
    /// ## `header`
    /// text to include as comment at the top of the file
    ///
    /// # Returns
    ///
    /// a new [`ConfigWriter`][crate::ConfigWriter] or [`None`] in case of an error
    #[doc(alias = "gimp_config_writer_new_from_stream")]
    #[doc(alias = "new_from_stream")]
    pub fn from_stream(output: &impl IsA<gio::OutputStream>, header: &str) -> Result<Option<ConfigWriter>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::gimp_config_writer_new_from_stream(output.as_ref().to_glib_none().0, header.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// Closes an element opened with [`open()`][Self::open()].
    #[doc(alias = "gimp_config_writer_close")]
    pub fn close(&self) {
        unsafe {
            ffi::gimp_config_writer_close(self.to_glib_none().0);
        }
    }

    /// Appends the `comment` to `str` and inserts linebreaks and hash-marks to
    /// format it as a comment. Note that this function does not handle non-ASCII
    /// characters.
    /// ## `comment`
    /// the comment to write (ASCII only)
    #[doc(alias = "gimp_config_writer_comment")]
    pub fn comment(&self, comment: &str) {
        unsafe {
            ffi::gimp_config_writer_comment(self.to_glib_none().0, comment.to_glib_none().0);
        }
    }

    /// This function toggles whether the `self` should create commented
    /// or uncommented output. This feature is used to generate the
    /// system-wide installed gimprc that documents the default settings.
    ///
    /// Since comments have to start at the beginning of a line, this
    /// function will insert a newline if necessary.
    /// ## `enable`
    /// [`true`] to enable comment mode, [`false`] to disable it
    #[doc(alias = "gimp_config_writer_comment_mode")]
    pub fn comment_mode(&self, enable: bool) {
        unsafe {
            ffi::gimp_config_writer_comment_mode(self.to_glib_none().0, enable.into_glib());
        }
    }

    /// Writes data to `self`.
    /// ## `data`
    /// The data to write
    #[doc(alias = "gimp_config_writer_data")]
    pub fn data(&self, data: &[u8]) {
        let length = data.len() as _;
        unsafe {
            ffi::gimp_config_writer_data(self.to_glib_none().0, length, data.to_glib_none().0);
        }
    }

    /// This function finishes the work of `self` and unrefs it
    /// afterwards. It closes all open elements, appends an optional
    /// comment and releases all resources allocated by `self`.
    ///
    /// Using any function except `gimp_config_writer_ref()` or
    /// `gimp_config_writer_unref()` after this function is forbidden
    /// and will trigger warnings.
    /// ## `footer`
    /// text to include as comment at the bottom of the file
    ///
    /// # Returns
    ///
    /// [`true`] if everything could be successfully written,
    ///  [`false`] otherwise
    #[doc(alias = "gimp_config_writer_finish")]
    pub fn finish(&self, footer: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gimp_config_writer_finish(self.to_glib_none().0, footer.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    /// Writes an identifier to `self`. The `string` is *not* quoted and special
    /// characters are *not* escaped.
    /// ## `identifier`
    /// a NUL-terminated string
    #[doc(alias = "gimp_config_writer_identifier")]
    pub fn identifier(&self, identifier: &str) {
        unsafe {
            ffi::gimp_config_writer_identifier(self.to_glib_none().0, identifier.to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_config_writer_linefeed")]
    pub fn linefeed(&self) {
        unsafe {
            ffi::gimp_config_writer_linefeed(self.to_glib_none().0);
        }
    }

    /// This function writes the opening parenthesis followed by `name`.
    /// It also increases the indentation level and sets a mark that
    /// can be used by [`revert()`][Self::revert()].
    /// ## `name`
    /// name of the element to open
    #[doc(alias = "gimp_config_writer_open")]
    pub fn open(&self, name: &str) {
        unsafe {
            ffi::gimp_config_writer_open(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    /// Appends a space followed by `string` to the `self`. Note that string
    /// must not contain any special characters that might need to be escaped.
    /// ## `string`
    /// a string to write
    /// ## `len`
    /// number of bytes from `string` or -1 if `string` is NUL-terminated.
    #[doc(alias = "gimp_config_writer_print")]
    pub fn print(&self, string: &str) {
        let len = string.len() as _;
        unsafe {
            ffi::gimp_config_writer_print(self.to_glib_none().0, string.to_glib_none().0, len);
        }
    }

    //#[doc(alias = "gimp_config_writer_printf")]
    //pub fn printf(&self, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gimp_config_writer_printf() }
    //}

    /// Reverts all changes to `self` that were done since the last call
    /// to [`open()`][Self::open()]. This can only work if you didn't call
    /// [`close()`][Self::close()] yet.
    #[doc(alias = "gimp_config_writer_revert")]
    pub fn revert(&self) {
        unsafe {
            ffi::gimp_config_writer_revert(self.to_glib_none().0);
        }
    }

    /// Writes a string value to `self`. The `string` is quoted and special
    /// characters are escaped.
    /// ## `string`
    /// a NUL-terminated string
    #[doc(alias = "gimp_config_writer_string")]
    pub fn string(&self, string: &str) {
        unsafe {
            ffi::gimp_config_writer_string(self.to_glib_none().0, string.to_glib_none().0);
        }
    }
}
