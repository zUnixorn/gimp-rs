// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,ExportCapabilities};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    /// A class holding generic export options.
    ///
    /// Note: right now, GIMP does not provide any generic export option to
    /// manipulate, and there is practically no reason for you to create this
    /// object yourself. In Export PDB procedure, or again in functions such
    /// as [func`Gimp`], you may just pass [`None`].
    ///
    /// In the future, this object will enable to pass various generic
    /// options, such as ability to crop or resize images at export time.
    ///
    /// ## Properties
    ///
    ///
    /// #### `capabilities`
    ///  What [flags`ExportCapabilities`] are supported.
    ///
    /// Readable | Writeable | Construct
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpExportOptions")]
    pub struct ExportOptions(Object<ffi::GimpExportOptions, ffi::GimpExportOptionsClass>);

    match fn {
        type_ => || ffi::gimp_export_options_get_type(),
    }
}

impl ExportOptions {
    //#[doc(alias = "gimp_export_options_get_image")]
    //#[doc(alias = "get_image")]
    //pub fn image(&self, image: /*Unimplemented*/Image) -> ExportReturn {
    //    unsafe { TODO: call ffi:gimp_export_options_get_image() }
    //}

    /// What [flags`ExportCapabilities`] are supported.
    pub fn capabilities(&self) -> ExportCapabilities {
        ObjectExt::property(self, "capabilities")
    }

    /// What [flags`ExportCapabilities`] are supported.
    pub fn set_capabilities(&self, capabilities: ExportCapabilities) {
        ObjectExt::set_property(self,"capabilities", capabilities)
    }

    #[doc(alias = "capabilities")]
    pub fn connect_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_capabilities_trampoline<F: Fn(&ExportOptions) + 'static>(this: *mut ffi::GimpExportOptions, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::capabilities".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_capabilities_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
