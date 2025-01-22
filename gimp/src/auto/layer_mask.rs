// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Channel,Drawable,Item};
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "GimpLayerMask")]
    pub struct LayerMask(Object<ffi::GimpLayerMask, ffi::GimpLayerMaskClass>) @extends Channel, Drawable, Item;

    match fn {
        type_ => || ffi::gimp_layer_mask_get_type(),
    }
}

impl LayerMask {
    #[doc(alias = "gimp_layer_mask_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(layer_mask_id: i32) -> Option<LayerMask> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_layer_mask_get_by_id(layer_mask_id))
        }
    }
}
