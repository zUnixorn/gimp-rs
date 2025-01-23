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
<!-- impl LoadProcedure::fn new -->
Creates a new load procedure named `name` which will call `run_func`
when invoked.

See [`Procedure::new()`][crate::Procedure::new()] for information about `proc_type`.
## `plug_in`
a [`PlugIn`][crate::PlugIn].
## `name`
the new procedure's name.
## `proc_type`
the new procedure's [`PDBProcType`][crate::PDBProcType].
## `run_func`
the run function for the new procedure.
## `run_data`
user data passed to `run_func`.
## `run_data_destroy`
free function for `run_data`, or [`None`].

# Returns

a new [`Procedure`][crate::Procedure].
<!-- trait ModuleExt::fn info -->

# Returns

The `self`'s `GimpModuleInfo` as provided
 by the actual module, or [`None`].
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
<!-- impl Vector2::fn new -->
Creates a [struct`Vector2`] of coordinates `x` and `y`.
## `x`
the X coordinate.
## `y`
the Y coordinate.

# Returns

the resulting vector
<!-- impl Vector2::fn add_val -->
This function is identical to [`add()`][Self::add()] but the vectors
are passed by value rather than by reference.
## `vector2`
the second [`Vector2`][crate::Vector2].

# Returns

the resulting [`Vector2`][crate::Vector2].
<!-- impl Vector2::fn cross_product -->
Compute the cross product of two vectors. The result is a
[`Vector2`][crate::Vector2] which is orthogonal to both `self` and `vector2`. If
`self` and `vector2` are parallel, the result will be the nul
vector.

Note that in 2D, this function is useful to test if two vectors are
parallel or not, or to compute the area spawned by two vectors.
## `vector2`
a pointer to the second [`Vector2`][crate::Vector2].

# Returns

The cross product.
<!-- impl Vector2::fn cross_product_val -->
Identical to [method`Vector2`], but the
vectors are passed by value rather than by reference.
## `vector2`
the second [`Vector2`][crate::Vector2].

# Returns

The cross product.
<!-- impl Vector2::fn inner_product -->
Computes the inner (dot) product of two 2D vectors.
This product is zero if and only if the two vectors are orthogonal.
## `vector2`
a pointer to the second [`Vector2`][crate::Vector2].

# Returns

The inner product.
<!-- impl Vector2::fn inner_product_val -->
Identical to [method`Vector2`], but the
vectors are passed by value rather than by reference.
## `vector2`
the second [`Vector2`][crate::Vector2].

# Returns

The inner product.
<!-- impl Vector2::fn length -->
Computes the length of a 2D vector.

# Returns

the length of `self` (a positive gdouble).
<!-- impl Vector2::fn length_val -->
Identical to [method`Vector2`], but the vector is passed by value
rather than by reference.

# Returns

the length of `self` (a positive gdouble).
<!-- impl Vector2::fn mul -->
Multiplies each component of the `self` by `factor`. Note that this
is equivalent to multiplying the vectors length by `factor`.
## `factor`
a scalar.
<!-- impl Vector2::fn mul_val -->
Identical to [method`Vector2`], but the vector is passed by value rather
than by reference.
## `factor`
a scalar.

# Returns

the resulting [`Vector2`][crate::Vector2].
<!-- impl Vector2::fn neg -->
Negates the `self` (i.e. negate all its coordinates).
<!-- impl Vector2::fn neg_val -->
Identical to [method`Vector2`], but the vector
is passed by value rather than by reference.

# Returns

the negated [`Vector2`][crate::Vector2].
<!-- impl Vector2::fn normal -->
Compute a normalized perpendicular vector to `self`

# Returns

a [`Vector2`][crate::Vector2] perpendicular to `self`, with a length of 1.0.
<!-- impl Vector2::fn normal_val -->
Identical to [method`Vector2`], but the vector
is passed by value rather than by reference.

# Returns

