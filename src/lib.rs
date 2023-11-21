mod result_iterator;
mod tess_base_api;
mod text;

use self::tesseract_sys::TessVersion;
pub use leptonica_plumbing;
pub use leptonica_plumbing::leptonica_sys;
pub use result_iterator::ResultIterator;
use std::ffi::CStr;
pub use tess_base_api::{
    TessBaseApi, TessBaseApiInitError, TessBaseApiRecogniseError, TessBaseApiSetImageSafetyError,
    TessBaseApiSetVariableError,
};
pub use tesseract_sys;
pub use text::{Text, TextNewError};

/// Wrapper for [`Version`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a3785779c909fcdd77e24b340f5913e4b)
///
/// Returns the version identifier as a static string.
pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(TessVersion()) }
}
