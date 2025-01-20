pub mod plug_in;

pub mod prelude {
    pub use macros::object_subclass_impl;
    pub use super::plug_in::PlugInImpl;
}
