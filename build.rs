extern crate cbindgen;

use cbindgen::Config;
use cbindgen::Language;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let cpp_header_output_file = target_dir()
        .join(format!("include/{}/{}.hpp", package_name, package_name))
        .display()
        .to_string();

    let cpp_config = Config {
        namespace: Some(package_name),
        language: Language::Cxx,
        pragma_once: true,
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, cpp_config)
        .unwrap()
        .write_to_file(&cpp_header_output_file);

    println!("cargo:rerun-if-changed=build.rs");
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
