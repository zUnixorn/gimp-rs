<!-- file * -->
<!-- fn adaptive_supersample_area -->
## `x1`
left x coordinate of the area to process.
## `y1`
top y coordinate of the area to process.
## `x2`
right x coordinate of the area to process.
## `y2`
bottom y coordinate of the area to process.
## `max_depth`
maximum depth of supersampling.
## `threshold`
lower threshold of pixel difference that stops
 supersampling.
## `render_func`
function calculate the color value at
 given coordinates.
## `render_data`
user data passed to `render_func`.
## `put_pixel_func`
function to a pixels to a color at
 given coordinates.
## `put_pixel_data`
user data passed to `put_pixel_func`.
## `progress_func`
function to report progress.
## `progress_data`
user data passed to `progress_func`.

# Returns

the number of pixels processed.
<!-- fn any_to_utf8 -->
This function takes any string (UTF-8 or not) and always returns a valid
UTF-8 string.

If `str` is valid UTF-8, a copy of the string is returned.

If UTF-8 validation fails, `g_locale_to_utf8()` is tried and if it
succeeds the resulting string is returned.

Otherwise, the portion of `str` that is UTF-8, concatenated
with "(invalid UTF-8 string)" is returned. If not even the start
of `str` is valid UTF-8, only "(invalid UTF-8 string)" is returned.
## `str`
The string to be converted to UTF-8.
## `warning_format`
The message format for the warning message if conversion
 to UTF-8 fails. See the `<function>``printf()``</function>`
 documentation.

# Returns

The UTF-8 string as described above.
<!-- fn bilinear_rgb -->
## `values`
Array of pixels in RGBA double format
## `has_alpha`
Whether `values` has an alpha channel
## `retvalues`
Resulting pixel
<!-- fn buffer_get_image_type -->
Retrieves the specified buffer's image type.

This procedure retrieves the specified named buffer's image type.
## `buffer_name`
The buffer name.

# Returns

The buffer image type.
<!-- fn cairo_checkerboard_create -->
Create a repeating checkerboard pattern.
## `cr`
Cairo context
## `size`
check size
## `light`
light check color or [`None`] to use the default light gray
## `dark`
dark check color or [`None`] to use the default dark gray

# Returns

a new Cairo pattern that can be used as a source on `cr`.
<!-- fn cairo_surface_create_buffer -->
This function returns a [`gegl::Buffer`][crate::gegl::Buffer] which wraps `surface`'s pixels.
It must only be called on image surfaces, calling it on other surface
types is an error.

If `format` is set, the returned [class`Gegl`] will use it. It has to
map with `surface` Cairo format. If unset, the buffer format will be
determined from `surface`. The main difference is that automatically
determined format has sRGB space and TRC by default.
## `surface`
a Cairo surface
## `format`
a Babl format.

# Returns

a [`gegl::Buffer`][crate::gegl::Buffer]
<!-- fn cairo_surface_get_format -->
This function returns a `Babl` format that corresponds to `surface`'s
pixel format.
## `surface`
a Cairo surface

# Returns

the `Babl` format of `surface`.
<!-- fn check_size -->
Returns the size of the checkerboard to be used in previews.

This is a constant value given at plug-in configuration time.

# Returns

the check_size value
<!-- fn check_type -->
Returns the type of the checkerboard to be used in previews.

This is a constant value given at plug-in configuration time.

# Returns

the check_type value
<!-- fn checks_get_colors -->
Retrieves the colors to use when drawing a checkerboard for a certain
`GimpCheckType` and custom colors.
If `type_` is `GIMP_CHECK_TYPE_CUSTOM_CHECKS`, then `color1` and `color2`
will remain untouched, which means you must initialize them to the
values expected for custom checks.

To obtain the user-set colors in Preferences, just call:


**⚠️ The following code is in C ⚠️**

```C
GeglColor *color1 = gimp_check_custom_color1 ();
GeglColor *color2 = gimp_check_custom_color2 ();
gimp_checks_get_colors (gimp_check_type (), &color1, &color2);
```
## `type_`
the checkerboard type
## `color1`
current custom color and return location for the first color.
## `color2`
current custom color and return location for the second color.
<!-- fn clone -->
Clone from the source to the dest drawable using the current brush

This tool clones (copies) from the source drawable starting at the
specified source coordinates to the dest drawable. If the
\"clone_type\" argument is set to PATTERN-CLONE, then the current
pattern is used as the source and the \"src_drawable\" argument is
ignored. Pattern cloning assumes a tileable pattern and mods the sum
of the src coordinates and subsequent stroke offsets with the width
and height of the pattern. For image cloning, if the sum of the src
coordinates and subsequent stroke offsets exceeds the extents of the
src drawable, then no paint is transferred. The clone tool is
capable of transforming between any image types including
RGB-&gt;Indexed--although converting from any type to indexed is
significantly slower.
## `drawable`
The affected drawable.
## `src_drawable`
The source drawable.
## `clone_type`
The type of clone.
## `src_x`
The x coordinate in the source image.
## `src_y`
The y coordinate in the source image.
## `strokes`
Array of stroke coordinates: { s1.x, s1.y, s2.x, s2.y, ..., sn.x, sn.y }.

# Returns

TRUE on success.
<!-- fn color_is_out_of_gamut -->
Determine whether `color` is out of its `space` gamut.
A small error of margin is accepted, so that for instance a component
at -0.0000001 is not making the whole color to be considered as
out-of-gamut while it may just be computation imprecision.
## `color`
a [class`Gegl`]
## `space`
a color space to convert `color` to.

# Returns

whether the color is out of `space` gamut.
<!-- fn context_get_distance_metric -->
Get the distance metric used in some computations.

Returns the distance metric in the current context. See
`gimp_context_set_distance_metric()` to know more about its usage.

# Returns

The distance metric.
<!-- fn context_get_gradient_blend_color_space -->
Get the gradient blend color space.

Get the gradient blend color space for paint tools and the gradient
tool.

# Returns

Color blend space.
<!-- fn context_get_gradient_repeat_mode -->
Get the gradient repeat mode.

Get the gradient repeat mode for paint tools and the gradient tool.

# Returns

Repeat mode.
<!-- fn context_get_ink_blob_type -->
Get ink blob type.

Get the ink blob type for ink tool.

# Returns

Ink blob type.
<!-- fn context_get_interpolation -->
Get the interpolation type.

Returns the interpolation setting. The return value is an integer
which corresponds to the values listed in the argument description.
If the interpolation has not been set explicitly by
`gimp_context_set_interpolation()`, the default interpolation set in
gimprc will be used.

# Returns

The interpolation type.
<!-- fn context_get_line_cap_style -->
Get the line cap style setting.

Returns the line cap style setting.

# Returns

The line cap style setting.
<!-- fn context_get_line_join_style -->
Get the line join style setting.

Returns the line join style setting.

# Returns

The line join style setting.
<!-- fn context_get_sample_criterion -->
Get the sample criterion setting.

Returns the sample criterion setting.

# Returns

The sample criterion setting.
<!-- fn context_get_stroke_method -->
Get the currently active stroke method.

Returns the currently active stroke method.

# Returns

The active stroke method.
<!-- fn context_get_transform_direction -->
Get the transform direction.

Returns the transform direction. The return value is an integer
which corresponds to the values listed in the argument description.

# Returns

The transform direction.
<!-- fn context_get_transform_resize -->
Get the transform resize type.

Returns the transform resize setting. The return value is an integer
which corresponds to the values listed in the argument description.

# Returns

The transform resize type.
<!-- fn context_set_distance_metric -->
Set the distance metric used in some computations.

Modifies the distance metric used in some computations, such as
`gimp_drawable_edit_gradient_fill()`. In particular, it does not
change the metric used in generic distance computation on canvas, as
in the Measure tool.

This setting affects the following procedures:
`gimp_drawable_edit_gradient_fill()`.
## `metric`
The distance metric.

# Returns

TRUE on success.
<!-- fn context_set_gradient_blend_color_space -->
Set the gradient blend color space.

Set the gradient blend color space for paint tools and the gradient
tool.
## `blend_color_space`
Blend color space.

# Returns

TRUE on success.
<!-- fn context_set_gradient_repeat_mode -->
Set the gradient repeat mode.

Set the gradient repeat mode for paint tools and the gradient tool.
## `repeat_mode`
Repeat mode.

# Returns

TRUE on success.
<!-- fn context_set_ink_blob_type -->
Set ink blob type.

Set the ink blob type for ink tool.
## `type_`
Ink blob type.

# Returns

TRUE on success.
<!-- fn context_set_interpolation -->
Set the interpolation type.

