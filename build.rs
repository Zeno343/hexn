fn main() {
    if cfg!(debug_assertions) {
        println!(r#"cargo:rustc-cfg=feature="std""#);
    } else if cfg!(not(feature = "std")) {
        println!("cargo:rustc-link-arg-bin=hexn=-nostartfiles");
        println!("cargo:rustc-link-lib=c");
    }
}
