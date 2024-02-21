use std::path::PathBuf;

pub fn register_linker_script(path: &str) {
    let manifest_dir_path =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set by cargo");

    let linker_path = PathBuf::from_iter([&manifest_dir_path, path]);

    println!(
        "cargo:rustc-link-arg=-T{}",
        linker_path
            .as_os_str()
            .to_str()
            .expect("Cannot find path to linker file")
    );
}