Modifies the interpolation setting.

This setting affects affects the following procedures:
[`ItemExt::transform_flip()`][crate::prelude::ItemExt::transform_flip()], [`ItemExt::transform_perspective()`][crate::prelude::ItemExt::transform_perspective()],
[`ItemExt::transform_rotate()`][crate::prelude::ItemExt::transform_rotate()], [`ItemExt::transform_scale()`][crate::prelude::ItemExt::transform_scale()],
[`ItemExt::transform_shear()`][crate::prelude::ItemExt::transform_shear()], [`ItemExt::transform_2d()`][crate::prelude::ItemExt::transform_2d()],
[`ItemExt::transform_matrix()`][crate::prelude::ItemExt::transform_matrix()], [`Image::scale()`][crate::Image::scale()],
[`LayerExt::scale()`][crate::prelude::LayerExt::scale()].
## `interpolation`
The interpolation type.

# Returns

TRUE on success.
<!-- fn context_set_line_cap_style -->
Set the line cap style setting.

Modifies the line cap style setting for stroking lines.

This setting affects the following procedures:
[`DrawableExt::edit_stroke_selection()`][crate::prelude::DrawableExt::edit_stroke_selection()],
[`DrawableExt::edit_stroke_item()`][crate::prelude::DrawableExt::edit_stroke_item()].
## `cap_style`
The line cap style setting.

# Returns

TRUE on success.
<!-- fn context_set_line_join_style -->
Set the line join style setting.

Modifies the line join style setting for stroking lines.
This setting affects the following procedures:
[`DrawableExt::edit_stroke_selection()`][crate::prelude::DrawableExt::edit_stroke_selection()],
[`DrawableExt::edit_stroke_item()`][crate::prelude::DrawableExt::edit_stroke_item()].
## `join_style`
The line join style setting.

# Returns

TRUE on success.
<!-- fn context_set_sample_criterion -->
Set the sample criterion setting.

Modifies the sample criterion setting. If an operation depends on
the colors of the pixels present in a drawable, like when doing a
seed fill, this setting controls how color similarity is determined.
SELECT_CRITERION_COMPOSITE is the default value.

This setting affects the following procedures:
[`Image::select_color()`][crate::Image::select_color()], [`Image::select_contiguous_color()`][crate::Image::select_contiguous_color()],
`gimp_drawable_edit_bucket_fill()`.
## `sample_criterion`
The sample criterion setting.

# Returns

TRUE on success.
<!-- fn context_set_stroke_method -->
Set the active stroke method.

Sets the active stroke method. The method will be used in all
subsequent stroke operations.
## `stroke_method`
The new stroke method.

# Returns

TRUE on success.
<!-- fn context_set_transform_direction -->
Set the transform direction.

Modifies the transform direction setting.

This setting affects affects the following procedures:
[`ItemExt::transform_flip()`][crate::prelude::ItemExt::transform_flip()], [`ItemExt::transform_perspective()`][crate::prelude::ItemExt::transform_perspective()],
[`ItemExt::transform_rotate()`][crate::prelude::ItemExt::transform_rotate()], [`ItemExt::transform_scale()`][crate::prelude::ItemExt::transform_scale()],
[`ItemExt::transform_shear()`][crate::prelude::ItemExt::transform_shear()], [`ItemExt::transform_2d()`][crate::prelude::ItemExt::transform_2d()],
[`ItemExt::transform_matrix()`][crate::prelude::ItemExt::transform_matrix()].
## `transform_direction`
The transform direction.

# Returns

TRUE on success.
<!-- fn context_set_transform_resize -->
Set the transform resize type.

Modifies the transform resize setting. When transforming pixels, if
the result of a transform operation has a different size than the
original area, this setting determines how the resulting area is
sized.

This setting affects affects the following procedures:
[`ItemExt::transform_flip()`][crate::prelude::ItemExt::transform_flip()], [`ItemExt::transform_flip_simple()`][crate::prelude::ItemExt::transform_flip_simple()],
[`ItemExt::transform_perspective()`][crate::prelude::ItemExt::transform_perspective()], [`ItemExt::transform_rotate()`][crate::prelude::ItemExt::transform_rotate()],
[`ItemExt::transform_rotate_simple()`][crate::prelude::ItemExt::transform_rotate_simple()], [`ItemExt::transform_scale()`][crate::prelude::ItemExt::transform_scale()],
[`ItemExt::transform_shear()`][crate::prelude::ItemExt::transform_shear()], [`ItemExt::transform_2d()`][crate::prelude::ItemExt::transform_2d()],
[`ItemExt::transform_matrix()`][crate::prelude::ItemExt::transform_matrix()].
## `transform_resize`
The transform resize type.

# Returns

TRUE on success.
<!-- fn convolve -->
Convolve (Blur, Sharpen) using the current brush.

This tool convolves the specified drawable with either a sharpening
or blurring kernel. The pressure parameter controls the magnitude of
the operation. Like the paintbrush, this tool linearly interpolates
between the specified stroke coordinates.
## `drawable`
The affected drawable.
## `pressure`
The pressure.
## `convolve_type`
Convolve type.
## `strokes`
Array of stroke coordinates: { s1.x, s1.y, s2.x, s2.y, ..., sn.x, sn.y }.

# Returns

TRUE on success.
<!-- fn core_object_array_get_length -->
## `array`
a [`None`]-terminated array of objects.

# Returns

the number of [class`GObject`] in `array`.
<!-- fn cpu_accel_get_support -->
Query for CPU acceleration support.

# Returns

`GimpCpuAccelFlags` as supported by the CPU.
<!-- fn data_directory_file -->
Returns a [`gio::File`][crate::gio::File] in the data directory, or the data directory
itself if `first_element` is [`None`].

See also: [`data_directory()`][crate::data_directory()].
## `first_element`
the first element of a path to a file in the
 data directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- fn default_display -->
Returns the default display ID. This corresponds to the display the
running procedure's menu entry was invoked from.

This is a constant value given at plug-in configuration time.

# Returns

the default display ID
 The object belongs to libgimp and you should not free it.
<!-- fn directory_file -->
Returns a [`gio::File`][crate::gio::File] in the user's GIMP directory, or the GIMP
directory itself if `first_element` is [`None`].

See also: [`directory()`][crate::directory()].
## `first_element`
the first element of a path to a file in the
 user's GIMP directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- fn dodgeburn -->
Dodgeburn image with varying exposure.

Dodgeburn. More details here later.
## `drawable`
The affected drawable.
## `exposure`
The exposure of the strokes.
## `dodgeburn_type`
The type either dodge or burn.
## `dodgeburn_mode`
The mode.
## `strokes`
Array of stroke coordinates: { s1.x, s1.y, s2.x, s2.y, ..., sn.x, sn.y }.

# Returns

TRUE on success.
<!-- fn enum_get_desc -->
Retrieves `GimpEnumDesc` associated with the given value, or [`None`].
## `enum_class`
a `GEnumClass`
## `value`
a value from `enum_class`

# Returns

the value's `GimpEnumDesc`.
<!-- fn enum_get_value_descriptions -->
Retrieves the array of human readable and translatable descriptions
and help texts for enum values.
## `enum_type`
a `GType`

# Returns

a [`None`] terminated constant array of `GimpEnumDesc`
<!-- fn enum_set_value_descriptions -->
Sets the array of human readable and translatable descriptions
and help texts for enum values.
## `enum_type`
a `GType`
## `descriptions`
a [`None`] terminated constant static array of `GimpEnumDesc`
<!-- fn enum_value_get_abbrev -->
Retrieves the translated abbreviation for a given `enum_value`.
## `enum_class`
a `GEnumClass`
## `enum_value`
a `GEnumValue` from `enum_class`

# Returns

the translated abbreviation of the enum value
<!-- fn enum_value_get_desc -->
Retrieves the translated description for a given `enum_value`.
## `enum_class`
a `GEnumClass`
## `enum_value`
a `GEnumValue` from `enum_class`

# Returns

the translated description of the enum value
<!-- fn enum_value_get_help -->
Retrieves the translated help for a given `enum_value`.
## `enum_class`
a `GEnumClass`
## `enum_value`
a `GEnumValue` from `enum_class`

# Returns

the translated help of the enum value
<!-- fn eraser -->
Erase using the current brush.

This tool erases using the current brush mask. If the specified
drawable contains an alpha channel, then the erased pixels will
become transparent. Otherwise, the eraser tool replaces the contents
of the drawable with the background color. Like paintbrush, this
tool linearly interpolates between the specified stroke coordinates.
## `drawable`
The affected drawable.
## `strokes`
Array of stroke coordinates: { s1.x, s1.y, s2.x, s2.y, ..., sn.x, sn.y }.
## `hardness`
How to apply the brush.
## `method`
The paint method to use.

