mod tess_base_api;
mod text;

pub use leptonica_plumbing;
pub use leptonica_plumbing::leptonica_sys;
pub use tess_base_api::{
    TessBaseApi, TessBaseApiGetAltoTextError, TessBaseApiGetHocrTextError,
    TessBaseApiGetLstmBoxTextError, TessBaseApiGetTsvTextError, TessBaseApiGetUtf8TextError,
    TessBaseApiGetWordStrBoxTextError, TessBaseApiInitError, TessBaseApiRecogniseError,
    TessBaseApiSetImageSafetyError, TessBaseApiSetVariableError,
};
pub use tesseract_sys;
pub use text::Text;
