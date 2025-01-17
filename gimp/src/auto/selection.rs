// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Channel,Drawable,Image,Item,Layer};
use glib::{translate::*};

glib::wrapper! {
    /// Functions for manipulating selections.
    ///
    /// # Implements
    ///
    /// [`ChannelExt`][trait@crate::prelude::ChannelExt], [`DrawableExt`][trait@crate::prelude::DrawableExt], [`ItemExt`][trait@crate::prelude::ItemExt]
    #[doc(alias = "GimpSelection")]
    pub struct Selection(Object<ffi::GimpSelection, ffi::GimpSelectionClass>) @extends Channel, Drawable, Item;

    match fn {
        type_ => || ffi::gimp_selection_get_type(),
    }
}

impl Selection {
    /// Select all of the image.
    ///
    /// This procedure sets the selection mask to completely encompass the
    /// image. Every pixel in the selection channel is set to 255.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_all")]
    pub fn all(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_all(image.to_glib_none().0))
        }
    }

    /// Border the image's selection
    ///
    /// This procedure borders the selection. Bordering creates a new
    /// selection which is defined along the boundary of the previous
    /// selection at every point within the specified radius.
    /// ## `image`
    /// The image.
    /// ## `radius`
    /// Radius of border (in pixels).
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_border")]
    pub fn border(image: &Image, radius: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_border(image.to_glib_none().0, radius))
        }
    }

    /// Find the bounding box of the current selection.
    ///
    /// This procedure returns whether there is a selection for the
    /// specified image. If there is one, the upper left and lower right
    /// corners of the bounding box are returned. These coordinates are
    /// relative to the image. Please note that the pixel specified by the
    /// lower right coordinate of the bounding box is not part of the
    /// selection. The selection ends at the upper left corner of this
    /// pixel. This means the width of the selection can be calculated as
    /// (x2 - x1), its height as (y2 - y1).
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    ///
    /// ## `non_empty`
    /// TRUE if there is a selection.
    ///
    /// ## `x1`
    /// x coordinate of upper left corner of selection bounds.
    ///
    /// ## `y1`
    /// y coordinate of upper left corner of selection bounds.
    ///
    /// ## `x2`
    /// x coordinate of lower right corner of selection bounds.
    ///
    /// ## `y2`
    /// y coordinate of lower right corner of selection bounds.
    #[doc(alias = "gimp_selection_bounds")]
    pub fn bounds(image: &Image) -> Option<(bool, i32, i32, i32, i32)> {
        skip_assert_initialized!();
        unsafe {
            let mut non_empty = std::mem::MaybeUninit::uninit();
            let mut x1 = std::mem::MaybeUninit::uninit();
            let mut y1 = std::mem::MaybeUninit::uninit();
            let mut x2 = std::mem::MaybeUninit::uninit();
            let mut y2 = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gimp_selection_bounds(image.to_glib_none().0, non_empty.as_mut_ptr(), x1.as_mut_ptr(), y1.as_mut_ptr(), x2.as_mut_ptr(), y2.as_mut_ptr()));
            if ret { Some((from_glib(non_empty.assume_init()), x1.assume_init(), y1.assume_init(), x2.assume_init(), y2.assume_init())) } else { None }
        }
    }

    /// Feather the image's selection
    ///
    /// This procedure feathers the selection. Feathering is implemented
    /// using a gaussian blur.
    /// ## `image`
    /// The image.
    /// ## `radius`
    /// Radius of feather (in pixels).
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_feather")]
    pub fn feather(image: &Image, radius: f64) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_feather(image.to_glib_none().0, radius))
        }
    }

    /// Float the selection from the specified drawable with initial offsets
    /// as specified.
    ///
    /// This procedure determines the region of the specified drawable that
    /// lies beneath the current selection. The region is then cut from the
    /// drawable and the resulting data is made into a new layer which is
    /// instantiated as a floating selection. The offsets allow initial
    /// positioning of the new floating selection.
    /// ## `image`
    /// ignored
    /// ## `drawables`
    /// The drawables from which to
    ///  float selection.
    /// ## `offx`
    /// x offset for translation.
    /// ## `offy`
    /// y offset for translation.
    ///
    /// # Returns
    ///
    /// The floated layer.
    #[doc(alias = "gimp_selection_float")]
    pub fn float(image: &Image, drawables: &[Drawable], offx: i32, offy: i32) -> Option<Layer> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_selection_float(image.to_glib_none().0, drawables.to_glib_none().0, offx, offy))
        }
    }

    /// Remove holes from the image's selection
    ///
    /// This procedure removes holes from the selection, that can come from
    /// selecting a patchy area with the Fuzzy Select Tool. In technical
    /// terms this procedure floods the selection. See the Algorithms page
    /// in the developer wiki for details.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_flood")]
    pub fn flood(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_flood(image.to_glib_none().0))
        }
    }

    /// Returns a [`Selection`][crate::Selection] representing `selection_id`. This function
    /// calls [`Item::by_id()`][crate::Item::by_id()] and returns the item if it is selection
    /// or [`None`] otherwise.
    /// ## `selection_id`
    /// The selection id.
    ///
    /// # Returns
    ///
    /// a [`Selection`][crate::Selection] for
    ///  `selection_id` or [`None`] if `selection_id` does not represent
    ///  a valid selection. The object belongs to libgimp and you
    ///  must not modify or unref it.
    #[doc(alias = "gimp_selection_get_by_id")]
    #[doc(alias = "get_by_id")]
    pub fn by_id(selection_id: i32) -> Option<Selection> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gimp_selection_get_by_id(selection_id))
        }
    }

    /// Grow the image's selection
    ///
    /// This procedure grows the selection. Growing involves expanding the
    /// boundary in all directions by the specified pixel amount.
    /// ## `image`
    /// The image.
    /// ## `steps`
    /// Steps of grow (in pixels).
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_grow")]
    pub fn grow(image: &Image, steps: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_grow(image.to_glib_none().0, steps))
        }
    }

    /// Invert the selection mask.
    ///
    /// This procedure inverts the selection mask. For every pixel in the
    /// selection channel, its new value is calculated as (255 - old-value).
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_invert")]
    pub fn invert(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_invert(image.to_glib_none().0))
        }
    }

    /// Determine whether the selection is empty.
    ///
    /// This procedure returns TRUE if the selection for the specified image
    /// is empty.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// Is the selection empty?
    #[doc(alias = "gimp_selection_is_empty")]
    pub fn is_empty(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_is_empty(image.to_glib_none().0))
        }
    }

    /// Deselect the entire image.
    ///
    /// This procedure deselects the entire image. Every pixel in the
    /// selection channel is set to 0.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_none")]
    pub fn none(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_none(image.to_glib_none().0))
        }
    }

    /// Copy the selection mask to a new channel.
    ///
    /// This procedure copies the selection mask and stores the content in a
    /// new channel. The new channel is automatically inserted into the
    /// image's list of channels.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// The new channel.
    #[doc(alias = "gimp_selection_save")]
    pub fn save(image: &Image) -> Option<Channel> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gimp_selection_save(image.to_glib_none().0))
        }
    }

    /// Sharpen the selection mask.
    ///
    /// This procedure sharpens the selection mask. For every pixel in the
    /// selection channel, if the value is &gt; 127, the new pixel is
    /// assigned a value of 255. This removes any \"anti-aliasing\" that
    /// might exist in the selection mask's boundary.
    /// ## `image`
    /// The image.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_sharpen")]
    pub fn sharpen(image: &Image) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_sharpen(image.to_glib_none().0))
        }
    }

    /// Shrink the image's selection
    ///
    /// This procedure shrinks the selection. Shrinking involves trimming
    /// the existing selection boundary on all sides by the specified number
    /// of pixels.
    /// ## `image`
    /// The image.
    /// ## `steps`
    /// Steps of shrink (in pixels).
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_shrink")]
    pub fn shrink(image: &Image, steps: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_shrink(image.to_glib_none().0, steps))
        }
    }

    /// Translate the selection by the specified offsets.
    ///
    /// This procedure actually translates the selection for the specified
    /// image by the specified offsets. Regions that are translated from
    /// beyond the bounds of the image are set to empty. Valid regions of
    /// the selection which are translated beyond the bounds of the image
    /// because of this call are lost.
    /// ## `image`
    /// The image.
    /// ## `offx`
    /// x offset for translation.
    /// ## `offy`
    /// y offset for translation.
    ///
    /// # Returns
    ///
    /// TRUE on success.
    #[doc(alias = "gimp_selection_translate")]
    pub fn translate(image: &Image, offx: i32, offy: i32) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gimp_selection_translate(image.to_glib_none().0, offx, offy))
        }
    }

    /// Find the value of the selection at the specified coordinates.
    ///
    /// This procedure returns the value of the selection at the specified
    /// coordinates. If the coordinates lie out of bounds, 0 is returned.
    /// ## `image`
    /// The image.
    /// ## `x`
    /// x coordinate of value.
    /// ## `y`
    /// y coordinate of value.
    ///
    /// # Returns
    ///
    /// Value of the selection.
    #[doc(alias = "gimp_selection_value")]
    pub fn value(image: &Image, x: i32, y: i32) -> i32 {
        skip_assert_initialized!();
        unsafe {
            ffi::gimp_selection_value(image.to_glib_none().0, x, y)
        }
    }
}
