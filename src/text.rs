extern crate tesseract_sys;

use self::tesseract_sys::TessDeleteText;
use std::convert::AsRef;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_char;

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
    pub unsafe fn new(raw: *mut c_char) -> Self {
        Self(raw)
    }
}

impl AsRef<CStr> for Text {
    fn as_ref(&self) -> &CStr {
        if self.0.is_null() {
            // TODO: This is a dumb hack.
            // Tesseract may choose to return a null pointer for no text for some invalid states.
            // This breaks one of the invariants on `from_ptr` and is invalid,
            // so we return a static string with no characters.
            return unsafe { CStr::from_ptr("\0".as_ptr().cast()) };
        }
        unsafe { CStr::from_ptr(self.0) }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.as_ref()
                .to_str()
                .expect("Tesseract returned invalid UTF-8 str")
        )
    }
}