# Returns

TRUE on success.
<!-- fn file_save -->
Saves a file by extension.

This procedure invokes the correct file save handler according to
the file's extension and/or prefix.
The `options` argument is currently unused and should be set to [`None`]
right now.
## `run_mode`
The run mode.
## `image`
Input image.
## `file`
The file to save the image in.
## `options`
Export option settings.

# Returns

TRUE on success.
<!-- fn flags_get_first_desc -->
Retrieves the first `GimpFlagsDesc` that matches the given value, or [`None`].
## `flags_class`
a `GFlagsClass`
## `value`
a value from `flags_class`

# Returns

the value's `GimpFlagsDesc`.
<!-- fn flags_get_value_descriptions -->
Retrieves the array of human readable and translatable descriptions
and help texts for flags values.
## `flags_type`
a `GType`

# Returns

a [`None`] terminated constant array of `GimpFlagsDesc`
<!-- fn flags_set_value_descriptions -->
Sets the array of human readable and translatable descriptions
and help texts for flags values.
## `flags_type`
a `GType`
## `descriptions`
a [`None`] terminated constant static array of `GimpFlagsDesc`
<!-- fn flags_value_get_abbrev -->
Retrieves the translated abbreviation for a given `flags_value`.
## `flags_class`
a `GFlagsClass`
## `flags_value`
a `GFlagsValue` from `flags_class`

# Returns

the translated abbreviation of the flags value
<!-- fn flags_value_get_desc -->
Retrieves the translated description for a given `flags_value`.
## `flags_class`
a `GFlagsClass`
## `flags_value`
a `GFlagsValue` from `flags_class`

# Returns

the translated description of the flags value
<!-- fn flags_value_get_help -->
Retrieves the translated help for a given `flags_value`.
## `flags_class`
a `GFlagsClass`
## `flags_value`
a `GFlagsValue` from `flags_class`

# Returns

the translated help of the flags value
<!-- fn color_configuration -->
Retrieve a copy of the current color management configuration.

# Returns

A copy of the core's `GimpColorConfig`. You
 should unref this copy if you don't need it any longer.
<!-- fn pdb -->
This function returns the plug-in's `GimpPDB` instance, which is a
singleton that can exist exactly once per running plug-in.

# Returns

The plug-in's `GimpPDB` singleton.
<!-- fn installation_directory_file -->
Returns a [`gio::File`][crate::gio::File] in the installation directory, or the installation
directory itself if `first_element` is [`None`].

See also: [`installation_directory()`][crate::installation_directory()].
## `first_element`
the first element of a path to a file in the
 top installation directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- fn locale_directory_file -->
Returns a [`gio::File`][crate::gio::File] in the locale directory, or the locale directory
itself if `first_element` is [`None`].

See also: [`locale_directory()`][crate::locale_directory()].
## `first_element`
the first element of a path to a file in the
 locale directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- fn message_get_handler -->
Returns the current state of where warning messages are displayed.

This procedure returns the way g_message warnings are displayed.
They can be shown in a dialog box or printed on the console where
gimp was started.

# Returns

The current handler type.
<!-- fn message_set_handler -->
Controls where warning messages are displayed.

This procedure controls how g_message warnings are displayed. They
can be shown in a dialog box or printed on the console where gimp
was started.
## `handler`
The new handler type.

# Returns

TRUE on success.
<!-- fn paintbrush -->
Paint in the current brush with optional fade out parameter and pull
colors from a gradient.

This tool is the standard paintbrush. It draws linearly interpolated
lines through the specified stroke coordinates. It operates on the
specified drawable in the foreground color with the active brush.
The 'fade-out' parameter is measured in pixels and allows the brush
stroke to linearly fall off. The pressure is set to the maximum at
the beginning of the stroke. As the distance of the stroke nears the
fade-out value, the pressure will approach zero. The gradient-length
is the distance to spread the gradient over. It is measured in
pixels. If the gradient-length is 0, no gradient is used.
## `drawable`
The affected drawable.
## `fade_out`
Fade out parameter.
## `strokes`
Array of stroke coordinates: { s1.x, s1.y, s2.x, s2.y, ..., sn.x, sn.y }.
## `method`
The paint method to use.
## `gradient_length`
Length of gradient to draw.

# Returns

TRUE on success.
<!-- fn param_spec_config_path -->
Creates a param spec to hold a filename, dir name,
or list of file or dir names.
See [`glib::ParamSpec::internal()`][crate::glib::ParamSpec::internal()] for more information.
## `name`
Canonical name of the param
## `nick`
Nickname of the param
## `blurb`
Brief description of param.
## `type_`
a `GimpConfigPathType` value.
## `default_value`
Value to use if none is assigned.
## `flags`
a combination of [`glib::ParamFlags`][crate::glib::ParamFlags]

# Returns

a newly allocated [`glib::ParamSpec`][crate::glib::ParamSpec] instance
<!-- fn param_spec_config_path_type -->
Tells whether the path param encodes a filename,
dir name, or list of file or dir names.
## `pspec`
A [`glib::ParamSpec`][crate::glib::ParamSpec] for a path param

# Returns

a `GimpConfigPathType` value
<!-- fn param_spec_matrix2 -->
Creates a param spec to hold a `GimpMatrix2` value.
See [`glib::ParamSpec::internal()`][crate::glib::ParamSpec::internal()] for more information.
## `name`
Canonical name of the param
## `nick`
Nickname of the param
## `blurb`
Brief description of param.
## `default_value`
Value to use if none is assigned.
## `flags`
a combination of [`glib::ParamFlags`][crate::glib::ParamFlags]

# Returns

a newly allocated [`glib::ParamSpec`][crate::glib::ParamSpec] instance
<!-- fn param_spec_matrix3 -->
Creates a param spec to hold a `GimpMatrix3` value.
See [`glib::ParamSpec::internal()`][crate::glib::ParamSpec::internal()] for more information.
## `name`
Canonical name of the param
## `nick`
Nickname of the param
## `blurb`
Brief description of param.
## `default_value`
Value to use if none is assigned.
## `flags`
a combination of [`glib::ParamFlags`][crate::glib::ParamFlags]

# Returns

a newly allocated [`glib::ParamSpec`][crate::glib::ParamSpec] instance
<!-- fn pixbuf_get_format -->
Returns the Babl format that corresponds to the `pixbuf`'s pixel format.
## `pixbuf`
a [`gdk_pixbuf::Pixbuf`][crate::gdk_pixbuf::Pixbuf]

# Returns

the `pixbuf`'s pixel format
<!-- fn progress_init_printf -->
Initializes the progress bar for the current plug-in.

Initializes the progress bar for the current plug-in. It is only
valid to call this procedure from a plug-in.
## `format`
a standard `printf()` format string

# Returns

[`true`] on success.
<!-- fn progress_install_vtable -->
## `vtable`
a pointer to a `GimpProgressVtable`.
## `user_data_destroy`
destroy function for `user_data`, or [`None`].

# Returns

the name of the temporary procedure that's been installed
<!-- fn progress_set_text_printf -->
Changes the text in the progress bar for the current plug-in.

This function changes the text in the progress bar for the current
plug-in. Unlike [`progress_init()`][crate::progress_init()] it does not change the
displayed value.
## `format`
a standard `printf()` format string

# Returns

[`true`] on success.
<!-- fn stack_trace_print -->
Attempts to generate a stack trace at current code position in
`prog_name`. `prog_name` is mostly a helper and can be set to NULL.
Nevertheless if set, it has to be the current program name (argv[0]).
This function is not meant to generate stack trace for third-party
programs, and will attach the current process id only.
Internally, this function uses `gdb` or `lldb` if they are available,
or the `stacktrace()` API on platforms where it is available. It always
fails on Win32.

The stack trace, once generated, will either be printed to `stream` or
returned as a newly allocated string in `trace`, if not [`None`].

In some error cases (e.g. segmentation fault), trying to allocate
more memory will trigger more segmentation faults and therefore loop
our error handling (which is just wrong). Therefore printing to a
file description is an implementation without any memory allocation.
## `prog_name`
the program to attach to.
## `stream`
a FILE* stream.

# Returns

[`true`] if a stack trace could be generated, [`false`]
otherwise.

## `trace`
location to store a newly allocated string of
 the trace.
<!-- fn sysconf_directory_file -->
Returns a [`gio::File`][crate::gio::File] in the sysconf directory, or the sysconf directory
itself if `first_element` is [`None`].

