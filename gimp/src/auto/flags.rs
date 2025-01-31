// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{bitflags::bitflags,prelude::*,translate::*};

bitflags! {
    /// The types of images and layers an export procedure can handle
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GimpExportCapabilities")]
    pub struct ExportCapabilities: u32 {
        /// Handles RGB images
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_RGB")]
        const CAN_HANDLE_RGB = ffi::GIMP_EXPORT_CAN_HANDLE_RGB as _;
        /// Handles grayscale images
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_GRAY")]
        const CAN_HANDLE_GRAY = ffi::GIMP_EXPORT_CAN_HANDLE_GRAY as _;
        /// Handles indexed images
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_INDEXED")]
        const CAN_HANDLE_INDEXED = ffi::GIMP_EXPORT_CAN_HANDLE_INDEXED as _;
        /// Handles two-color indexed images
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_BITMAP")]
        const CAN_HANDLE_BITMAP = ffi::GIMP_EXPORT_CAN_HANDLE_BITMAP as _;
        /// Handles alpha channels
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_ALPHA")]
        const CAN_HANDLE_ALPHA = ffi::GIMP_EXPORT_CAN_HANDLE_ALPHA as _;
        /// Handles layers
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_LAYERS")]
        const CAN_HANDLE_LAYERS = ffi::GIMP_EXPORT_CAN_HANDLE_LAYERS as _;
        /// Handles animation of layers
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_LAYERS_AS_ANIMATION")]
        const CAN_HANDLE_LAYERS_AS_ANIMATION = ffi::GIMP_EXPORT_CAN_HANDLE_LAYERS_AS_ANIMATION as _;
        /// Handles layer masks
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_LAYER_MASKS")]
        const CAN_HANDLE_LAYER_MASKS = ffi::GIMP_EXPORT_CAN_HANDLE_LAYER_MASKS as _;
        /// Handles layer effects
        #[doc(alias = "GIMP_EXPORT_CAN_HANDLE_LAYER_EFFECTS")]
        const CAN_HANDLE_LAYER_EFFECTS = ffi::GIMP_EXPORT_CAN_HANDLE_LAYER_EFFECTS as _;
        /// Needs alpha channels
        #[doc(alias = "GIMP_EXPORT_NEEDS_ALPHA")]
        const NEEDS_ALPHA = ffi::GIMP_EXPORT_NEEDS_ALPHA as _;
        /// Needs to crop content to image bounds
        #[doc(alias = "GIMP_EXPORT_NEEDS_CROP")]
        const NEEDS_CROP = ffi::GIMP_EXPORT_NEEDS_CROP as _;
    }
}

#[doc(hidden)]
impl IntoGlib for ExportCapabilities {
    type GlibType = ffi::GimpExportCapabilities;

    #[inline]
    fn into_glib(self) -> ffi::GimpExportCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GimpExportCapabilities> for ExportCapabilities {
    #[inline]
    unsafe fn from_glib(value: ffi::GimpExportCapabilities) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ExportCapabilities {
                #[inline]
    #[doc(alias = "gimp_export_capabilities_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::gimp_export_capabilities_get_type()) }
                }
            }

impl glib::HasParamSpec for ExportCapabilities {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder
                }
}

