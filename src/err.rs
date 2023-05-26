use std::ffi::NulError;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DeSmuMEError {
    #[error("The emulator is already instantiated.")]
    AlreadyInit,
    #[error("Failed to initialize the emulator.")]
    FailedInit,
    #[error("Failed to open the ROM file.")]
    FailedOpen,
    #[error("Failed to initialize the SDL window.")]
    FailedInitWindow,
    #[error("Failed to load savestate.")]
    LoadSavestateFailed,
    #[error("Failed to save savestate.")]
    SaveSavestateFailed,
    #[error("{0}")]
    MoviePlayError(String),
    #[error("No movie is active.")]
    NoMovieActive,
    #[error("Joystick not initialized.")]
    JoystickNotInit,
    #[error("Failed to initialize the joystick controls.")]
    FailedInitJoystick,
    #[error("Null error while trying to convert string.")]
    NulError(#[source] NulError),
}

impl From<NulError> for DeSmuMEError {
    fn from(e: NulError) -> Self {
        Self::NulError(e)
    }
}
