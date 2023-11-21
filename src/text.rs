extern crate tesseract_sys;
extern crate thiserror;

use self::tesseract_sys::TessDeleteText;
use self::thiserror::Error;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_char;

#[derive(Debug, Error)]
pub enum TextNewError {
    #[error("Attempted to initialize with null pointer")]
    NullPointer(),
}

/// Wrapper around Tesseract's returned strings
#[derive(Debug)]
pub struct Text(*mut c_char);

unsafe impl Send for Text {}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe { TessDeleteText(self.0) }
    }
}

impl Text {
    /// # Safety
    ///
    /// This function should only be called with a valid string pointer from Tesseract.
    /// `TesseractText` will be responsible for freeing it.
    pub unsafe fn new(raw: *mut c_char) -> Result<Self, TextNewError> {
        if raw.is_null() {
            Err(TextNewError::NullPointer())
        } else {
            Ok(Self(raw))
        }
    }

    fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cstr().to_string_lossy())
    }
}
