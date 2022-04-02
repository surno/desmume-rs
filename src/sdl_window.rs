use std::marker::PhantomData;
use crate::DeSmuMEError;
use crate::ffi::*;

/// A window that displays the emulator and processes touchscreen and keyboard inputs
/// (default keyboard configuration only).
///
/// This is meant to be a simple way to use and test the library, for intergration in custom UIs
/// you probably want to process input and display manually.
pub struct DeSmuMESdlWindow(PhantomData<()>);

impl DeSmuMESdlWindow {
    pub(crate) fn new(auto_pause: bool, use_opengl_if_possible: bool) -> Result<DeSmuMESdlWindow, DeSmuMEError> {
        unsafe {
            if desmume_draw_window_init(
                auto_pause as c_bool,
                use_opengl_if_possible as c_bool
            ) < 0 {
                return Err(DeSmuMEError::FailedInitWindow)
            }
        }
        Ok(Self(PhantomData))
    }

    /// Draw the current framebuffer to the window.
    pub fn draw(&mut self) {
        unsafe { desmume_draw_window_frame() }
    }

    /// Process the touchscreen input for the current cycle.
    pub fn process_input(&mut self) {
        unsafe { desmume_draw_window_input() }
    }

    /// Returns true, when the window was closed by the user.
    pub fn has_quit(&mut self) -> bool {
        unsafe { desmume_draw_window_has_quit() > 0 }
    }
}

impl Drop for DeSmuMESdlWindow {
    fn drop(&mut self) {
        unsafe { desmume_draw_window_free() }
    }
}