a [`Vector2`][crate::Vector2] perpendicular to `self`, with a length of 1.0.
<!-- impl Vector2::fn normalize -->
Normalizes the `self` so the length of the `self` is 1.0. The nul
vector will not be changed.
<!-- impl Vector2::fn normalize_val -->
Identical to [method`Vector2`], but the
vector is passed by value rather than by reference.

# Returns

a [`Vector2`][crate::Vector2] parallel to `self`, pointing in the same
direction but with a length of 1.0.
<!-- impl Vector2::fn rotate -->
Rotates the `self` counterclockwise by `alpha` radians.
## `alpha`
an angle (in radians).
<!-- impl Vector2::fn rotate_val -->
Identical to [method`Vector2`], but the vector
is passed by value rather than by reference.
## `alpha`
an angle (in radians).

# Returns

a [`Vector2`][crate::Vector2] representing `self` rotated by `alpha`
radians.
<!-- impl Vector2::fn set -->
Sets the X and Y coordinates of `self` to `x` and `y`.
## `x`
the X coordinate.
## `y`
the Y coordinate.
<!-- impl Vector2::fn sub_val -->
This function is identical to [`sub()`][Self::sub()] but the vectors
are passed by value rather than by reference.
## `vector2`
the second [`Vector2`][crate::Vector2].

# Returns

the resulting [`Vector2`][crate::Vector2].
<!-- impl Vector2::fn add -->
Computes the sum of two 2D vectors. The resulting [`Vector2`][crate::Vector2] is
stored in `result`.
## `vector1`
a pointer to the first [`Vector2`][crate::Vector2].
## `vector2`
a pointer to the second [`Vector2`][crate::Vector2].

# Returns


## `result`
destination for the resulting [`Vector2`][crate::Vector2].
<!-- impl Vector2::fn sub -->
Computes the difference of two 2D vectors (`vector1` minus `vector2`).
The resulting [`Vector2`][crate::Vector2] is stored in `result`.
## `vector1`
a pointer to the first [`Vector2`][crate::Vector2].
## `vector2`
a pointer to the second [`Vector2`][crate::Vector2].

# Returns


## `result`
the destination for the resulting [`Vector2`][crate::Vector2].
<!-- impl Vector3::fn new -->
Creates a [`Vector3`][crate::Vector3] of coordinate `x`, `y` and `z`.
## `x`
the X coordinate.
## `y`
the Y coordinate.
## `z`
the Z coordinate.

# Returns

