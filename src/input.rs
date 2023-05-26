use crate::ffi::*;
use crate::DeSmuMEError;

/// Manage input processing for the emulator.
pub struct DeSmuMEInput {
    pub(crate) joystick_was_init: bool,
}

impl DeSmuMEInput {
    /// Initialize the joystick input processing.
    /// Call this to enable automatic joystick input processing.
    pub fn joy_init(&mut self) -> Result<(), DeSmuMEError> {
        unsafe {
            if desmume_input_joy_init() > 0 {
                self.joystick_was_init = true;
                Ok(())
            } else {
                Err(DeSmuMEError::FailedInitJoystick)
            }
        }
    }

    /// De-initialize the joystick input processing.
    pub fn joy_uninit(&mut self) {
        if self.joystick_was_init {
            unsafe { desmume_input_joy_uninit() }
            self.joystick_was_init = false;
        }
    }

    /// Returns the number of connected joysticks. Joysticks must be initialized.
    pub fn joy_number_connected(&self) -> Result<u16, DeSmuMEError> {
        if self.joystick_was_init {
            Ok(unsafe { desmume_input_joy_number_connected() })
        } else {
            Err(DeSmuMEError::JoystickNotInit)
        }
    }

    /// Returns the number of connected joysticks. Joysticks must be initialized.
    pub fn joy_get_key(&self, index: u16) -> Result<u16, DeSmuMEError> {
        if self.joystick_was_init {
            Ok(unsafe { desmume_input_joy_get_key(index as c_int) })
        } else {
            Err(DeSmuMEError::JoystickNotInit)
        }
    }

    /// Pause the thread and wait for the user to press a button.
    /// This button will be assigned to the specified emulator key. Joysticks must be initialized.
    pub fn joy_get_set_key(&mut self, index: u16) -> Result<u16, DeSmuMEError> {
        if self.joystick_was_init {
            Ok(unsafe { desmume_input_joy_get_set_key(index as c_int) })
        } else {
            Err(DeSmuMEError::JoystickNotInit)
        }
    }

    /// Sets the emulator key `index` to the specified joystick key `joystick_key_index`.
    /// Joysticks must be initialized.
    pub fn joy_set_key(&mut self, index: u16, joystick_key_index: i32) -> Result<(), DeSmuMEError> {
        if self.joystick_was_init {
            unsafe { desmume_input_joy_set_key(index as c_int, joystick_key_index as c_int) };
            Ok(())
        } else {
            Err(DeSmuMEError::JoystickNotInit)
        }
    }

    /// Update the keypad (pressed DS buttons) of currently pressed emulator keys.
    /// You should probably use `keypad_add_key` and `keypad_rm_key` instead.
    pub fn keypad_update(&mut self, keys: u16) {
        unsafe { desmume_input_keypad_update(keys) }
    }

    /// Returns the current emulator key keypad (pressed DS buttons).
    pub fn keypad_get(&self) -> u16 {
        unsafe { desmume_input_keypad_get() }
    }

    /// Adds a key to the emulators current keymask (presses it). To be used with `keymask`:
    ///
    /// ```rs
    /// let emu: DeSmuME;
    /// emu.input.keypad_add_key(keymask(Key::A)));
    /// ```
    pub fn keypad_add_key(&mut self, keymask: u16) {
        let old_keypad = self.keypad_get();
        self.keypad_update(add_key(old_keypad, keymask));
    }

    /// Removes a key from the emulators current keymask (releases it).
    /// See ``keypad_add_key`` for a usage example.
    pub fn keypad_rm_key(&mut self, keymask: u16) {
        let old_keypad = self.keypad_get();
        self.keypad_update(rm_key(old_keypad, keymask));
    }

    pub fn touch_set_pos(&mut self, x: u16, y: u16) {
        unsafe { desmume_input_set_touch_pos(x, y) }
    }

    pub fn touch_release(&mut self) {
        unsafe { desmume_input_release_touch() }
    }
}

impl Drop for DeSmuMEInput {
    fn drop(&mut self) {
        self.joy_uninit()
    }
}

/// Joystick input types.
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum Joy {
    Axis = 0,
    Hat = 1,
    Button = 2,
}

/// Jostick hat identifiers.
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum JoyHats {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
}

/// The total number of keys.
pub const NB_KEYS: u8 = 15;
pub const NO_KEY_SET: u16 = 0xFFFF;

/// DS key identifiers.
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum Key {
    None = 0,
    A = 1,
    B = 2,
    Select = 3,
    Start = 4,
    Right = 5,
    Left = 6,
    Up = 7,
    Down = 8,
    R = 9,
    L = 10,
    X = 11,
    Y = 12,
    Debug = 13,
    Boost = 14,
    Lid = 15,
}

#[inline(always)]
fn add_key(keypad: u16, keymask: u16) -> u16 {
    keypad | keymask
}

#[inline(always)]
fn rm_key(keypad: u16, keymask: u16) -> u16 {
    keypad & !keymask
}

pub fn keymask(k: Key) -> u16 {
    // Returns the keymask for key ``k``. ``k`` is a constant of the ``Keys`` class.
    if k == Key::None {
        0
    } else {
        1 << (k as u16 - 1)
    }
}
