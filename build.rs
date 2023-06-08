use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") && cfg!(target_pointer_width = "32") {
        // Needed for 32bit version of zlib
        println!("cargo:rustc-link-arg=/SAFESEH:NO");
    }
}
