use std::ffi::c_int;
use glib::object::IsA;
use glib::translate::ToGlibPtr;
use crate::{Procedure, ProcedureSensitivityMask};

pub trait ProcedureExtManual: 'static {
    fn set_sensitivity_mask(&self, sensitivity_mask: ProcedureSensitivityMask);
}

impl<O: IsA<Procedure>> ProcedureExtManual for O {
    #[doc(alias = "gimp_procedure_set_sensitivity_mask")]
    fn set_sensitivity_mask(&self, sensitivity_mask: ProcedureSensitivityMask) {
        unsafe {
            ffi::gimp_procedure_set_sensitivity_mask(self.as_ref().to_glib_none().0, sensitivity_mask.bits() as c_int);
        }
    }
}
