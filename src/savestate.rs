use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use crate::DeSmuMEError;
use crate::ffi::*;

/// Load and save savestates. Either slots can be used  (maximum number of slots is in the
/// constant `NB_STATES`), or savestates can be directly loaded from / saved to files.
pub struct DeSmuMESavestate(pub(crate) PhantomData<()>);

impl DeSmuMESavestate {
    /// Scan all savestate slots for if they exist or not.
    /// Required to be called before calling `exists`.
    pub fn scan(&mut self) {
        unsafe { desmume_savestate_scan() }
    }

    /// Returns whether or not a savestate in the specified slot exists.
    ///
    /// # Safety
    /// The caller needs to make sure scan() was called before.
    pub unsafe fn exists(&self, slot_id: u8) -> bool {
        desmume_savestate_slot_exists(slot_id as c_int) > 0
    }

    /// Load the savestate in the specified slot. It needs to exist.
    ///
    /// # Safety
    /// The caller needs to make sure the savestate exists.
    pub unsafe fn load(&mut self, slot_id: u8) {
        desmume_savestate_slot_load(slot_id as c_int)
    }

    /// Save the current game state to the savestate in the specified slot.
    pub fn save(&mut self, slot_id: u8) {
        unsafe { desmume_savestate_slot_save(slot_id as c_int) }
    }

    /// Load a savestate from file.
    pub fn load_file(&mut self, file_name: &str) -> Result<(), DeSmuMEError> {
        unsafe {
            if desmume_savestate_load(CString::new(file_name)?.as_ptr()) <= 0 {
                Err(DeSmuMEError::LoadSavestateFailed)
            } else {
                Ok(())
            }
        }
    }

    /// Save a savestate to file.
    pub fn save_file(&mut self, file_name: &str) -> Result<(), DeSmuMEError> {
        unsafe {
            if desmume_savestate_save(CString::new(file_name)?.as_ptr()) <= 0 {
                Err(DeSmuMEError::SaveSavestateFailed)
            } else {
                Ok(())
            }
        }
    }

    /// Return the date a savestate was saved at as a string.
    /// May panic if the date can not be represented as a Rust string.
    pub fn date(&self, slot_id: u8) -> String {
        unsafe {
            CStr::from_ptr(desmume_savestate_slot_date(slot_id as c_int))
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}
