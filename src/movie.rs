pub use crate::ffi::SimpleDate;
use crate::ffi::*;
use crate::DeSmuMEError;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

/// Record and play movies.
pub struct DeSmuMEMovie(pub(crate) PhantomData<()>);

impl DeSmuMEMovie {
    /// Load a movie file from a file and play it back.
    pub fn play(&mut self, file_name: &str) -> Result<(), DeSmuMEError> {
        unsafe {
            let err_raw = desmume_movie_play(CString::new(file_name)?.as_ptr());
            if !err_raw.is_null() {
                let err = CStr::from_ptr(err_raw).to_str().unwrap();
                if !err.is_empty() {
                    return Err(DeSmuMEError::MoviePlayError(err.to_owned()));
                }
            }
        }
        Ok(())
    }

    /// Record a movie.
    pub fn record(&mut self, record_data: RecordData) -> Result<(), DeSmuMEError> {
        unsafe {
            match record_data.rtc_date {
                None => desmume_movie_record(
                    CString::new(record_data.file_name)?.as_ptr(),
                    CString::new(record_data.author_name)?.as_ptr(),
                    record_data.start_from,
                    CString::new(record_data.sram_save)?.as_ptr(),
                ),
                Some(rtc_data) => desmume_movie_record_from_date(
                    CString::new(record_data.file_name)?.as_ptr(),
                    CString::new(record_data.author_name)?.as_ptr(),
                    record_data.start_from,
                    CString::new(record_data.sram_save)?.as_ptr(),
                    rtc_data,
                ),
            }
        }
        Ok(())
    }

    /// Stops the current movie playback.
    pub fn stop(&mut self) {
        unsafe { desmume_movie_stop() }
    }

    pub fn is_active(&self) -> bool {
        unsafe { desmume_movie_is_active() > 0 }
    }

    pub fn is_recording(&self) -> bool {
        unsafe { desmume_movie_is_recording() > 0 }
    }

    pub fn is_playing(&self) -> bool {
        unsafe { desmume_movie_is_playing() > 0 }
    }

    pub fn is_finished(&self) -> bool {
        unsafe { desmume_movie_is_finished() > 0 }
    }

    /// Returns an error if no movie is active.
    pub fn get_length(&self) -> Result<u32, DeSmuMEError> {
        if self.is_active() {
            Ok(unsafe { desmume_movie_get_length() as u32 })
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }

    /// Returns an error if no movie is active.
    pub fn get_name(&self) -> Result<String, DeSmuMEError> {
        if self.is_active() {
            Ok(unsafe {
                CStr::from_ptr(desmume_movie_get_name())
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }

    /// Returns an error if no movie is active.
    pub fn get_rerecord_count(&self) -> Result<isize, DeSmuMEError> {
        if self.is_active() {
            Ok(unsafe { desmume_movie_get_rerecord_count() as isize })
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }

    /// Returns an error if no movie is active.
    pub fn set_rerecord_count(&self, count: isize) -> Result<(), DeSmuMEError> {
        if self.is_active() {
            unsafe { desmume_movie_set_rerecord_count(count as c_int) };
            Ok(())
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }

    /// Returns an error if no movie is active.
    pub fn get_readonly(&self) -> Result<bool, DeSmuMEError> {
        if self.is_active() {
            Ok(unsafe { desmume_movie_get_readonly() > 0 })
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }

    /// Returns an error if no movie is active.
    pub fn set_readonly(&self, state: bool) -> Result<(), DeSmuMEError> {
        if self.is_active() {
            unsafe { desmume_movie_set_readonly(state as c_bool) };
            Ok(())
        } else {
            Err(DeSmuMEError::NoMovieActive)
        }
    }
}

/// Metadata for a recording
pub struct RecordData<'a> {
    /// The name of the file to save to.
    file_name: &'a str,
    /// Name of the author of the movie.
    author_name: &'a str,
    /// Where to start the recording from.
    start_from: StartFrom,
    /// Filename of the SRAM save to use when starting from SRAM.
    sram_save: &'a str,
    /// Date to set the real-time-clock to, defaults to now.
    rtc_date: Option<SimpleDate>,
}

impl<'a> RecordData<'a> {
    /// Start from reset.
    pub fn new_from_blank(file_name: &'a str, author_name: &'a str) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::Blank,
            sram_save: "",
            rtc_date: None,
        }
    }

    /// Start from the given SRAM save name.
    pub fn new_from_sram(
        file_name: &'a str,
        author_name: &'a str,
        sram_save_name: &'a str,
    ) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::SRAM,
            sram_save: sram_save_name,
            rtc_date: None,
        }
    }

    /// Start from the first savestate.
    pub fn new_from_savestate(file_name: &'a str, author_name: &'a str) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::Savestate,
            sram_save: "",
            rtc_date: None,
        }
    }

    /// Start from reset, set the RTC before starting.
    pub fn new_from_blank_with_date(
        file_name: &'a str,
        author_name: &'a str,
        rtc_date: SimpleDate,
    ) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::Blank,
            sram_save: "",
            rtc_date: Some(rtc_date),
        }
    }

    /// Start from the given SRAM save name, set the RTC before starting.
    pub fn new_from_sram_with_date(
        file_name: &'a str,
        author_name: &'a str,
        sram_save_name: &'a str,
        rtc_date: SimpleDate,
    ) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::SRAM,
            sram_save: sram_save_name,
            rtc_date: Some(rtc_date),
        }
    }

    /// Start from the first savestate,  set the RTC before starting.
    pub fn new_from_savestate_with_date(
        file_name: &'a str,
        author_name: &'a str,
        rtc_date: SimpleDate,
    ) -> Self {
        RecordData {
            file_name,
            author_name,
            start_from: StartFrom::Savestate,
            sram_save: "",
            rtc_date: Some(rtc_date),
        }
    }
}
