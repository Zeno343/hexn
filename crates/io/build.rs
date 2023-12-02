fn main() {
    println!("cargo:rerun-if-changed=sdl.h");
    println!("cargo:rustc-link-lib=SDL2");

    let out_dir = std::env::var("OUT_DIR").expect("$OUT_DIR undfined");
    let out_path = std::path::PathBuf::from(out_dir);

    bindgen::builder()
        .use_core()
        .ctypes_prefix("core::ffi")
        .prepend_enum_name(false)
        .header("sdl.h")
        .blocklist_item("FP_.*")
        .generate()
        .expect("failed to generate sdl.h binding")
        .write_to_file(out_path.join("sdl.rs"))
        .expect("failed to write sdl.rs");
}
