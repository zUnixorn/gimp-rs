use crate::{ffi, Drawable, Image, ImageProcedure, PDBProcType, PlugIn, Procedure, ProcedureConfig, RunMode, ValueArray};
use glib::{prelude::*,translate::*};
use std::{boxed::Box as Box_};

impl ImageProcedure {
    #[doc(alias = "gimp_image_procedure_new")]
    pub fn new<P: Fn(&Procedure, RunMode, &Image, &Vec<Drawable>, &ProcedureConfig) -> ValueArray + 'static>(plug_in: &impl IsA<PlugIn>, name: &str, proc_type: PDBProcType, run_func: P) -> ImageProcedure {
        skip_assert_initialized!();
        let run_func_data: Box_<P> = Box_::new(run_func);
        unsafe extern "C" fn run_func_func<P: Fn(&Procedure, RunMode, &Image, &Vec<Drawable>, &ProcedureConfig) -> ValueArray + 'static>(procedure: *mut ffi::GimpProcedure, run_mode: ffi::GimpRunMode, image: *mut ffi::GimpImage, drawables: *mut *mut ffi::GimpDrawable, config: *mut ffi::GimpProcedureConfig, run_data: glib::ffi::gpointer) -> *mut ffi::GimpValueArray {
            let procedure = from_glib_borrow(procedure);
            let run_mode = from_glib(run_mode);
            let image = from_glib_borrow(image);
            let drawables = FromGlibPtrContainer::from_glib_none(drawables);
            let config = from_glib_borrow(config);
            let callback = &*(run_data as *mut P);
            (*callback)(&procedure, run_mode, &image, &drawables, &config)
                .to_glib_full()
        }
        let run_func = Some(run_func_func::<P> as _);
        unsafe extern "C" fn run_data_destroy_func<P: Fn(&Procedure, RunMode, &Image, &Vec<Drawable>, &ProcedureConfig) -> ValueArray + 'static>(data: glib::ffi::gpointer) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call5 = Some(run_data_destroy_func::<P> as _);
        let super_callback0: Box_<P> = run_func_data;
        unsafe {
            Procedure::from_glib_full(ffi::gimp_image_procedure_new(plug_in.as_ref().to_glib_none().0, name.to_glib_none().0, proc_type.into_glib(), run_func, Box_::into_raw(super_callback0) as *mut _, destroy_call5)).unsafe_cast()
        }
    }
}