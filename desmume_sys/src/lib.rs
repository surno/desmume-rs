pub use libc::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort};

#[allow(non_camel_case_types)]
pub type c_bool = c_int;

pub type MemoryCbFnc = Option<extern "C" fn(addr: c_uint, size: c_int) -> c_bool>;

#[repr(C)]
pub enum DesmumeAudioCore {
    Dummy = 0,
    SDL = 2,
}

#[repr(C)]
pub enum Desmume3DRenderer {
    Null = 0,
    SoftRasterizer = 1,
    Metal = 2000,
}

#[repr(C)]
pub struct DesmumeInitOptions {
    pub audio_core: c_int,
    pub audio_buffer_size: c_int,
    pub renderer_3d: c_int,
    pub init_sdl_timer: c_int,
}

#[repr(u32)]
#[allow(clippy::upper_case_acronyms)]
pub enum StartFrom {
    Blank = 0,
    SRAM = 1,
    Savestate = 2,
}

pub const GPU_FRAMEBUFFER_NATIVE_WIDTH: usize = 256;
pub const GPU_FRAMEBUFFER_NATIVE_HEIGHT: usize = 192;
// The framebuffer *may* be bigger (more pages etc.) but this is the only relevant parts of it.
// Also yes this constant is defined is a bit weird, but I just took it from the DeSmuME source
// (see _displayInfo.framebufferPageSize = ... in GPU.cpp).
pub const FRAMEBUFFER_SIZE: usize = ((GPU_FRAMEBUFFER_NATIVE_WIDTH
    * GPU_FRAMEBUFFER_NATIVE_HEIGHT)
    + (GPU_FRAMEBUFFER_NATIVE_WIDTH * GPU_FRAMEBUFFER_NATIVE_HEIGHT))
    * 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SimpleDate {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub hour: c_int,
    pub minute: c_int,
    pub second: c_int,
    pub millisecond: c_int,
}

