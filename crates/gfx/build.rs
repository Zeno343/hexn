fn main() {
    println!("cargo:rerun-if-changed=gl.h");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=m");

    let out_dir = std::env::var("OUT_DIR").expect("$OUT_DIR undfined");
    let out_path = std::path::PathBuf::from(out_dir);

    bindgen::builder()
        .use_core()
        .ctypes_prefix("core::ffi")
        .prepend_enum_name(false)
        .header("gl.h")
        .generate()
        .expect("failed to generate gl.h binding")
        .write_to_file(out_path.join("gl.rs"))
        .expect("failed to write gl.rs");
}
