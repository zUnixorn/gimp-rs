use std::ffi::c_int;
use glib::object::IsA;
use glib::translate::ToGlibPtr;
use crate::{Procedure, ProcedureSensitivityMask};

pub trait ProcedureExtManual: 'static {
    fn set_sensitivity_mask(&self, sensitivity_mask: ProcedureSensitivityMask);
}

impl<O: IsA<Procedure>> ProcedureExtManual for O {
    /// Sets the case when `self` is supposed to be sensitive or not.
    /// Note that it will be used by the core to determine whether to show a
    /// procedure as sensitive (hence forbid running it otherwise), yet it
    /// will not forbid thid-party plug-ins for instance to run manually your
    /// registered procedure. Therefore you should still handle non-supported
    /// cases appropriately by returning with [`PDBStatusType::ExecutionError`][crate::PDBStatusType::ExecutionError] and a
    /// suitable error message.
    ///
    /// Similarly third-party plug-ins should verify they are allowed to call
    /// a procedure with [method`Procedure`] when running
    /// with dynamic contents.
    ///
    /// Note that by default, a procedure works on an image with a single
    /// drawable selected. Hence not setting the mask, setting it with 0 or
    /// setting it with a mask of [`ProcedureSensitivityMask::DRAWABLE`][crate::ProcedureSensitivityMask::DRAWABLE] only are
    /// equivalent.
    /// ## `sensitivity_mask`
    /// A binary mask of [`ProcedureSensitivityMask`][crate::ProcedureSensitivityMask].
    #[doc(alias = "gimp_procedure_set_sensitivity_mask")]
    fn set_sensitivity_mask(&self, sensitivity_mask: ProcedureSensitivityMask) {
        unsafe {
            ffi::gimp_procedure_set_sensitivity_mask(self.as_ref().to_glib_none().0, sensitivity_mask.bits() as c_int);
        }
    }
}