See also: [`sysconf_directory()`][crate::sysconf_directory()].
## `first_element`
the first element of a path to a file in the
 sysconf directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- fn value_dup_double_array -->
Gets the contents of a `GIMP_TYPE_DOUBLE_ARRAY` `GValue`
## `value`
A valid value of type `GIMP_TYPE_DOUBLE_ARRAY`

# Returns

The contents of `value`
<!-- fn value_dup_int32_array -->
Gets the contents of a `GIMP_TYPE_INT32_ARRAY` `GValue`
## `value`
A valid value of type `GIMP_TYPE_INT32_ARRAY`

# Returns

The contents of `value`
<!-- fn value_get_double_array -->
Gets the contents of a `GIMP_TYPE_DOUBLE_ARRAY` `GValue`
## `value`
A valid value of type `GIMP_TYPE_DOUBLE_ARRAY`

# Returns

The contents of `value`
<!-- fn value_get_int32_array -->
Gets the contents of a `GIMP_TYPE_INT32_ARRAY` `GValue`
## `value`
A valid value of type `GIMP_TYPE_INT32_ARRAY`

# Returns

The contents of `value`
<!-- fn value_set_double_array -->
Sets the contents of `value` to `data`.
## `value`
A valid value of type `GIMP_TYPE_DOUBLE_ARRAY`
## `data`
A `gdouble` array
<!-- fn value_set_int32_array -->
Sets the contents of `value` to `data`.
## `value`
A valid value of type `GIMP_TYPE_INT32_ARRAY`
## `data`
A `gint32` array
<!-- fn value_set_static_double_array -->
Sets the contents of `value` to `data`, without copying the data.
## `value`
A valid value of type `GIMP_TYPE_DOUBLE_ARRAY`
## `data`
A `gdouble` array
<!-- fn value_set_static_int32_array -->
Sets the contents of `value` to `data`, without copying the data.
## `value`
A valid value of type `GIMP_TYPE_INT32_ARRAY`
## `data`
A `gint32` array
<!-- fn value_take_double_array -->
Sets the contents of `value` to `data`, and takes ownership of `data`.
## `value`
A valid value of type `GIMP_TYPE_DOUBLE_ARRAY`
## `data`
A `gdouble` array
<!-- fn value_take_int32_array -->
Sets the contents of `value` to `data`, and takes ownership of `data`.
## `value`
A valid value of type `GIMP_TYPE_int32_ARRAY`
## `data`
A `gint32` array
<!-- fn vector_2d_to_3d -->
\"Compute screen (sx, sy) - (sx + w, sy + h) to 3D unit square
mapping. The plane to map to is given in the z field of p. The
observer is located at position vp (vp->z != 0.0).\"

In other words, this computes the projection of the point (`x`, `y`)
to the plane z = `p`->z (parallel to XY), from the `vp` point of view
through the screen (`sx`, `sy`)->(`sx` + `w`, `sy` + `h`)
## `sx`
the abscissa of the upper-left screen rectangle.
## `sy`
the ordinate of the upper-left screen rectangle.
## `w`
the width of the screen rectangle.
## `h`
the height of the screen rectangle.
## `x`
the abscissa of the point in the screen rectangle to map.
## `y`
the ordinate of the point in the screen rectangle to map.
## `vp`
the position of the observer.
## `p`
the resulting point.
<!-- fn vector_2d_to_3d_val -->
This function is identical to `gimp_vector_2d_to_3d()` but the
position of the `observer` and the resulting point `p` are passed by
value rather than by reference.
## `sx`
the abscissa of the upper-left screen rectangle.
## `sy`
the ordinate of the upper-left screen rectangle.
## `w`
the width of the screen rectangle.
## `h`
the height of the screen rectangle.
## `x`
the abscissa of the point in the screen rectangle to map.
## `y`
the ordinate of the point in the screen rectangle to map.
## `vp`
position of the observer.
## `p`
the resulting point.

# Returns

