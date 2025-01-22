// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,PDBErrorHandler,Procedure};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "GimpPlugIn")]
    pub struct PlugIn(Object<ffi::GimpPlugIn, ffi::GimpPlugInClass>);

    match fn {
        type_ => || ffi::gimp_plug_in_get_type(),
    }
}

impl PlugIn {
        pub const NONE: Option<&'static PlugIn> = None;
    

    #[doc(alias = "gimp_plug_in_directory")]
    pub fn directory() -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_plug_in_directory())
        }
    }

    //#[doc(alias = "gimp_plug_in_directory_file")]
    //pub fn directory_file(first_element: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<gio::File> {
    //    unsafe { TODO: call ffi:gimp_plug_in_directory_file() }
    //}

    #[doc(alias = "gimp_plug_in_error_quark")]
    pub fn error_quark() -> glib::Quark {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gimp_plug_in_error_quark())
        }
    }
}

pub trait PlugInExt: IsA<PlugIn> + 'static {
    #[doc(alias = "gimp_plug_in_add_menu_branch")]
    fn add_menu_branch(&self, menu_path: &str, menu_label: &str) {
        unsafe {
            ffi::gimp_plug_in_add_menu_branch(self.as_ref().to_glib_none().0, menu_path.to_glib_none().0, menu_label.to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_plug_in_add_temp_procedure")]
    fn add_temp_procedure(&self, procedure: &impl IsA<Procedure>) {
        unsafe {
            ffi::gimp_plug_in_add_temp_procedure(self.as_ref().to_glib_none().0, procedure.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_plug_in_get_pdb_error_handler")]
    #[doc(alias = "get_pdb_error_handler")]
    fn pdb_error_handler(&self) -> PDBErrorHandler {
        unsafe {
            from_glib(ffi::gimp_plug_in_get_pdb_error_handler(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_plug_in_get_temp_procedure")]
    #[doc(alias = "get_temp_procedure")]
    fn temp_procedure(&self, procedure_name: &str) -> Option<Procedure> {
        unsafe {
            from_glib_none(ffi::gimp_plug_in_get_temp_procedure(self.as_ref().to_glib_none().0, procedure_name.to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_plug_in_get_temp_procedures")]
    #[doc(alias = "get_temp_procedures")]
    fn temp_procedures(&self) -> Vec<Procedure> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gimp_plug_in_get_temp_procedures(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gimp_plug_in_persistent_enable")]
    fn persistent_enable(&self) {
        unsafe {
            ffi::gimp_plug_in_persistent_enable(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_plug_in_persistent_process")]
    fn persistent_process(&self, timeout: u32) {
        unsafe {
            ffi::gimp_plug_in_persistent_process(self.as_ref().to_glib_none().0, timeout);
        }
    }

    #[doc(alias = "gimp_plug_in_remove_temp_procedure")]
    fn remove_temp_procedure(&self, procedure_name: &str) {
        unsafe {
            ffi::gimp_plug_in_remove_temp_procedure(self.as_ref().to_glib_none().0, procedure_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_plug_in_set_help_domain")]
    fn set_help_domain(&self, domain_name: &str, domain_uri: &impl IsA<gio::File>) {
        unsafe {
            ffi::gimp_plug_in_set_help_domain(self.as_ref().to_glib_none().0, domain_name.to_glib_none().0, domain_uri.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gimp_plug_in_set_pdb_error_handler")]
    fn set_pdb_error_handler(&self, handler: PDBErrorHandler) {
        unsafe {
            ffi::gimp_plug_in_set_pdb_error_handler(self.as_ref().to_glib_none().0, handler.into_glib());
        }
    }
}

impl<O: IsA<PlugIn>> PlugInExt for O {}
