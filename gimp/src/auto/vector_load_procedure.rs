// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,FileProcedure,LoadProcedure,Procedure};

glib::wrapper! {
    /// A [class`Procedure`] subclass that makes it easier to write load procedures
    /// for vector image formats.
    ///
    /// It automatically adds the standard arguments:
    /// ([enum`RunMode`], [iface`Gio`], int width, int height)
    ///
    /// and the standard return value: ( [class`Image`] )
    ///
    /// It is possible to add additional arguments.
    ///
    /// When invoked via [method`Procedure`], it unpacks these standard
    /// arguments and calls `run_func` which is a [callback`RunImageFunc`]. The
    /// [class`ProcedureConfig`] of [callback`Gimp`] contains
    /// additionally added arguments but also the arguments added by this class.
    ///
    /// # Implements
    ///
    /// [`LoadProcedureExt`][trait@crate::prelude::LoadProcedureExt], [`FileProcedureExt`][trait@crate::prelude::FileProcedureExt], [`ProcedureExt`][trait@crate::prelude::ProcedureExt], [`trait@glib::ObjectExt`], [`ProcedureExtManual`][trait@crate::prelude::ProcedureExtManual]
    #[doc(alias = "GimpVectorLoadProcedure")]
    pub struct VectorLoadProcedure(Object<ffi::GimpVectorLoadProcedure, ffi::GimpVectorLoadProcedureClass>) @extends LoadProcedure, FileProcedure, Procedure;

    match fn {
        type_ => || ffi::gimp_vector_load_procedure_get_type(),
    }
}

impl VectorLoadProcedure {
    //#[doc(alias = "gimp_vector_load_procedure_new")]
    //pub fn new(plug_in: &impl IsA<PlugIn>, name: &str, proc_type: PDBProcType, extract_func: /*Unimplemented*/Fn(&Procedure, &RunMode, &gio::File, &Metadata, Option<&ProcedureConfig>, /*Ignored*/VectorLoadData, /*Unimplemented*/Option<Basic: Pointer>, Fn() + 'static, Option<&glib::Error>) -> bool, extract_data: /*Unimplemented*/Option<Basic: Pointer>, run_func: /*Unimplemented*/Fn(&Procedure, &RunMode, &gio::File, i32, i32, /*Ignored*/VectorLoadData, &Metadata, &MetadataLoadFlags, &ProcedureConfig) -> ValueArray, run_data: /*Unimplemented*/Option<Basic: Pointer>) -> VectorLoadProcedure {
    //    unsafe { TODO: call ffi:gimp_vector_load_procedure_new() }
    //}

    //#[doc(alias = "gimp_vector_load_procedure_extract_dimensions")]
    //pub fn extract_dimensions(&self, file: &impl IsA<gio::File>, data: /*Ignored*/VectorLoadData) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:gimp_vector_load_procedure_extract_dimensions() }
    //}
}
