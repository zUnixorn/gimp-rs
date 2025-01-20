pub mod plug_in;

pub mod prelude {
    pub use gimp_macros::object_subclass_impl;
    pub use super::plug_in::PlugInImpl;
}