#[link(name = "desmume")]
extern "C" {
    pub fn desmume_init() -> c_int;

    pub fn desmume_free();

    pub fn desmume_init_sdl() -> c_int;

    pub fn desmume_init_with_options(options: *const DesmumeInitOptions) -> c_int;

    pub fn desmume_audio_set_core(core_id: c_int, buffer_size: c_int) -> c_int;

    pub fn desmume_audio_get_core() -> c_int;

    pub fn desmume_init_metal() -> c_int;

    pub fn desmume_metal_bootstrap_init() -> c_int;

    pub fn desmume_has_metal() -> c_bool;

    pub fn desmume_get_jit_enabled() -> c_bool;

    pub fn desmume_set_jit_enabled(enabled: c_bool);

    pub fn desmume_get_jit_max_block_size() -> u32;

    pub fn desmume_set_jit_max_block_size(size: u32);

    pub fn desmume_set_language(language: u8);

    pub fn desmume_open(filename: *const c_char) -> c_int;

    pub fn desmume_set_savetype(type_: c_int);

    pub fn desmume_pause();

    pub fn desmume_resume();

    pub fn desmume_reset();

    pub fn desmume_running() -> c_bool;

    pub fn desmume_skip_next_frame();

    pub fn desmume_cycle(with_joystick: c_bool);

    pub fn desmume_sdl_get_ticks() -> c_int;

    pub fn desmume_has_opengl() -> c_bool;

    pub fn desmume_draw_window_init(auto_pause: c_bool, use_opengl_if_possible: c_bool) -> c_int;

    pub fn desmume_draw_window_input();

    pub fn desmume_draw_window_frame();

    pub fn desmume_draw_window_has_quit() -> c_bool;

    pub fn desmume_draw_window_free();

    pub fn desmume_draw_raw() -> *mut u16;

    pub fn desmume_draw_raw_as_rgbx(buffer: *mut u8);

    pub fn desmume_savestate_clear();

    pub fn desmume_savestate_load(file_name: *const c_char) -> c_bool;

    pub fn desmume_savestate_save(file_name: *const c_char) -> c_bool;

    pub fn desmume_savestate_scan();

    pub fn desmume_savestate_slot_load(index: c_int);

    pub fn desmume_savestate_slot_save(index: c_int);

    pub fn desmume_savestate_slot_exists(index: c_int) -> c_bool;

    pub fn desmume_savestate_slot_date(index: c_int) -> *mut c_char;

    pub fn desmume_gpu_get_layer_main_enable_state(layer_index: c_int) -> c_bool;

    pub fn desmume_gpu_get_layer_sub_enable_state(layer_index: c_int) -> c_bool;

    pub fn desmume_gpu_set_layer_main_enable_state(layer_index: c_int, the_state: c_bool);

    pub fn desmume_gpu_set_layer_sub_enable_state(layer_index: c_int, the_state: c_bool);

    pub fn desmume_volume_get() -> c_int;

    pub fn desmume_volume_set(volume: c_int);

    pub fn desmume_memory_read_byte(address: c_int) -> c_uchar;

    pub fn desmume_memory_read_byte_signed(address: c_int) -> c_schar;

    pub fn desmume_memory_read_short(address: c_int) -> c_ushort;

    pub fn desmume_memory_read_short_signed(address: c_int) -> c_short;

    pub fn desmume_memory_read_long(address: c_int) -> c_ulong;

    pub fn desmume_memory_read_long_signed(address: c_int) -> c_long;

    pub fn desmume_memory_write_byte(address: c_int, value: c_uchar);

    pub fn desmume_memory_write_short(address: c_int, value: c_ushort);

    pub fn desmume_memory_write_long(address: c_int, value: c_ulong);

    pub fn desmume_memory_read_register(register_name: *mut c_char) -> u32;

    pub fn desmume_memory_write_register(register_name: *mut c_char, value: u32);

    pub fn desmume_memory_get_next_instruction() -> u32;

    pub fn desmume_memory_set_next_instruction(value: u32);

    pub fn desmume_memory_register_write(address: c_int, size: c_int, cb: MemoryCbFnc);

    pub fn desmume_memory_register_read(address: c_int, size: c_int, cb: MemoryCbFnc);

    pub fn desmume_memory_register_exec(address: c_int, size: c_int, cb: MemoryCbFnc);

    pub fn desmume_screenshot(screenshot_buffer: *mut c_char);

    pub fn desmume_input_joy_init() -> c_bool;

    pub fn desmume_input_joy_uninit();

    pub fn desmume_input_joy_number_connected() -> u16;

    pub fn desmume_input_joy_get_key(index: c_int) -> u16;

    pub fn desmume_input_joy_get_set_key(index: c_int) -> u16;

    pub fn desmume_input_joy_set_key(index: c_int, joystick_key_index: c_int);

    pub fn desmume_input_keypad_update(keys: u16);

    pub fn desmume_input_keypad_get() -> u16;

    pub fn desmume_input_set_touch_pos(x: u16, y: u16);

    pub fn desmume_input_release_touch();

    pub fn desmume_movie_is_active() -> c_bool;

    pub fn desmume_movie_is_recording() -> c_bool;

    pub fn desmume_movie_is_playing() -> c_bool;

    pub fn desmume_movie_is_finished() -> c_bool;

    pub fn desmume_movie_get_length() -> c_int;

    pub fn desmume_movie_get_name() -> *mut c_char;

    pub fn desmume_movie_get_rerecord_count() -> c_int;

    pub fn desmume_movie_set_rerecord_count(count: c_int);

    pub fn desmume_movie_get_readonly() -> c_bool;

    pub fn desmume_movie_set_readonly(state: c_bool);

    pub fn desmume_movie_play(file_name: *const c_char) -> *const c_char;

    pub fn desmume_movie_record_simple(save_file_name: *const c_char, author_name: *const c_char);

    pub fn desmume_movie_record(
        save_file_name: *const c_char,
        author_name: *const c_char,
        start_from: StartFrom,
        sram_file_name: *const c_char,
    );

    pub fn desmume_movie_record_from_date(
        save_file_name: *const c_char,
        author_name: *const c_char,
        start_from: StartFrom,
        sram_file_name: *const c_char,
        date: SimpleDate,
    );

    pub fn desmume_movie_replay();

    pub fn desmume_movie_stop();
}
