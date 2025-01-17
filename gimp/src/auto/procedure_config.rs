// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Image,Procedure};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    /// The base class for [class`Procedure`] specific config objects and the main
    /// interface to manage aspects of [class`Procedure`]'s arguments such as
    /// persistency of the last used arguments across GIMP sessions.
    ///
    /// A procedure config is created by a [class`Procedure`] using
    /// [method`Procedure`] and its properties match the
    /// procedure's arguments and auxiliary arguments in number, order and
    /// type.
    ///
    /// It implements the [struct`Config`] interface and therefore has all its
    /// serialization and deserialization features.
    ///
    /// This is an Abstract Base Class, you cannot instantiate it.
    ///
    /// ## Properties
    ///
    ///
    /// #### `procedure`
    ///  Readable | Writeable | Construct Only
    ///
    /// # Implements
    ///
    /// [`ProcedureConfigExt`][trait@crate::prelude::ProcedureConfigExt]
    #[doc(alias = "GimpProcedureConfig")]
    pub struct ProcedureConfig(Object<ffi::GimpProcedureConfig, ffi::GimpProcedureConfigClass>);

    match fn {
        type_ => || ffi::gimp_procedure_config_get_type(),
    }
}

impl ProcedureConfig {
        pub const NONE: Option<&'static ProcedureConfig> = None;
    
}

/// Trait containing all [`struct@ProcedureConfig`] methods.
///
/// # Implementors
///
/// [`ProcedureConfig`][struct@crate::ProcedureConfig]
pub trait ProcedureConfigExt: IsA<ProcedureConfig> + 'static {
    /// A utility function which will get the current string value of a
    /// [struct`ParamSpecChoice`] property in `self` and convert it to the integer ID
    /// mapped to this value.
    /// This makes it easy to work with an Enum type locally, within a plug-in code.
    /// ## `property_name`
    /// the name of a [struct`ParamSpecChoice`] property.
    #[doc(alias = "gimp_procedure_config_get_choice_id")]
    #[doc(alias = "get_choice_id")]
    fn choice_id(&self, property_name: &str) -> i32 {
        unsafe {
            ffi::gimp_procedure_config_get_choice_id(self.as_ref().to_glib_none().0, property_name.to_glib_none().0)
        }
    }

    /// A function for bindings to get a [type`ColorArray`] property. Getting
    /// these with [method`GObject`.get] or [method`GObject`.get_property] won't
    /// [work for the time being](https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/492)
    /// so all our boxed array types must be set and get using these
    /// alternative functions instead.
    ///
    /// C plug-ins should just use [method`GObject`.get].
    /// ## `property_name`
    /// the name of a [struct`ParamSpecCoreObjectArray`] param spec.
    ///
    /// # Returns
    ///
    /// an array of `GObjects`.
    #[doc(alias = "gimp_procedure_config_get_color_array")]
    #[doc(alias = "get_color_array")]
    fn color_array(&self, property_name: &str) -> Vec<gegl::Color> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gimp_procedure_config_get_color_array(self.as_ref().to_glib_none().0, property_name.to_glib_none().0))
        }
    }

    //#[doc(alias = "gimp_procedure_config_get_core_object_array")]
    //#[doc(alias = "get_core_object_array")]
    //fn core_object_array(&self, property_name: &str) -> /*Ignored*/Vec<glib::Object> {
    //    unsafe { TODO: call ffi:gimp_procedure_config_get_core_object_array() }
    //}

    /// This function returns the [class`Procedure`] which created `self`, see
    /// [method`Procedure`].
    ///
    /// # Returns
    ///
    /// The procedure which created this config.
    #[doc(alias = "gimp_procedure_config_get_procedure")]
    #[doc(alias = "get_procedure")]
    fn procedure(&self) -> Option<Procedure> {
        unsafe {
            from_glib_none(ffi::gimp_procedure_config_get_procedure(self.as_ref().to_glib_none().0))
        }
    }

    /// *Note: There is normally no need to call this function because it's
    /// already called by [class`ExportProcedure`] after the ``run()`` callback.*
    ///
    /// *Only use this function if the [class`Metadata`] passed as argument of a
    /// [class`ExportProcedure`]'s `run()` method needs to be written at a specific
    /// point of the export, other than its end.*
    ///
    /// This function syncs back `self`'s export properties to the
    /// metadata's [flags`MetadataSaveFlags`] and writes the metadata to
    /// `file`.
    ///
    /// The metadata is only ever written once. If this function has been
    /// called explicitly, it will do nothing when called a second time at the end of
    /// the ``run()`` callback.
    /// ## `exported_image`
    /// the image that was actually exported
    /// ## `file`
    /// the file `exported_image` was written to
    #[doc(alias = "gimp_procedure_config_save_metadata")]
    fn save_metadata(&self, exported_image: &Image, file: &impl IsA<gio::File>) {
        unsafe {
            ffi::gimp_procedure_config_save_metadata(self.as_ref().to_glib_none().0, exported_image.to_glib_none().0, file.as_ref().to_glib_none().0);
        }
    }

    /// A function for bindings to set a [type`ColorArray`] property. Setting
    /// these with [method`GObject`.set] or [method`GObject`.set_property] won't
    /// [work for the time being](https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/492)
    /// so all our boxed array types must be set and get using these
    /// alternative functions instead.
    ///
    /// C plug-ins should just use [method`GObject`.set].
    /// ## `property_name`
    /// the name of a [struct`ParamSpecCoreObjectArray`] param spec.
    /// ## `colors`
    /// an array of [class`Gegl`].
    #[doc(alias = "gimp_procedure_config_set_color_array")]
    fn set_color_array(&self, property_name: &str, colors: &[gegl::Color]) {
        let n_colors = colors.len() as _;
        unsafe {
            ffi::gimp_procedure_config_set_color_array(self.as_ref().to_glib_none().0, property_name.to_glib_none().0, colors.to_glib_none().0, n_colors);
        }
    }

    //#[doc(alias = "gimp_procedure_config_set_core_object_array")]
    //fn set_core_object_array(&self, property_name: &str, objects: /*Ignored*/&[glib::Object]) {
    //    unsafe { TODO: call ffi:gimp_procedure_config_set_core_object_array() }
    //}
}

impl<O: IsA<ProcedureConfig>> ProcedureConfigExt for O {}
