use std::ffi::{c_char, CStr, CString};
use glib::{GStr, GString};
use glib::object::IsA;
use glib::translate::ToGlibPtr;
use glib::translate::IntoGlibPtr;
use glib::subclass::prelude::*;
use glib_sys::{g_list_append, gpointer};
use crate::PlugIn;
use crate::Procedure;

pub trait PlugInImpl: ObjectImpl + ObjectSubclass<Type: IsA<PlugIn>> {
    /// This method must be overridden by all plug-ins and return a newly
    /// allocated [`Procedure`][crate::Procedure] named `name`.
    ///
    /// This method will be called for every `name` as returned by
    /// [vfunc`PlugIn`] and [vfunc`PlugIn`] so care
    /// must be taken to handle them all. Upon procedure registration,
    /// [vfunc`PlugIn`] will be called in the order of the lists
    /// returned by [vfunc`PlugIn`] and
    /// [vfunc`PlugIn`]
    /// ## `procedure_name`
    /// procedure name.
    ///
    /// # Returns
    ///
    /// the procedure to be registered or executed by `self`.
    fn create_procedure(&self, procedure_name: &str) -> Option<impl IsA<Procedure>>;
    /// This method can be overridden by all plug-ins to return a newly allocated
    /// list of allocated strings naming procedures registered by this plug-in.
    /// It is different from [vfunc`PlugIn`] in that init happens
    /// at every startup, whereas query happens only once in the life of a plug-in
    /// (right after installation or update). Hence [vfunc`PlugIn`]
    /// typically returns procedures dependent to runtime conditions (such as the
    /// presence of a third-party tool), whereas [vfunc`PlugIn`]
    /// would usually return procedures that are always available unconditionally.
    ///
    /// Most of the time, you only want to override
    /// [vfunc`PlugIn`] and leave [vfunc`PlugIn`]
    /// untouched.
    ///
    /// # Returns
    ///
    /// the names of the procedures registered by `self`.
    fn init_procedures(&self) -> Vec<String>;
    /// This method can be overridden by all plug-ins to return a newly allocated
    /// list of allocated strings naming the procedures registered by this
    /// plug-in. See documentation of [vfunc`PlugIn`] for
    /// differences.
    ///
    /// # Returns
    ///
    /// the names of the procedures registered by `self`.
    fn query_procedures(&self) -> Vec<String>;
    /// This method can be overridden by a plug-in which needs to perform some
    /// actions upon quitting.
    fn quit(&self);
    /// This method can be overridden by all plug-ins to customize
    /// internationalization of the plug-in.
    ///
    /// This method will be called before initializing, querying or running
    /// `procedure_name` (respectively with [vfunc`PlugIn`],
    /// [vfunc`PlugIn`] or with the ``run()`` function set in
    /// `[`ImageProcedure::new()`][crate::ImageProcedure::new()]`).
    ///
    /// By default, GIMP plug-ins look up gettext compiled message catalogs
    /// in the subdirectory `locale/` under the plug-in folder (same folder
    /// as `[`progname()`][crate::progname()]`) with a text domain equal to the plug-in
    /// name (regardless `procedure_name`). It is unneeded to override this
    /// method if you follow this localization scheme.
    ///
    /// If you wish to disable localization or localize with another system,
    /// simply set the method to [`None`], or possibly implement this method
    /// to do something useful for your usage while returning [`false`].
    ///
    /// If you wish to tweak the `gettext_domain` or the `catalog_dir`, return
    /// [`true`] and allocate appropriate `gettext_domain` and/or `catalog_dir`
    /// (these use the default if set [`None`]).
    ///
    /// Note that `catalog_dir` must be a relative path, encoded as UTF-8,
    /// subdirectory of the directory of `[`progname()`][crate::progname()]`.
    /// The domain names "gimp30-std-plug-ins", "gimp30-script-fu" and
    /// "gimp30-python" are reserved and can only be used with a [`None`]
    /// `catalog_dir`. These will use the translation catalogs installed for
    /// core plug-ins, so you are not expected to use these for your
    /// plug-ins, except if you are making a core plug-in. More domain
    /// names may become reserved so we discourage using a gettext domain
    /// starting with "gimp30-".
    ///
    /// When localizing your plug-in this way, GIMP also binds
    /// `gettext_domain` to the UTF-8 encoding.
    /// ## `procedure_name`
    /// procedure name.
    ///
    /// # Returns
    ///
    /// [`true`] if this plug-in will use Gettext localization. You
    ///  may return [`false`] if you wish to disable localization or
    ///  set it up differently.
    ///
    /// ## `gettext_domain`
    /// Gettext domain. If [`None`], it
    ///  defaults to the plug-in name as determined by the
    ///  directory the binary is called from.
    ///
    /// ## `catalog_dir`
    /// relative path to a
    ///  subdirectory of the plug-in folder containing the compiled
    ///  Gettext message catalogs. If [`None`], it defaults to
    ///  "locale/".
    fn set_i18n(&self, procedure_name: String, gettext_domain: String, catalog_dir: String) -> bool;
}

unsafe impl<T: PlugInImpl> IsSubclassable<T> for PlugIn {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.create_procedure = Some(plug_in_create_procedure::<T>);
        klass.query_procedures = Some(plug_in_query_procedures::<T>);
    }
}

unsafe extern "C" fn plug_in_create_procedure<T: PlugInImpl>(ptr: *mut ffi::GimpPlugIn, procedure_name: *const c_char) -> *mut ffi::GimpProcedure {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // TODO should invalid utf-8 procedure names just be skipped and logged or panic?
    imp.create_procedure(CStr::from_ptr(procedure_name).to_str().expect("procedure name is not valid utf-8")).into_glib_ptr() as *mut ffi::GimpProcedure
}

unsafe extern "C" fn plug_in_query_procedures<T: PlugInImpl>(ptr: *mut ffi::GimpPlugIn) -> *mut glib::ffi::GList {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // TODO find a nice way, maybe with imp.query_procedures().to_glib_container().0
    let mut list: *mut glib::ffi::GList = std::ptr::null_mut();
    for procedure in imp.query_procedures() {
        list = g_list_append(list, procedure.leak().as_ptr() as gpointer);
    }

    list
}