the resulting [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn add_val -->
This function is identical to [`add()`][Self::add()] but the vectors
are passed by value rather than by reference.
## `vector2`
a [`Vector3`][crate::Vector3].

# Returns

the resulting [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn cross_product -->
Compute the cross product of two vectors. The result is a
[`Vector3`][crate::Vector3] which is orthogonal to both `self` and `vector2`. If
`self` and `vector2` and parallel, the result will be the nul
vector.

This function can be used to compute the normal of the plane
defined by `self` and `vector2`.
## `vector2`
a pointer to the second [`Vector3`][crate::Vector3].

# Returns

The cross product.
<!-- impl Vector3::fn cross_product_val -->
Identical to [method`Vector3`], but the
vectors are passed by value rather than by reference.
## `vector2`
the second [`Vector3`][crate::Vector3].

# Returns

The cross product.
<!-- impl Vector3::fn inner_product -->
Computes the inner (dot) product of two 3D vectors. This product
is zero if and only if the two vectors are orthogonal.
## `vector2`
a pointer to the second [`Vector3`][crate::Vector3].

# Returns

The inner product.
<!-- impl Vector3::fn inner_product_val -->
Identical to [method`Vector3`], but the
vectors are passed by value rather than by reference.
## `vector2`
the second [`Vector3`][crate::Vector3].

# Returns

The inner product.
<!-- impl Vector3::fn length -->
Computes the length of a 3D vector.

# Returns

the length of `self` (a positive gdouble).
<!-- impl Vector3::fn length_val -->
Identical to [method`Vector3`], but the vector
is passed by value rather than by reference.

# Returns

the length of `self` (a positive gdouble).
<!-- impl Vector3::fn mul -->
Multiplies each component of the `self` by `factor`. Note that
this is equivalent to multiplying the vectors length by `factor`.
## `factor`
a scalar.
<!-- impl Vector3::fn mul_val -->
Identical to [method`Vector3`], but the vector is
passed by value rather than by reference.
## `factor`
a scalar.

# Returns

the resulting [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn neg -->
Negates the `self` (i.e. negate all its coordinates).
<!-- impl Vector3::fn neg_val -->
Identical to [method`Vector3`], but the vector
is passed by value rather than by reference.

# Returns

the negated [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn normalize -->
Normalizes the `self` so the length of the `self` is 1.0. The nul
vector will not be changed.
<!-- impl Vector3::fn normalize_val -->
Identical to [method`Vector3`], but the
vector is passed by value rather than by reference.

# Returns

a [`Vector3`][crate::Vector3] parallel to `self`, pointing in the same
direction but with a length of 1.0.
<!-- impl Vector3::fn rotate -->
Rotates the `self` around the three axis (Z, Y, and X) by `alpha`,
`beta` and `gamma`, respectively.

Note that the order of the rotation is very important. If you
expect a vector to be rotated around X, and then around Y, you will
have to call this function twice. Also, it is often wise to call
this function with only one of `alpha`, `beta` and `gamma` non-zero.
## `alpha`
the angle (in radian) of rotation around the Z axis.
## `beta`
the angle (in radian) of rotation around the Y axis.
## `gamma`
the angle (in radian) of rotation around the X axis.
<!-- impl Vector3::fn rotate_val -->
Identical to [method`Vector3`], but the vectors
are passed by value rather than by reference.
## `alpha`
the angle (in radian) of rotation around the Z axis.
## `beta`
the angle (in radian) of rotation around the Y axis.
## `gamma`
the angle (in radian) of rotation around the X axis.

# Returns

the rotated vector.
<!-- impl Vector3::fn set -->
Sets the X, Y and Z coordinates of `self` to `x`, `y` and `z`.
## `x`
the X coordinate.
## `y`
the Y coordinate.
## `z`
the Z coordinate.
<!-- impl Vector3::fn sub_val -->
This function is identical to [`sub()`][Self::sub()] but the vectors
are passed by value rather than by reference.
## `vector2`
a [`Vector3`][crate::Vector3].

# Returns

the resulting [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn add -->
Computes the sum of two 3D vectors. The resulting [`Vector3`][crate::Vector3] is
stored in `result`.
## `vector1`
a pointer to the first [`Vector3`][crate::Vector3].
## `vector2`
a pointer to the second [`Vector3`][crate::Vector3].

# Returns


## `result`
destination for the resulting [`Vector3`][crate::Vector3].
<!-- impl Vector3::fn sub -->
Computes the difference of two 3D vectors (`vector1` minus `vector2`).
The resulting [`Vector3`][crate::Vector3] is stored in `result`.
## `vector1`
a pointer to the first [`Vector3`][crate::Vector3].
## `vector2`
a pointer to the second [`Vector3`][crate::Vector3].

# Returns


## `result`
the destination for the resulting [`Vector3`][crate::Vector3].
<!-- impl VectorLoadProcedure::fn new -->
Creates a new load procedure named `name` which will call `run_func`
when invoked.

See [`Procedure::new()`][crate::Procedure::new()] for information about `proc_type`.
## `plug_in`
a [`PlugIn`][crate::PlugIn].
## `name`
the new procedure's name.
## `proc_type`
the new procedure's [`PDBProcType`][crate::PDBProcType].
## `run_func`
the run function for the new procedure.
## `run_data`
user data passed to `run_func`.
## `run_data_destroy`
free function for `run_data`, or [`None`].

# Returns

a new [`Procedure`][crate::Procedure].
<!-- impl VectorLoadProcedure::fn extract_dimensions -->
Extracts native or suggested dimensions from `file`, which must be a vector
file in the right format supported by `self`. It is considered a
programming error to pass a file of invalid format.
## `file`
a [iface`Gio`] which can be processed by `self`.

# Returns

[`true`] if dimensions could be extracted.

## `data`
the returned dimension data.
