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
<!-- fn checks_get_colors -->
Retrieves the colors to use when drawing a checkerboard for a certain
[`CheckType`][crate::CheckType] and custom colors.
If `type_` is [`CheckType::CustomChecks`][crate::CheckType::CustomChecks], then `color1` and `color2`
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
<!-- fn directory_file -->
Returns a [`gio::File`][crate::gio::File] in the user's GIMP directory, or the GIMP
directory itself if `first_element` is [`None`].

See also: [`directory()`][crate::directory()].
## `first_element`
the first element of a path to a file in the
 user's GIMP directory, or [`None`].

# Returns


 a new [`gio::File`][crate::gio::File] for the path, Free with `g_object_unref()`.
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
a [`glib::EnumValue`][crate::glib::EnumValue] from `enum_class`

# Returns

the translated abbreviation of the enum value
<!-- fn enum_value_get_desc -->
Retrieves the translated description for a given `enum_value`.
## `enum_class`
a `GEnumClass`
## `enum_value`
a [`glib::EnumValue`][crate::glib::EnumValue] from `enum_class`

# Returns

the translated description of the enum value
<!-- fn enum_value_get_help -->
Retrieves the translated help for a given `enum_value`.
## `enum_class`
a `GEnumClass`
## `enum_value`
a [`glib::EnumValue`][crate::glib::EnumValue] from `enum_class`

# Returns

the translated help of the enum value
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
a [`glib::FlagsValue`][crate::glib::FlagsValue] from `flags_class`

# Returns

the translated abbreviation of the flags value
<!-- fn flags_value_get_desc -->
Retrieves the translated description for a given `flags_value`.
## `flags_class`
a `GFlagsClass`
## `flags_value`
a [`glib::FlagsValue`][crate::glib::FlagsValue] from `flags_class`

# Returns

the translated description of the flags value
<!-- fn flags_value_get_help -->
Retrieves the translated help for a given `flags_value`.
## `flags_class`
a `GFlagsClass`
## `flags_value`
a [`glib::FlagsValue`][crate::glib::FlagsValue] from `flags_class`

# Returns

the translated help of the flags value
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
[flags[`glib::Object`][crate::glib::Object]].
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
[flags[`glib::Object`][crate::glib::Object]] (see libgimpconfig/gimpconfig-params.h).
<!-- const PARAM_NO_VALIDATE -->
Since 3.0
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
<!-- impl ColorProfile::fn lcms_profile -->
This function returns `self`'s cmsHPROFILE. The returned
value belongs to `self` and must not be modified or freed.

# Returns

a pointer to the cmsHPROFILE.
<!-- impl ColorTransform::fn process_pixels -->
This function transforms a contiguous line of pixels.

See [`new()`][Self::new()]: only the pixel encoding of
`src_format` and `dest_format` is honored, their color spaces are
ignored. The transform always takes place between the color spaces
determined by `self`'s color profiles.
## `src_format`
`Babl` format of `src_pixels`
## `src_pixels`
pointer to the source pixels
## `dest_format`
`Babl` format of `dest_pixels`
## `dest_pixels`
pointer to the destination pixels
## `length`
number of pixels to process
<!-- impl ConfigWriter::fn printf -->
A printf-like function for [`ConfigWriter`][crate::ConfigWriter].
## `format`
a format string as described for `g_strdup_printf()`.
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
<!-- impl ExportOptions::fn image -->
Takes an image to be exported, possibly creating a temporary copy
modified according to export settings in `self` (such as the
capabilities of the export format).

If necessary, a copy is created, converted and modified, `image`
changed to point to the new image and the procedure returns
[enum`Gimp`.EXPORT].
In this case, you must take care of deleting the created image using
[method`Image`] once the image has been exported, unless you
were planning to display it with [ctor`Display`], or you will leak
memory.

If [enum`Gimp`.IGNORE] is returned, then `image` is still the
original image. You should neither modify it, nor should you delete
it in the end. If you wish to temporarily modify the image before
export anyway, call [method`Image`] when
[enum`Gimp`.IGNORE] was returned.
## `image`
the image.

# Returns

An enum of [`ExportReturn`][crate::ExportReturn].
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
