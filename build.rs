use std::collections::HashSet;
use std::env;
use std::path::PathBuf;


fn main() {
    println!("cargo:rustc-link-lib=gimp-2.0");
    println!("cargo:rustc-link-lib=glib-2.0");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")

        .clang_arg("-I/usr/include/gimp-2.0")
        .clang_arg("-I/usr/include/cairo")
        .clang_arg("-I/usr/include/gegl-0.4")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        .clang_arg("-I/usr/include/babl-0.1")
        .clang_arg("-I/usr/include/gdk-pixbuf-2.0")

        .parse_callbacks(Box::new(IgnoreMacros::with_ignored()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl IgnoreMacros {
    fn with_ignored() -> Self {
        Self(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
            ]
                .into_iter()
                .collect(),
        )
    }
}

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(format!("````text\n{}\n````", comment))
    }
}