impl glib::value::ValueType for ExportCapabilities {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for ExportCapabilities {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ExportCapabilities {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ExportCapabilities> for glib::Value {
    #[inline]
    fn from(v: ExportCapabilities) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    /// What metadata to load when importing images.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GimpMetadataLoadFlags")]
    pub struct MetadataLoadFlags: u32 {
        /// Do not load the metadata
        #[doc(alias = "GIMP_METADATA_LOAD_NONE")]
        const NONE = ffi::GIMP_METADATA_LOAD_NONE as _;
        /// Load the comment
        #[doc(alias = "GIMP_METADATA_LOAD_COMMENT")]
        const COMMENT = ffi::GIMP_METADATA_LOAD_COMMENT as _;
        /// Load the resolution
        #[doc(alias = "GIMP_METADATA_LOAD_RESOLUTION")]
        const RESOLUTION = ffi::GIMP_METADATA_LOAD_RESOLUTION as _;
        /// Load the orientation (rotation)
        #[doc(alias = "GIMP_METADATA_LOAD_ORIENTATION")]
        const ORIENTATION = ffi::GIMP_METADATA_LOAD_ORIENTATION as _;
        /// Load the colorspace
        #[doc(alias = "GIMP_METADATA_LOAD_COLORSPACE")]
        const COLORSPACE = ffi::GIMP_METADATA_LOAD_COLORSPACE as _;
        /// Load all of the above
        #[doc(alias = "GIMP_METADATA_LOAD_ALL")]
        const ALL = ffi::GIMP_METADATA_LOAD_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for MetadataLoadFlags {
    type GlibType = ffi::GimpMetadataLoadFlags;

    #[inline]
    fn into_glib(self) -> ffi::GimpMetadataLoadFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GimpMetadataLoadFlags> for MetadataLoadFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GimpMetadataLoadFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    /// What kinds of metadata to save when exporting images.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GimpMetadataSaveFlags")]
    pub struct MetadataSaveFlags: u32 {
        /// Save EXIF
        #[doc(alias = "GIMP_METADATA_SAVE_EXIF")]
        const EXIF = ffi::GIMP_METADATA_SAVE_EXIF as _;
        /// Save XMP
        #[doc(alias = "GIMP_METADATA_SAVE_XMP")]
        const XMP = ffi::GIMP_METADATA_SAVE_XMP as _;
        /// Save IPTC
        #[doc(alias = "GIMP_METADATA_SAVE_IPTC")]
        const IPTC = ffi::GIMP_METADATA_SAVE_IPTC as _;
        /// Save a thumbnail of the image
        #[doc(alias = "GIMP_METADATA_SAVE_THUMBNAIL")]
        const THUMBNAIL = ffi::GIMP_METADATA_SAVE_THUMBNAIL as _;
        /// Save the image's color profile
        ///  Since: 2.10.10
        #[doc(alias = "GIMP_METADATA_SAVE_COLOR_PROFILE")]
        const COLOR_PROFILE = ffi::GIMP_METADATA_SAVE_COLOR_PROFILE as _;
        /// Save the image's comment
        ///  Since: 3.0
        #[doc(alias = "GIMP_METADATA_SAVE_COMMENT")]
        const COMMENT = ffi::GIMP_METADATA_SAVE_COMMENT as _;
        /// Save all of the above
        #[doc(alias = "GIMP_METADATA_SAVE_ALL")]
        const ALL = ffi::GIMP_METADATA_SAVE_ALL as _;
    }
}

#[doc(hidden)]
impl IntoGlib for MetadataSaveFlags {
    type GlibType = ffi::GimpMetadataSaveFlags;

    #[inline]
    fn into_glib(self) -> ffi::GimpMetadataSaveFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GimpMetadataSaveFlags> for MetadataSaveFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GimpMetadataSaveFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    /// The cases when a [`Procedure`][crate::Procedure] should be shown as sensitive.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GimpProcedureSensitivityMask")]
    pub struct ProcedureSensitivityMask: u32 {
        /// Handles image with one selected drawable.
        #[doc(alias = "GIMP_PROCEDURE_SENSITIVE_DRAWABLE")]
        const DRAWABLE = ffi::GIMP_PROCEDURE_SENSITIVE_DRAWABLE as _;
        /// Handles image with several selected drawables.
        #[doc(alias = "GIMP_PROCEDURE_SENSITIVE_DRAWABLES")]
        const DRAWABLES = ffi::GIMP_PROCEDURE_SENSITIVE_DRAWABLES as _;
        /// Handles image with no selected drawables.
        #[doc(alias = "GIMP_PROCEDURE_SENSITIVE_NO_DRAWABLES")]
        const NO_DRAWABLES = ffi::GIMP_PROCEDURE_SENSITIVE_NO_DRAWABLES as _;
        /// Handles no image.
        #[doc(alias = "GIMP_PROCEDURE_SENSITIVE_NO_IMAGE")]
        const NO_IMAGE = ffi::GIMP_PROCEDURE_SENSITIVE_NO_IMAGE as _;
        #[doc(alias = "GIMP_PROCEDURE_SENSITIVE_ALWAYS")]
        const ALWAYS = ffi::GIMP_PROCEDURE_SENSITIVE_ALWAYS as _;
    }
}

#[doc(hidden)]
impl IntoGlib for ProcedureSensitivityMask {
    type GlibType = ffi::GimpProcedureSensitivityMask;

    #[inline]
    fn into_glib(self) -> ffi::GimpProcedureSensitivityMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GimpProcedureSensitivityMask> for ProcedureSensitivityMask {
    #[inline]
    unsafe fn from_glib(value: ffi::GimpProcedureSensitivityMask) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ProcedureSensitivityMask {
                #[inline]
    #[doc(alias = "gimp_procedure_sensitivity_mask_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::gimp_procedure_sensitivity_mask_get_type()) }
                }
            }

impl glib::HasParamSpec for ProcedureSensitivityMask {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder
                }
}

impl glib::value::ValueType for ProcedureSensitivityMask {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for ProcedureSensitivityMask {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ProcedureSensitivityMask {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ProcedureSensitivityMask> for glib::Value {
    #[inline]
    fn from(v: ProcedureSensitivityMask) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

