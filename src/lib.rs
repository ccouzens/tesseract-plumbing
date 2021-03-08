mod tess_base_api;
mod text;

pub use leptonica_plumbing;
pub use leptonica_plumbing::leptonica_sys;
pub use tess_base_api::{
    TessBaseAPI, TessBaseAPIGetAltoTextError, TessBaseAPIGetHOCRTextError,
    TessBaseAPIGetTsvTextError, TessBaseAPIGetUTF8TextError, TessBaseAPIInitError,
    TessBaseAPIRecogniseError, TessBaseAPISetImageSafetyError, TessBaseAPISetVariableError,
};
pub use tesseract_sys;
pub use text::Text;
