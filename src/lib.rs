use crate::ffi::*;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::slice_from_raw_parts;
#[macro_use]
mod macros;

mod err;
mod ffi;
pub mod input;
pub mod mem;
mod movie;
mod savestate;
mod sdl_window;

pub use crate::err::DeSmuMEError;
pub use crate::input::DeSmuMEInput;
pub use crate::mem::DeSmuMEMemory;
pub use crate::movie::DeSmuMEMovie;
pub use crate::savestate::DeSmuMESavestate;
pub use crate::sdl_window::DeSmuMESdlWindow;

static mut WAS_EVER_ALREADY_INITIALIZED: bool = false;
static mut ALREADY_INITIALIZED: bool = false;
pub const SCREEN_WIDTH: usize = GPU_FRAMEBUFFER_NATIVE_WIDTH;
pub const SCREEN_HEIGHT: usize = GPU_FRAMEBUFFER_NATIVE_HEIGHT;
pub const SCREEN_HEIGHT_BOTH: usize = SCREEN_HEIGHT * 2;
pub const SCREEN_PIXEL_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const SCREEN_PIXEL_SIZE_BOTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT_BOTH;
pub const NB_STATES: usize = 10;

/// Firmware language
#[repr(u8)]
pub enum Language {
    Japanese = 0,
    English = 1,
    French = 2,
    German = 3,
    Italian = 4,
    Spanish = 5,
}

/// The DeSmuME emulator.
pub struct DeSmuME {
    input: DeSmuMEInput,
    memory: DeSmuMEMemory,
    movie: DeSmuMEMovie,
    savestate: DeSmuMESavestate,
    window: Option<DeSmuMESdlWindow>,
}

impl DeSmuME {
    /// Initializes DeSmuME and returns the struct managing it's state.
    ///
    /// **Note:**
    ///     At the current time only one instance of DeSmuME can be initialized.
    ///     Trying to call init before the previous instance is dropped will return an error.
    ///     This is done this way to be forward-compatible if we ever support multiple
    ///     running instances.
    ///     Additionally note that DeSmuME is not free'd at the moment when dropping it, this is
    ///     to allow future instances of DeSmuME being created again later on. Use [`free_desmume`]
    ///     to manually free resources created by the library.
    pub fn init() -> Result<DeSmuME, DeSmuMEError> {
        unsafe {
            if ALREADY_INITIALIZED {
                return Err(DeSmuMEError::AlreadyInit);
            }
            ALREADY_INITIALIZED = true;
            desmume_set_savetype(0);
            if !WAS_EVER_ALREADY_INITIALIZED {
                if desmume_init() < 0 {
                    return Err(DeSmuMEError::FailedInit);
                }
                WAS_EVER_ALREADY_INITIALIZED = true;
            }
        }
        Ok(Self {
            input: DeSmuMEInput {
                joystick_was_init: false,
            },
            memory: DeSmuMEMemory(PhantomData),
            movie: DeSmuMEMovie(PhantomData),
            savestate: DeSmuMESavestate(PhantomData),
            window: None,
        })
    }