the computed `GimpVector3` point.
<!-- fn vector_3d_to_2d -->
Convert the given 3D point to 2D (project it onto the viewing
plane, (sx, sy, 0) - (sx + w, sy + h, 0). The input is assumed to
be in the unit square (0, 0, z) - (1, 1, z). The viewpoint of the
observer is passed in vp.

This is basically the opposite of `gimp_vector_2d_to_3d()`.
## `sx`
the abscissa of the upper-left screen rectangle.
## `sy`
the ordinate of the upper-left screen rectangle.
## `w`
the width of the screen rectangle.
## `h`
the height of the screen rectangle.
## `vp`
position of the observer.
## `p`
the 3D point to project to the plane.

# Returns


## `x`
the abscissa of the point in the screen rectangle to map.

## `y`
the ordinate of the point in the screen rectangle to map.
<!-- const CHECK_DARK -->
The dark gray value for the default checkerboard pattern.
<!-- const CHECK_LIGHT -->
The dark light value for the default checkerboard pattern.
<!-- const CHECK_SIZE -->
The default checkerboard size in pixels. This is configurable in
the core but GIMP plug-ins can't access the user preference and
should use this constant instead.
<!-- const CHECK_SIZE_SM -->
The default small checkerboard size in pixels.
<!-- const CONFIG_PARAM_AGGREGATE -->
The object property is to be treated as part of the parent object.
<!-- const CONFIG_PARAM_CONFIRM -->
Changes to this property should be confirmed by the user before
being applied.
<!-- const CONFIG_PARAM_DEFAULTS -->
Don't serialize this property if it has the default value.
<!-- const CONFIG_PARAM_DONT_COMPARE -->
Ignore this property when comparing objects.
<!-- const CONFIG_PARAM_FLAGS -->
The default flags that should be used for serializable `GimpConfig`
properties.
<!-- const CONFIG_PARAM_FLAG_SHIFT -->
Minimum shift count to be used for core application defined
[flags`GObject`].
<!-- const CONFIG_PARAM_IGNORE -->
This property exists for obscure reasons or is needed for backward
compatibility. Ignore the value read and don't serialize it.
<!-- const CONFIG_PARAM_RESTART -->
Changes to this property take effect only after a restart.
<!-- const CONFIG_PARAM_SERIALIZE -->
A property that can and should be serialized and deserialized.
<!-- const MAJOR_VERSION -->
The major GIMP version number.
<!-- const MAX_IMAGE_SIZE -->
The maximum width and height of a GIMP image in pixels. This is a
somewhat arbitrary value that can be used when an upper value for
pixel sizes is needed; for example to give a spin button an upper
limit.
<!-- const MAX_MEMSIZE -->
A large but arbitrary value that can be used when an upper limit
for a memory size (in bytes) is needed. It is smaller than
`G_MAXDOUBLE` since the `GimpMemsizeEntry` doesn't handle larger
values.
<!-- const MAX_RESOLUTION -->
The maximum resolution of a GIMP image in pixels per inch. This is
a somewhat arbitrary value that can be used to when an upper value
for a resolution is needed. GIMP will not accept resolutions larger
than this value.
<!-- const MICRO_VERSION -->
The micro GIMP version number.
<!-- const MINOR_VERSION -->
The minor GIMP version number.
<!-- const MIN_IMAGE_SIZE -->
The minimum width and height of a GIMP image in pixels.
<!-- const MIN_RESOLUTION -->
The minimum resolution of a GIMP image in pixels per inch. This is
a somewhat arbitrary value that can be used when a lower value for a
resolution is needed. GIMP will not accept resolutions smaller than
this value.
<!-- const MODULE_ABI_VERSION -->
The version of the module system's ABI. Modules put this value into
`GimpModuleInfo`'s `abi_version` field so the code loading the modules
can check if it was compiled against the same module ABI the modules
are compiled against.

 GIMP_MODULE_ABI_VERSION is incremented each time one of the
 following changes:

 - the libgimpmodule implementation (if the change affects modules).

 - one of the classes implemented by modules (currently `GimpColorDisplay`,
 `GimpColorSelector` and `GimpController`).
<!-- const PARAM_DONT_SERIALIZE -->
This property will be ignored when serializing and deserializing.
This is useful for GimpProcedure arguments for which you never want
the last run values to be restored.

Since 3.0
<!-- const PARAM_FLAG_SHIFT -->
Minimum shift count to be used for libgimpconfig defined
[flags`GObject`] (see libgimpconfig/gimpconfig-params.h).
<!-- const PARAM_NO_VALIDATE -->
Since 3.0
<!-- impl Brush::fn buffer -->
Gets pixel data of the brush within the bounding box specified by `max_width`
and `max_height`. The data will be scaled down so that it fits within this
size without changing its ratio. If the brush is smaller than this size to
begin with, it will not be scaled up.

If `max_width` or `max_height` are [`None`], the buffer is returned in the brush's
native size.

When the brush is parametric or a raster mask, only the mask (as returned by
[method`Gimp`.get_mask]) will be set. The returned buffer will be NULL.

Make sure you called [func`Gegl`] before calling any function using
`GEGL`.
## `max_width`
a maximum width for the returned buffer.
## `max_height`
a maximum height for the returned buffer.
## `format`
an optional Babl format.

# Returns

a [class`Gegl`] of [`None`] if the brush is parametric
 or mask only.
<!-- impl Brush::fn mask -->
Gets mask data of the brush within the bounding box specified by `max_width`
and `max_height`. The data will be scaled down so that it fits within this
size without changing its ratio. If the brush is smaller than this size to
begin with, it will not be scaled up.

If `max_width` or `max_height` are [`None`], the buffer is returned in the brush's
native size.

Make sure you called [func`Gegl`] before calling any function using
`GEGL`.
## `max_width`
a maximum width for the returned buffer.
## `max_height`
a maximum height for the returned buffer.
## `format`
an optional Babl format.

# Returns

a [class`Gegl`] representing the `self` mask.
<!-- impl Brush::fn shape -->
Gets the shape of a generated brush.

Gets the shape of a generated brush. Returns an error when called
for a non-parametric brush. The choices for shape are Circle
(GIMP_BRUSH_GENERATED_CIRCLE), Square (GIMP_BRUSH_GENERATED_SQUARE),
and Diamond (GIMP_BRUSH_GENERATED_DIAMOND). Other shapes might be
added in the future.

# Returns

TRUE on success.

## `shape`
The brush shape.
<!-- impl Brush::fn set_shape -->
Sets the shape of a generated brush.

Sets the shape of a generated brush. Returns an error when brush is
non-parametric or not editable. The choices for shape are Circle
(GIMP_BRUSH_GENERATED_CIRCLE), Square (GIMP_BRUSH_GENERATED_SQUARE),
and Diamond (GIMP_BRUSH_GENERATED_DIAMOND).
## `shape_in`
The brush shape.

# Returns

TRUE on success.

## `shape_out`
The brush shape actually assigned.
<!-- impl Choice::fn with_values -->
## `nick`
the first value.
## `id`
integer ID for `nick`.
## `label`
the label of `nick`.
## `help`
longer help text for `nick`.
...: more triplets of string to pre-fill the created `GimpChoice`.

# Returns

a [`Choice`][crate::Choice].
<!-- impl ColorProfile::fn from_lcms_profile -->
This function creates a GimpColorProfile from a cmsHPROFILE. On
error, [`None`] is returned and `error` is set. The passed
`lcms_profile` pointer is not retained by the created
[`ColorProfile`][crate::ColorProfile].
## `lcms_profile`
an LCMS cmsHPROFILE pointer

# Returns

the [`ColorProfile`][crate::ColorProfile], or [`None`].
<!-- impl ColorProfile::fn lcms_format -->
This function takes a `Babl` format and returns the lcms format to
be used with that `format`. It also returns a `Babl` format to be
used instead of the passed `format`, which usually is the same as
`format`, unless lcms doesn't support `format`.

Note that this function currently only supports RGB, RGBA, R'G'B',
R'G'B'A, Y, YA, Y', Y'A and the cairo-RGB24 and cairo-ARGB32 formats.
## `format`
a `Babl` format

# Returns

the `Babl` format to be used instead of `format`, or [`None`]
 if the passed `format` is not supported at all.

## `lcms_format`
return location for an lcms format
<!-- impl ColorProfile::fn format -->
This function takes a [`ColorProfile`][crate::ColorProfile] and a `Babl` format and
returns a new `Babl` format with `self`'s RGB primaries and TRC,
and `format`'s pixel layout.
## `format`
a `Babl` format
## `intent`
a [`ColorRenderingIntent`][crate::ColorRenderingIntent]

# Returns

the new `Babl` format.
<!-- impl ColorProfile::fn lcms_profile -->
This function returns `self`'s cmsHPROFILE. The returned
value belongs to `self` and must not be modified or freed.

# Returns

a pointer to the cmsHPROFILE.
<!-- impl ColorProfile::fn space -->
This function returns the `Babl` space of `self`, for the
specified `intent`.
## `intent`
a [`ColorRenderingIntent`][crate::ColorRenderingIntent]

# Returns

the new `Babl` space.
<!-- trait DrawableExt::fn append_filter -->
This procedure appends the specified drawable effect at the top of the
effect list of `self`.

The `self` argument must be the same as the one used when you
created the effect with [ctor`Gimp`.new].
Some effects may be slower than others to render. In order to
minimize processing time, it is preferred to customize the
operation's arguments as received with
[method`Gimp`.get_config] before adding the effect.
## `filter`
The drawable filter to append.
<!-- trait DrawableExt::fn append_new_filter -->
Utility function which combines [ctor`Gimp`.new]
followed by setting arguments for the
[class`Gimp`] returned by
[method`Gimp`.get_config], and finally appending with
[method`Gimp`.append_filter]

The variable arguments are couples of an argument name followed by a
value, NULL-terminated, such as:

**⚠️ The following code is in C ⚠️**

```C
filter = gimp_drawable_append_new_filter (drawable,
                                          GIMP_LAYER_MODE_REPLACE, 1.0,
                                          "gegl:gaussian-blur", "My Gaussian Blur",
                                          "std-dev-x", 2.5,
                                          "std-dev-y", 2.5,
                                          "abyss-policy", "clamp",
                                          NULL);
```
## `operation_name`
The GEGL operation's name.
## `name`
The effect name.
## `mode`
The blend mode.
## `opacity`
The opacity from 0.0 (transparent) to 1.0 (opaque).

# Returns

The newly created filter.
<!-- trait DrawableExt::fn color_balance -->
Modify the color balance of the specified drawable.

Modify the color balance of the specified drawable. There are three
axis which can be modified: cyan-red, magenta-green, and
yellow-blue. Negative values increase the amount of the former,
positive values increase the amount of the latter. Color balance can
be controlled with the 'transfer_mode' setting, which allows
shadows, mid-tones, and highlights in an image to be affected
differently. The 'preserve-lum' parameter, if TRUE, ensures that the
luminosity of each pixel remains fixed.
## `transfer_mode`
Transfer mode.
## `preserve_lum`
Preserve luminosity values at each pixel.
## `cyan_red`
Cyan-Red color balance.
## `magenta_green`
Magenta-Green color balance.
## `yellow_blue`
Yellow-Blue color balance.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn curves_explicit -->
Modifies the intensity curve(s) for specified drawable.

Modifies the intensity mapping for one channel in the specified
drawable. The channel can be either an intensity component, or the
value. The 'values' parameter is an array of doubles which
explicitly defines how each pixel value in the drawable will be
modified. Use the `gimp_drawable_curves_spline()` function to modify
intensity levels with Catmull Rom splines.
## `channel`
The channel to modify.
## `values`
The explicit curve.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn curves_spline -->
Modifies the intensity curve(s) for specified drawable.

Modifies the intensity mapping for one channel in the specified
drawable. The channel can be either an intensity component, or the
value. The 'points' parameter is an array of doubles which define a
set of control points which describe a Catmull Rom spline which
yields the final intensity curve. Use the
`gimp_drawable_curves_explicit()` function to explicitly modify
intensity levels.
## `channel`
The channel to modify.
## `points`
The spline control points: { cp1.x, cp1.y, cp2.x, cp2.y, ... }.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn desaturate -->
Desaturate the contents of the specified drawable, with the
specified formula.

This procedure desaturates the contents of the specified drawable,
with the specified formula. This procedure only works on drawables
of type RGB color.
## `desaturate_mode`
The formula to use to desaturate.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn edit_bucket_fill -->
Fill the area by a seed fill starting at the specified coordinates.

This procedure does a seed fill at the specified coordinates, using
various parameters from the current context.
In the case of merged sampling, the x and y coordinates are relative
to the image's origin; otherwise, they are relative to the
drawable's origin.

This procedure is affected by the following context setters:
[`context_set_opacity()`][crate::context_set_opacity()], [`context_set_paint_mode()`][crate::context_set_paint_mode()],
[`context_set_foreground()`][crate::context_set_foreground()], [`context_set_background()`][crate::context_set_background()],
[`context_set_pattern()`][crate::context_set_pattern()], [`context_set_sample_threshold()`][crate::context_set_sample_threshold()],
[`context_set_sample_merged()`][crate::context_set_sample_merged()],
`gimp_context_set_sample_criterion()`,
[`context_set_diagonal_neighbors()`][crate::context_set_diagonal_neighbors()], [`context_set_antialias()`][crate::context_set_antialias()].
## `fill_type`
The type of fill.
## `x`
The x coordinate of this bucket fill's application.
## `y`
The y coordinate of this bucket fill's application.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn edit_fill -->
Fill selected area of drawable.

This procedure fills the specified drawable according to fill mode.
This procedure only affects regions within a selection if there is a
selection active. If you want to fill the whole drawable, regardless
of the selection, use `gimp_drawable_fill()`.

This procedure is affected by the following context setters:
[`context_set_opacity()`][crate::context_set_opacity()], [`context_set_paint_mode()`][crate::context_set_paint_mode()],
[`context_set_foreground()`][crate::context_set_foreground()], [`context_set_background()`][crate::context_set_background()],
[`context_set_pattern()`][crate::context_set_pattern()].
## `fill_type`
The type of fill.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn edit_gradient_fill -->
Draw a gradient between the starting and ending coordinates with the
specified gradient type.

This tool requires information on the gradient type. It creates the
specified variety of gradient using the starting and ending
coordinates as defined for each gradient type. For shapeburst
gradient types, the context's distance metric is also relevant and
can be updated with `gimp_context_set_distance_metric()`.

This procedure is affected by the following context setters:
[`context_set_opacity()`][crate::context_set_opacity()], [`context_set_paint_mode()`][crate::context_set_paint_mode()],
[`context_set_foreground()`][crate::context_set_foreground()], [`context_set_background()`][crate::context_set_background()],
[`context_set_gradient()`][crate::context_set_gradient()] and all gradient property settings,
`gimp_context_set_distance_metric()`.
## `gradient_type`
The type of gradient.
## `offset`
Offset relates to the starting and ending coordinates specified for the blend. This parameter is mode dependent.
## `supersample`
Do adaptive supersampling.
## `supersample_max_depth`
Maximum recursion levels for supersampling.
## `supersample_threshold`
Supersampling threshold.
## `dither`
Use dithering to reduce banding.
## `x1`
The x coordinate of this gradient's starting point.
## `y1`
The y coordinate of this gradient's starting point.
## `x2`
The x coordinate of this gradient's ending point.
## `y2`
The y coordinate of this gradient's ending point.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn fill -->
Fill the drawable with the specified fill mode.

This procedure fills the drawable. If the fill mode is foreground
the current foreground color is used. If the fill mode is
background, the current background color is used. If the fill type
is white, then white is used. Transparent fill only affects layers
with an alpha channel, in which case the alpha channel is set to
transparent. If the drawable has no alpha channel, it is filled to
white. No fill leaves the drawable's contents undefined.
This procedure is unlike `gimp_drawable_edit_fill()` or the bucket
fill tool because it fills regardless of a selection. Its main
purpose is to fill a newly created drawable before adding it to the
image. This operation cannot be undone.
## `fill_type`
The type of fill.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn foreground_extract -->
Extract the foreground of a drawable using a given trimap.

Image Segmentation by Uniform Color Clustering, see
https://www.inf.fu-berlin.de/inst/pubs/tr-b-05-07.pdf
## `mode`
The algorithm to use.
## `mask`
Tri-Map.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn filters -->
Returns the list of filters applied to the drawable.

This procedure returns the list of filters which are currently
applied non-destructively to `self`. The order of filters is from
topmost to bottommost.

# Returns


 The list of filters on the drawable.
 The returned value must be freed with `g_free()`.
<!-- trait DrawableExt::fn format -->
Returns the `Babl` format of the drawable.

# Returns

The `Babl` format.
<!-- trait DrawableExt::fn thumbnail_format -->
Returns the `Babl` thumbnail format of the drawable.

# Returns

The `Babl` thumbnail format.
<!-- trait DrawableExt::fn histogram -->
Returns information on the intensity histogram for the specified
drawable.

This tool makes it possible to gather information about the
intensity histogram of a drawable. A channel to examine is first
specified. This can be either value, red, green, or blue, depending
on whether the drawable is of type color or grayscale. Second, a
range of intensities are specified. The `gimp_drawable_histogram()`
function returns statistics based on the pixels in the drawable that
fall under this range of values. Mean, standard deviation, median,
number of pixels, and percentile are all returned. Additionally, the
total count of pixels in the image is returned. Counts of pixels are
weighted by any associated alpha values and by the current selection
mask. That is, pixels that lie outside an active selection mask will
not be counted. Similarly, pixels with transparent alpha values will
not be counted. The returned mean, std_dev and median are in the
range (0..255) for 8-bit images or if the plug-in is not
precision-aware, and in the range (0.0..1.0) otherwise.
## `channel`
The channel to query.
## `start_range`
Start of the intensity measurement range.
## `end_range`
End of the intensity measurement range.

# Returns

TRUE on success.

## `mean`
Mean intensity value.

## `std_dev`
Standard deviation of intensity values.

## `median`
Median intensity value.

## `pixels`
Alpha-weighted pixel count for entire image.

## `count`
Alpha-weighted pixel count for range.

## `percentile`
Percentile that range falls under.
<!-- trait DrawableExt::fn hue_saturation -->
Modify hue, lightness, and saturation in the specified drawable.

This procedure allows the hue, lightness, and saturation in the
specified drawable to be modified. The 'hue-range' parameter
provides the capability to limit range of affected hues. The
'overlap' parameter provides blending into neighboring hue channels
when rendering.
## `hue_range`
Range of affected hues.
## `hue_offset`
Hue offset in degrees.
## `lightness`
Lightness modification.
## `saturation`
Saturation modification.
## `overlap`
Overlap other hue channels.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn levels -->
Modifies intensity levels in the specified drawable.

This tool allows intensity levels in the specified drawable to be
remapped according to a set of parameters. The low/high input levels
specify an initial mapping from the source intensities. The gamma
value determines how intensities between the low and high input
intensities are interpolated. A gamma value of 1.0 results in a
linear interpolation. Higher gamma values result in more high-level
intensities. Lower gamma values result in more low-level
intensities. The low/high output levels constrain the final
intensity mapping--that is, no final intensity will be lower than
the low output level and no final intensity will be higher than the
high output level. This tool is only valid on RGB color and
grayscale images.
## `channel`
The channel to modify.
## `low_input`
Intensity of lowest input.
## `high_input`
Intensity of highest input.
## `clamp_input`
Clamp input values before applying output levels.
## `gamma`
Gamma adjustment factor.
## `low_output`
Intensity of lowest output.
## `high_output`
Intensity of highest output.
## `clamp_output`
Clamp final output values.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn merge_filter -->
This procedure applies the specified drawable effect on `self`
and merge it (therefore before any non-destructive effects are
computed).

The `self` argument must be the same as the one used when you
created the effect with [ctor`Gimp`.new].
Once this is run, `filter` is not valid anymore and you should not
try to do anything with it. In particular, you must customize the
operation's arguments as received with
[method`Gimp`.get_config] or set the filter's opacity
and blend mode before merging the effect.
## `filter`
The drawable filter to merge.
<!-- trait DrawableExt::fn merge_new_filter -->
Utility function which combines [ctor`Gimp`.new]
followed by setting arguments for the
[class`Gimp`] returned by
[method`Gimp`.get_config], and finally applying the
effect to `self` with [method`Gimp`.merge_filter]

The variable arguments are couples of an argument name followed by a
value, NULL-terminated, such as:

**⚠️ The following code is in C ⚠️**

```C
filter = gimp_drawable_merge_new_filter (drawable,
                                         GIMP_LAYER_MODE_REPLACE, 1.0,
                                         "gegl:gaussian-blur", "My Gaussian Blur",
                                         "std-dev-x", 2.5,
                                         "std-dev-y", 2.5,
                                         "abyss-policy", "clamp",
                                         NULL);
```
## `operation_name`
The GEGL operation's name.
## `name`
The effect name which will show in undo step.
## `mode`
The blend mode.
## `opacity`
The opacity from 0.0 (transparent) to 1.0 (opaque).
<!-- trait DrawableExt::fn offset -->
Offset the drawable by the specified amounts in the X and Y
directions

This procedure offsets the specified drawable by the amounts
specified by 'offset_x' and 'offset_y'. If 'wrap_around' is set to
TRUE, then portions of the drawable which are offset out of bounds
are wrapped around. Alternatively, the undefined regions of the
drawable can be filled with transparency or the background color, as
specified by the 'fill-type' parameter.
## `wrap_around`
wrap image around or fill vacated regions.
## `fill_type`
fill vacated regions of drawable with background or transparent.
## `color`
fills in the background color when fill_type is set to OFFSET-COLOR.
## `offset_x`
offset by this amount in X direction.
## `offset_y`
offset by this amount in Y direction.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn threshold -->
Threshold the specified drawable.

This procedures generates a threshold map of the specified drawable.
All pixels between the values of 'low_threshold' and
'high_threshold', on the scale of 'channel' are replaced with white,
and all other pixels with black.
## `channel`
The channel to base the threshold on.
## `low_threshold`
The low threshold value.
## `high_threshold`
The high threshold value.

# Returns

TRUE on success.
<!-- trait DrawableExt::fn type_ -->
Returns the drawable's type.

This procedure returns the drawable's type.

# Returns

The drawable's type.
<!-- trait DrawableExt::fn type_with_alpha -->
Returns the drawable's type with alpha.

This procedure returns the drawable's type as if had an alpha
channel. If the type is currently Gray, for instance, the returned
type would be GrayA. If the drawable already has an alpha channel,
the drawable's type is simply returned.

# Returns

The drawable's type with alpha.
<!-- impl Font::fn pango_font_description -->
Returns a [struct`Pango`] representing `self`.

# Returns

a `PangoFontDescription` representing `self`.
<!-- impl Gradient::fn segment_get_blending_function -->
Gets the gradient segment's blending function

Gets the blending function of the segment at the index.
Returns an error when the segment index is out of range.
## `segment`
The index of a segment within the gradient.

# Returns

TRUE on success.

## `blend_func`
The blending function of the segment.
<!-- impl Gradient::fn segment_get_coloring_type -->
Gets the gradient segment's coloring type

Gets the coloring type of the segment at the index.
Returns an error when the segment index is out of range.
## `segment`
The index of a segment within the gradient.

# Returns

TRUE on success.

## `coloring_type`
The coloring type of the segment.
<!-- impl Gradient::fn segment_range_set_blending_function -->
Sets the blending function of a range of segments

Sets the blending function of a range of segments.
Returns an error when a segment index is out of range, or gradient
is not editable.
## `start_segment`
Index of the first segment to operate on.
## `end_segment`
Index of the last segment to operate on. If negative, the range will extend to the end segment.
## `blending_function`
The blending function.

# Returns

TRUE on success.
<!-- impl Gradient::fn segment_range_set_coloring_type -->
Sets the coloring type of a range of segments

Sets the coloring type of a range of segments.
Returns an error when a segment index is out of range, or gradient
is not editable.
## `start_segment`
Index of the first segment to operate on.
## `end_segment`
Index of the last segment to operate on. If negative, the range will extend to the end segment.
## `coloring_type`
The coloring type.

# Returns

TRUE on success.
<!-- trait ItemExt::fn color_tag -->
Get the color tag of the specified item.

This procedure returns the specified item's color tag.

# Returns

The item's color tag.
<!-- trait ItemExt::fn set_color_tag -->
Set the color tag of the specified item.

This procedure sets the specified item's color tag.
## `color_tag`
The new item color tag.

# Returns

TRUE on success.
<!-- impl Layer::fn new -->
Create a new layer.

This procedure creates a new layer with the specified `width`, `height`, and
`type_`. If `name` is [`None`], a default layer name will be used.
`opacity` and `mode` are also supplied parameters.

The new layer still needs to be added to the image, as this is not automatic.
Add the new layer with the [method`Image`] method.

Other attributes such as layer mask modes, and offsets should be set with
explicit procedure calls.
## `image`
The image to which to add the layer.
## `name`
The layer name.
## `width`
The layer width.
## `height`
The layer height.
## `type_`
The layer type.
## `opacity`
The layer opacity.
## `mode`
The layer combination mode.

# Returns

The newly created layer.
 The object belongs to libgimp and you should not free it.
<!-- impl Layer::fn from_surface -->
Create a new layer from a [type`cairo`].

This procedure creates a new layer from the given
[type`cairo`]. The image has to be an RGB image and just like
with `gimp_layer_new()` you will still need to add the layer to it.

If you pass `progress_end` > `progress_start` to this function,
[`progress_update()`][crate::progress_update()] will be called for. You have to call
[`progress_init()`][crate::progress_init()] beforehand then.
## `image`
The RGB image to which to add the layer.
## `name`
The layer name.
## `surface`
A Cairo image surface.
## `progress_start`
start of progress
## `progress_end`
end of progress

# Returns

The newly created layer.
 The object belongs to libgimp and you should not free it.
<!-- trait LayerExt::fn blend_space -->
Get the blend space of the specified layer.

This procedure returns the specified layer's blend space.

# Returns

The layer blend space.
<!-- trait LayerExt::fn composite_mode -->
Get the composite mode of the specified layer.

This procedure returns the specified layer's composite mode.

# Returns

The layer composite mode.
<!-- trait LayerExt::fn composite_space -->
Get the composite space of the specified layer.

This procedure returns the specified layer's composite space.

# Returns

The layer composite space.
<!-- trait LayerExt::fn remove_mask -->
Remove the specified layer mask from the layer.

This procedure removes the specified layer mask from the layer. If
the mask doesn't exist, an error is returned.
## `mode`
Removal mode.

# Returns

TRUE on success.
<!-- trait LayerExt::fn set_blend_space -->
Set the blend space of the specified layer.

This procedure sets the specified layer's blend space.
## `blend_space`
The new layer blend space.

# Returns

TRUE on success.
<!-- trait LayerExt::fn set_composite_mode -->
Set the composite mode of the specified layer.

This procedure sets the specified layer's composite mode.
## `composite_mode`
The new layer composite mode.

# Returns

TRUE on success.
<!-- trait LayerExt::fn set_composite_space -->
Set the composite space of the specified layer.

This procedure sets the specified layer's composite space.
## `composite_space`
The new layer composite space.

# Returns

TRUE on success.
<!-- impl Metadata::fn colorspace -->
Returns values based on Exif.Photo.ColorSpace, Xmp.exif.ColorSpace,
Exif.Iop.InteroperabilityIndex, Exif.Nikon3.ColorSpace,
Exif.Canon.ColorSpace of `self`.

# Returns

The colorspace specified by above tags.
<!-- impl Metadata::fn set_colorspace -->
Sets Exif.Photo.ColorSpace, Xmp.exif.ColorSpace,
Exif.Iop.InteroperabilityIndex, Exif.Nikon3.ColorSpace,
Exif.Canon.ColorSpace of `self`.
## `colorspace`
The color space.
<!-- impl Palette::fn colormap -->
This procedure returns a palette's colormap as an array of bytes with
all colors converted to a given Babl `format`.

The byte-size of the returned colormap depends on the number of
colors and on the bytes-per-pixel size of `format`. E.g. that the
following equality is ensured:

**⚠️ The following code is in C ⚠️**

```C
num_bytes == num_colors * babl_format_get_bytes_per_pixel (format)
```

Therefore `num_colors` and `num_bytes` are kinda redundant since both
indicate the size of the return value in a different way. You may
both set them to [`None`] but not at the same time.
## `format`
The desired color format.

# Returns

The palette's colormap.

## `num_colors`
The number of colors in the palette.
<!-- impl Palette::fn set_colormap -->
This procedure sets the entries in the specified palette in one go,
though they must all be in the same `format`.

The number of entries depens on the `num_bytes` size of `colormap` and
the bytes-per-pixel size of `format`.
The procedure will fail if `num_bytes` is not an exact multiple of the
number of bytes per pixel of `format`.
## `format`
The desired color format.
`colormap` (array length=num_bytes): The new colormap values.
## `num_bytes`
The byte-size of `colormap`.

# Returns

[`true`] on success.
<!-- impl Parasite::fn new -->
Creates a new parasite and save `data` which may be a proper text (in
which case you may want to set `size` as strlen(`data`) + 1) or not.
## `name`
the new [`Parasite`][crate::Parasite] name.
## `flags`
see libgimpbase/gimpparasite.h macros.
## `data`
the data to save in a parasite.

# Returns

a new [`Parasite`][crate::Parasite].
<!-- impl Parasite::fn data -->
Gets the parasite's data. It may not necessarily be text, nor is it
guaranteed to be [`None`]-terminated. It is your responsibility to know
how to deal with this data.
Even when you expect a nul-terminated string, it is advised not to
assume the returned data to be, as parasites can be edited by third
party scripts. You may end up reading out-of-bounds data. So you
should only ignore `num_bytes` when you all you care about is checking
if the parasite has contents.

# Returns

parasite's data.
<!-- impl Path::fn parse -->
## `path`
A list of directories separated by `G_SEARCHPATH_SEPARATOR`.
## `max_paths`
The maximum number of directories to return.
## `check`
[`true`] if you want the directories to be checked.

# Returns


 A `GList` of all directories in `path`.

## `check_failed`

 Returns a `GList` of path elements for which the check failed.
<!-- impl Path::fn stroke_get_points -->
returns the control points of a stroke.

returns the control points of a stroke. The interpretation of the
coordinates returned depends on the type of the stroke. For Gimp 2.4
this is always a bezier stroke, where the coordinates are the
control points.
## `stroke_id`
The stroke ID.

# Returns

type of the stroke (always GIMP_PATH_STROKE_TYPE_BEZIER for now).

## `controlpoints`
List of the control points for the stroke (x0, y0, x1, y1, ...).

## `closed`
Whether the stroke is closed or not.
<!-- impl Path::fn stroke_new_from_points -->
Adds a stroke of a given type to the path object.

Adds a stroke of a given type to the path object. The coordinates of
the control points can be specified. For now only strokes of the
type GIMP_PATH_STROKE_TYPE_BEZIER are supported. The control points
are specified as a pair of double values for the x- and
y-coordinate. The Bezier stroke type needs a multiple of three
control points. Each Bezier segment endpoint (anchor, A) has two
additional control points (C) associated. They are specified in the
order CACCACCAC...
## `type_`
type of the stroke (always GIMP_PATH_STROKE_TYPE_BEZIER for now).
## `controlpoints`
List of the x- and y-coordinates of the control points.
## `closed`
Whether the stroke is to be closed or not.

# Returns

The stroke ID of the newly created stroke.
<!-- impl Pattern::fn buffer -->
Gets pixel data of the pattern within the bounding box specified by `max_width`
and `max_height`. The data will be scaled down so that it fits within this
size without changing its ratio. If the pattern is smaller than this size to
begin with, it will not be scaled up.

If `max_width` or `max_height` are [`None`], the buffer is returned in the pattern's
native size.

Make sure you called [func`Gegl`] before calling any function using
`GEGL`.
## `max_width`
a maximum width for the returned buffer.
## `max_height`
a maximum height for the returned buffer.
## `format`
an optional Babl format.

# Returns

a [class`Gegl`].
<!-- impl PlugIn::fn directory_file -->
Returns a [`gio::File`][crate::gio::File] in the plug-in directory, or the plug-in directory
itself if `first_element` is [`None`].

See also: [`directory()`][Self::directory()].
## `first_element`
the first element of a path to a file in the
 plug-in directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
<!-- trait ProcedureExt::fn run -->
Runs the procedure named `procedure_name` with arguments given as
list of `(name, value)` pairs, terminated by [`None`].

The order of arguments does not matter and if any argument is missing, its
default value will be used. The value type must correspond to the argument
type as registered for `procedure_name`.
## `first_arg_name`
the name of an argument of `self` or [`None`] to
 run `self` with default arguments.

# Returns

the return values for the procedure call.
<!-- trait ProcedureExt::fn run_valist -->
Runs `self` with arguments names and values, given in the order as passed
to [method`Procedure`].
## `first_arg_name`
the name of an argument of `self` or [`None`] to
 run `self` with default arguments.
`args` the value of `first_arg_name` and any more argument
 names and values as needed.

# Returns

the return values for the procedure call.
<!-- trait ProcedureImpl::fn create_config -->
called when a `GimpConfig` object is created using
 [`ProcedureExt::create_config()`][crate::prelude::ProcedureExt::create_config()].
<!-- trait ProcedureImpl::fn install -->
called to install the procedure with the main GIMP
 application. This is an implementation detail and must never
 be called by any plug-in code.
<!-- trait ProcedureImpl::fn run -->
called when the procedure is executed via `gimp_procedure_run()`.
 the default implementation simply calls the procedure's `GimpRunFunc`,
 [`Procedure`][crate::Procedure] subclasses are free to modify the passed `args` and
 call their own, subclass-specific run functions.
<!-- trait ProcedureImpl::fn uninstall -->
called to uninstall the procedure from the main GIMP
 application. This is an implementation detail and must never
 be called by any plug-in code.
<!-- trait ProcedureConfigExt::fn core_object_array -->
A function for bindings to get a [type`CoreObjectArray`] property. Getting
these with [method`GObject`.get] or [method`GObject`.get_property] won't
[work for the time being](https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/492)
so all our boxed array types must be set and get using alternative
functions instead.

C plug-ins should just use [method`GObject`.get].
## `property_name`
the name of a [struct`ParamSpecCoreObjectArray`] param spec.

# Returns

an array of `GObjects`.
<!-- trait ProcedureConfigExt::fn set_core_object_array -->
A function for bindings to set a [type`CoreObjectArray`] property. Setting
these with [method`GObject`.set] or [method`GObject`.set_property] won't
[work for the time being](https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/492)
so all our boxed array types must be set and get using alternative
functions instead.

C plug-ins should just use [method`GObject`.set].
## `property_name`
the name of a [struct`ParamSpecCoreObjectArray`] param spec.
## `objects`
an array of `GObjects`.
<!-- impl ValueArray::fn from_types -->
Allocate and initialize a new [`ValueArray`][crate::ValueArray], and fill it with
values that are given as a list of (`GType`, value) pairs,
terminated by `G_TYPE_NONE`.
## `error_msg`
return location for an error message.
## `first_type`
first type in the array, or `G_TYPE_NONE`.

# Returns

a newly allocated [`ValueArray`][crate::ValueArray], or [`None`] if
 an error happened.
<!-- impl ValueArray::fn from_types_valist -->
Allocate and initialize a new [`ValueArray`][crate::ValueArray], and fill it with
`va_args` given in the order as passed to
`gimp_value_array_new_from_types()`.
## `error_msg`
return location for an error message.
## `first_type`
first type in the array, or `G_TYPE_NONE`.
## `va_args`
a va_list of GTypes and values, terminated by `G_TYPE_NONE`

# Returns

a newly allocated [`ValueArray`][crate::ValueArray], or [`None`] if
 an error happened.
<!-- impl ValueArray::fn from_values -->
Allocate and initialize a new [`ValueArray`][crate::ValueArray], and fill it with
the given `GValues`. When no `GValues` are given, returns empty [`ValueArray`][crate::ValueArray].
## `values`
The `GValue` elements

# Returns

a newly allocated [`ValueArray`][crate::ValueArray].
<!-- impl ValueArray::fn append -->
Insert a copy of `value` as last element of `self`. If `value` is
[`None`], an uninitialized value is appended.
## `value`
`GValue` to copy into [`ValueArray`][crate::ValueArray], or [`None`]

# Returns

the [`ValueArray`][crate::ValueArray] passed in as `self`
<!-- impl ValueArray::fn core_object_array -->
Return a pointer to the value at `index` contained in `self`. This value
is supposed to be a [type`CoreObjectArray`].

*Note*: most of the time, you should use the generic [method`Gimp`.index]
to retrieve a value, then the relevant `g_value_get_*()` function.
This alternative function is mostly there for bindings because
GObject-Introspection is [not able yet to process correctly known
boxed array types](https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/492).

There are no reasons to use this function in C code.
## `index`
index of the value of interest

# Returns

the [type`CoreObjectArray`] stored at `index` in `self`.
<!-- impl ValueArray::fn index -->
Return a pointer to the value at `index` contained in `self`.

*Note*: in binding languages, some custom types fail to be correctly passed
through. For these types, you should use specific functions.
For instance, in the Python binding, a [type`ColorArray`] `GValue`
won't be usable with this function. You should use instead
[method`ValueArray`].
## `index`
index of the value of interest

# Returns

pointer to a value at `index` in `self`
<!-- impl ValueArray::fn insert -->
Insert a copy of `value` at specified position into `self`. If `value`
is [`None`], an uninitialized value is inserted.
## `index`
insertion position, must be &lt;= [`length()`][Self::length()]
## `value`
`GValue` to copy into [`ValueArray`][crate::ValueArray], or [`None`]

# Returns

the [`ValueArray`][crate::ValueArray] passed in as `self`
<!-- impl ValueArray::fn prepend -->
Insert a copy of `value` as first element of `self`. If `value` is
[`None`], an uninitialized value is prepended.
## `value`
`GValue` to copy into [`ValueArray`][crate::ValueArray], or [`None`]

# Returns

the [`ValueArray`][crate::ValueArray] passed in as `self`
