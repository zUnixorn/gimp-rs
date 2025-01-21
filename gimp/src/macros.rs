pub mod private {
    // From https://github.com/MaxXSoft/build_assert
    // This is basically a 1:1 copy, it just adds a panic to the release version and the possibility to add a message to the build error

    /// Raises a build-time error.
    ///
    /// This macro will stop the compilation process in release mode, and always
    /// panic in debug mode.
    ///
    #[cfg(debug_assertions)]
    macro_rules! build_error {
    () => {
        core::panic!()
    };
    ($message:literal) => {
        core::panic!($message)
    };
}

    /// Raises a build-time error.
    ///
    /// This macro will stop the compilation process in release mode, and always
    /// panic in debug mode.
    #[cfg(not(debug_assertions))]
    macro_rules! build_error {
        () => {
            unsafe {
                core::arch::asm!(
                    core::concat!(
                        "build error at ",
                        core::file!(),
                        ":",
                        core::line!(),
                        ":",
                        core::column!(),
                    )
                );
                core::panic!()
            }
        };
        ($message:literal) => {
            unsafe {
                core::arch::asm!(
                    core::concat!(
                        "build error at ",
                        core::file!(),
                        ":",
                        core::line!(),
                        ":",
                        core::column!(),
                        ": ",
                        $message,
                    )
                );
                core::panic!()
            }
        };
    }

    pub(crate) use build_error;
}

/// Generates the necessary main for plugins like the `GIMP_MAIN` macro in libgimp
/// The macro takes a type which subclasses [`crate::PlugIn`] and implements [`crate::subclass::plug_in::PlugInImpl`]
#[macro_export]
macro_rules! main {
    ($plug_in:ty) => {
        use gimp::glib::types::StaticType;

        fn main() -> std::process::ExitCode {
            let args: Vec<String> = std::env::args().collect::<Vec<_>>();
            let argv = args.iter().map(String::as_str).collect::<Vec<_>>();

            std::process::ExitCode::from($crate::functions::main(<$plug_in>::static_type(), argv.as_slice()) as u8)
        }
    }
}
