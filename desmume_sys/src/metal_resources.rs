// Embed the compiled Metal shader library
// This will be available at OUT_DIR/default.metallib after build
// Note: The build script must copy the metallib to OUT_DIR before Rust compilation
// If the file doesn't exist, we'll use an empty array and fall back to file-based loading
#[cfg(target_os = "macos")]
const METAL_LIBRARY: &[u8] = {
    // include_bytes! requires the file to exist at compile time
    // If it doesn't exist, the build will fail, but that's okay - we want to know
    // The build script should ensure the file is copied before Rust compilation
    include_bytes!(concat!(env!("OUT_DIR"), "/default.metallib"))
};

#[cfg(target_os = "macos")]
/// Get the embedded Metal library data
pub fn get_metal_library() -> &'static [u8] {
    METAL_LIBRARY
}

#[cfg(not(target_os = "macos"))]
/// Empty implementation for non-macOS platforms
pub fn get_metal_library() -> &'static [u8] {
    &[]
}