    pub fn input(&self) -> &DeSmuMEInput {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut DeSmuMEInput {
        &mut self.input
    }

    pub fn memory(&self) -> &DeSmuMEMemory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut DeSmuMEMemory {
        &mut self.memory
    }

    pub fn movie(&self) -> &DeSmuMEMovie {
        &self.movie
    }

    pub fn movie_mut(&mut self) -> &mut DeSmuMEMovie {
        &mut self.movie
    }

    pub fn savestate(&self) -> &DeSmuMESavestate {
        &self.savestate
    }

    pub fn savestate_mut(&mut self) -> &mut DeSmuMESavestate {
        &mut self.savestate
    }

    /// Set the current firmware language.
    pub fn set_language(&mut self, lang: Language) {
        unsafe { desmume_set_language(lang as u8) }
    }

    /// Open a ROM by file name.
    ///
    /// If `auto_resume` is true, the emulator will automatically begin emulating the game.
    /// Otherwise the emulator is paused and you may call `resume` to unpause it.
    pub fn open(&mut self, file_name: &str, auto_resume: bool) -> Result<(), DeSmuMEError> {
        unsafe {
            if desmume_open(CString::new(file_name)?.as_ptr()) < 0 {
                return Err(DeSmuMEError::FailedOpen);
            }
        }
        if auto_resume {
            self.resume(false);
        }
        Ok(())
    }

    /// Set the type of the SRAM. Please see the DeSmuME documentation for possible values.
    /// 0 is auto-detect and set by default.
    pub fn set_savetype(&mut self, value: u8) {
        // TODO: Enum?
        unsafe { desmume_set_savetype(value as c_int) }
    }

    /// Pause the emulator.
    pub fn pause(&mut self) {
        unsafe { desmume_pause() }
    }

    /// Resume / unpause the emulator. This will reset the keypad (release all keys),
    /// except if `keep_keypad` is true.
    pub fn resume(&mut self, keep_keypad: bool) {
        if keep_keypad {
            self.input.keypad_update(0);
        }
        unsafe {
            desmume_resume();
        }
    }

    /// Resets the emulator / restarts the current game.
    pub fn reset(&mut self) {
        unsafe {
            desmume_reset();
        }
    }

    /// Returns `true`, if a game is loaded and the emulator is running (not paused).
    pub fn is_running(&self) -> bool {
        unsafe { desmume_running() > 0 }
    }

    /// Tell the emulator to skip the next frame.
    pub fn skip_next_frame(&mut self) {
        unsafe { desmume_skip_next_frame() }
    }

    /// Cycle one game cycle / frame.
    pub fn cycle(&mut self) {
        unsafe { desmume_cycle(self.input.joystick_was_init as c_bool) }
    }

    /// Returns `true`, if OpenGL is available for rendering.
    pub fn has_opengl(&self) -> bool {
        unsafe { desmume_has_opengl() > 0 }
    }

    /// Create an SDL window for drawing the
    /// emulator in and processing inputs.
    ///
    /// If a window was already created, it is returned.
    ///
    /// Otherwise a new window is created and it is given the properties based on the parameters:
    /// - `auto_pause`: Whether or not "tabbing out" of the window pauses the game.
    /// - `use_opengl_if_possible`: Whether or not to use OpenGL for rendering, if available.
    pub fn create_sdl_window(
        &mut self,
        auto_pause: bool,
        use_opengl_if_possible: bool,
    ) -> Result<&mut DeSmuMESdlWindow, DeSmuMEError> {
        if self.window.is_none() {
            self.window = Some(DeSmuMESdlWindow::new(auto_pause, use_opengl_if_possible)?);
        }
        Ok(self.window.as_mut().unwrap())
    }

    /// Return the display buffer in the internal format.
    /// You probably want to use display_buffer_as_rgbx instead.
    pub fn display_buffer(&self) -> &[u16] {
        unsafe { &*slice_from_raw_parts(desmume_draw_raw(), ffi::FRAMEBUFFER_SIZE) }
    }

    /// Fill in the display buffer as RGBX color values,
    /// see the screen size constants for how many pixels make up lines.
    ///
    /// # Safety
    /// The slice passed in must hold `SCREEN_WIDTH * SCREEN_HEIGHT_BOTH * 4` bytes.
    pub unsafe fn display_buffer_as_rgbx_into(&self, buffer: &mut [u8]) {
        desmume_draw_raw_as_rgbx(buffer.as_mut_ptr());
    }

    /// Return the display buffer as RGBX color values,
    /// see the screen size constants for how many pixels make up lines.
    pub fn display_buffer_as_rgbx(&self) -> Vec<u8> {
        let mut buff = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT_BOTH * 4];
        unsafe { self.display_buffer_as_rgbx_into(&mut buff) }
        buff
    }

    /// Get the current SDL tick number.
    pub fn get_ticks(&self) -> u32 {
        unsafe { desmume_sdl_get_ticks() as u32 }
    }

    /// Get the current value (between 0 - 100).
    pub fn volume_get(&self) -> u8 {
        unsafe { desmume_volume_get() as u8 }
    }

    /// Set the current value (to a value between 0 - 100).
    pub fn volume_set(&self, volume: u8) {
        unsafe { desmume_volume_set(volume as c_int) }
    }

    /// Get the current display status of the specified layer on the main GPU.
    pub fn gpu_get_layer_main_enable_state(&self, layer_index: u8) -> bool {
        unsafe { desmume_gpu_get_layer_main_enable_state(layer_index as c_int) > 0 }
    }

    /// Get the current display status of the specified layer on the sub GPU.
    pub fn gpu_get_layer_sub_enable_state(&self, layer_index: u8) -> bool {
        unsafe { desmume_gpu_get_layer_sub_enable_state(layer_index as c_int) > 0 }
    }

    /// Get the current display status of the specified layer on the main GPU.
    pub fn gpu_set_layer_main_enable_state(&self, layer_index: u8, state: bool) {
        unsafe { desmume_gpu_set_layer_main_enable_state(layer_index as c_int, state as c_bool) }
    }

    /// Get the current display status of the specified layer on the main GPU.
    pub fn gpu_set_layer_sub_enable_state(&self, layer_index: u8, state: bool) {
        unsafe { desmume_gpu_set_layer_sub_enable_state(layer_index as c_int, state as c_bool) }
    }
}

impl Drop for DeSmuME {
    fn drop(&mut self) {
        unsafe {
            // Freeing DeSmuME will prevent it from ever be used again. So at the moment we keep
            // it around, in case it is needed again later. You can use free_desmume() to manually
            // free it.
            //desmume_free();
            ALREADY_INITIALIZED = false;
        }
    }
}

/// This will free DeSmuME preventing it from ever be used again within this process.
/// This function could be removed in a future update and DeSmuME freeing could be automated,
/// but only when freeing DeSmuME doesn't prevent it from being initialized again.
///
/// # Safety
/// This is only safe to call ONCE after an instance of DeSmuME was created and is only safe to call
/// if no instance exists anymore. After calling this function no instance of DeSmuME can be created
/// anymore (doing so leads to undefined behaviour).
pub unsafe fn free_desmume() {
    desmume_free()
}
