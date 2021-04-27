extern crate tesseract_sys;

use self::tesseract_sys::TessDeleteText;
use std::convert::AsRef;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Wrapper around Tesseract's returned strings
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
    pub unsafe fn new(raw: *mut c_char) -> Self {
        Self(raw)
    }
}

impl AsRef<CStr> for Text {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
