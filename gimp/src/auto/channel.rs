// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,ChannelOps,ChannelType,Drawable,Image,Item};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    /// Functions for manipulating channels.
    ///
    /// # Implements
    ///
    /// [`ChannelExt`][trait@crate::prelude::ChannelExt], [`DrawableExt`][trait@crate::prelude::DrawableExt], [`ItemExt`][trait@crate::prelude::ItemExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "GimpChannel")]
    pub struct Channel(Object<ffi::GimpChannel, ffi::GimpChannelClass>) @extends Drawable, Item;

    match fn {
        type_ => || ffi::gimp_channel_get_type(),
    }
}

impl Channel {
        pub const NONE: Option<&'static Channel> = None;
    

    /// Create a new channel.
    ///
    /// This procedure creates a new channel with the specified `width`,
    /// `height`, `name`, `opacity` and `color`.
    ///
    /// Other attributes, such as channel visibility, should be set with
    /// explicit procedure calls.
    ///
    /// The new channel still needs to be added to the image, as this is not
    /// automatic. Add the new channel with
    /// [method`Gimp`.insert_channel].
    ///
    /// The channel's contents are undefined initially.
    /// ## `image`
    /// The image to which to add the channel.
    /// ## `name`
    /// The channel name.
    /// ## `width`
    /// The channel width.
    /// ## `height`
    /// The channel height.
    /// ## `opacity`
    /// The channel opacity.
    /// ## `color`
    /// The channel compositing color.
    ///
    /// # Returns
    ///
    /// The newly created channel.
    #[doc(alias = "gimp_channel_new")]
    pub fn new(image: &Image, name: &str, width: i32, height: i32, opacity: f64, color: &impl IsA<gegl::Color>) -> Channel {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_channel_new(image.to_glib_none().0, name.to_glib_none().0, width, height, opacity, color.as_ref().to_glib_none().0))
        }
    }

    /// Create a new channel from a color component
    ///
    /// This procedure creates a new channel from a color component.
    /// The new channel still needs to be added to the image, as this is not
    /// automatic. Add the new channel with [`Image::insert_channel()`][crate::Image::insert_channel()].
    /// Other attributes, such as channel visibility, should be set with
    /// explicit procedure calls.
    /// ## `image`
    /// The image to which to add the channel.
    /// ## `component`
    /// The image component.
    /// ## `name`
    /// The channel name.
    ///
    /// # Returns
    ///
    /// The newly created channel.
    #[doc(alias = "gimp_channel_new_from_component")]
    #[doc(alias = "new_from_component")]
    pub fn from_component(image: &Image, component: ChannelType, name: &str) -> Channel {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_channel_new_from_component(image.to_glib_none().0, component.into_glib(), name.to_glib_none().0))
        }
    }

    /// Returns a [`Channel`][crate::Channel] representing `channel_id`. This function
    /// calls [`Item::by_id()`][crate::Item::by_id()] and returns the item if it is channel
    /// or [`None`] otherwise.
    /// ## `channel_id`
    /// The channel id.
    ///
    /// # Returns
    ///
    /// a [`Channel`][crate::Channel] for `channel_id`
    ///  or [`None`] if `channel_id` does not represent a valid
    ///  channel. The object belongs to libgimp and you must not
    ///  modify or unref it.
    #[doc(alias = "gimp_channel_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(channel_id: i32) -> Option<Channel> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_channel_get_by_id(channel_id))
        }
    }
}

/// Trait containing all [`struct@Channel`] methods.
///
/// # Implementors
///
/// [`Channel`][struct@crate::Channel], [`LayerMask`][struct@crate::LayerMask], [`Selection`][struct@crate::Selection]
pub trait ChannelExt: IsA<Channel> + 'static {
    /// Combine two channel masks.
    ///
    /// This procedure combines two channel masks. The result is stored in
    /// the first channel.
    /// ## `channel2`
    /// The channel2.
    /// ## `operation`
    /// The selection operation.
    /// ## `offx`
    /// x offset between upper left corner of channels: (second - first).
    /// ## `offy`
    /// y offset between upper left corner of channels: (second - first).
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_channel_combine_masks")]
    fn combine_masks(&self, channel2: &impl IsA<Channel>, operation: ChannelOps, offx: i32, offy: i32) -> bool {
        unsafe {
            from_glib(ffi::gimp_channel_combine_masks(self.as_ref().to_glib_none().0, channel2.as_ref().to_glib_none().0, operation.into_glib(), offx, offy))
        }
    }

    #[doc(alias = "gimp_channel_copy")]
#[must_use]
    fn copy(&self) -> Option<Channel> {
        unsafe {
            from_glib_none(ffi::gimp_channel_copy(self.as_ref().to_glib_none().0))
        }
    }

    /// Get the compositing color of the specified channel.
    ///
    /// This procedure returns the specified channel's compositing color.
    ///
    /// # Returns
    ///
    /// The channel compositing color.
    #[doc(alias = "gimp_channel_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> Option<gegl::Color> {
        unsafe {
            from_glib_full(ffi::gimp_channel_get_color(self.as_ref().to_glib_none().0))
        }
    }

    /// Get the opacity of the specified channel.
    ///
    /// This procedure returns the specified channel's opacity.
    ///
    /// # Returns
    ///
    /// The channel opacity.
    #[doc(alias = "gimp_channel_get_opacity")]
    #[doc(alias = "get_opacity")]
    fn opacity(&self) -> f64 {
        unsafe {
            ffi::gimp_channel_get_opacity(self.as_ref().to_glib_none().0)
        }
    }

    /// Get the composite method of the specified channel.
    ///
    /// This procedure returns the specified channel's composite method. If
    /// it is TRUE, then the channel is composited with the image so that
    /// masked regions are shown. Otherwise, selected regions are shown.
    ///
    /// # Returns
    ///
    /// The channel composite method.
    #[doc(alias = "gimp_channel_get_show_masked")]
    #[doc(alias = "get_show_masked")]
    fn shows_masked(&self) -> bool {
        unsafe {
            from_glib(ffi::gimp_channel_get_show_masked(self.as_ref().to_glib_none().0))
        }
    }

    /// Set the compositing color of the specified channel.
    ///
    /// This procedure sets the specified channel's compositing color.
    /// ## `color`
    /// The new channel compositing color.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_channel_set_color")]
    fn set_color(&self, color: &impl IsA<gegl::Color>) -> bool {
        unsafe {
            from_glib(ffi::gimp_channel_set_color(self.as_ref().to_glib_none().0, color.as_ref().to_glib_none().0))
        }
    }

    /// Set the opacity of the specified channel.
    ///
    /// This procedure sets the specified channel's opacity.
    /// ## `opacity`
    /// The new channel opacity.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_channel_set_opacity")]
    fn set_opacity(&self, opacity: f64) -> bool {
        unsafe {
            from_glib(ffi::gimp_channel_set_opacity(self.as_ref().to_glib_none().0, opacity))
        }
    }

    /// Set the composite method of the specified channel.
    ///
    /// This procedure sets the specified channel's composite method. If it
    /// is TRUE, then the channel is composited with the image so that
    /// masked regions are shown. Otherwise, selected regions are shown.
    /// ## `show_masked`
    /// The new channel composite method.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_channel_set_show_masked")]
    fn set_show_masked(&self, show_masked: bool) -> bool {
        unsafe {
            from_glib(ffi::gimp_channel_set_show_masked(self.as_ref().to_glib_none().0, show_masked.into_glib()))
        }
    }
}

impl<O: IsA<Channel>> ChannelExt for O {}
