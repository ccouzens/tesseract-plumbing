extern crate tesseract_sys;
extern crate thiserror;

use self::tesseract_sys::{
    TessBaseAPICreate, TessBaseAPIDelete, TessBaseAPIGetAltoText, TessBaseAPIGetHOCRText,
    TessBaseAPIGetInputImage, TessBaseAPIGetSourceYResolution, TessBaseAPIGetTsvText,
    TessBaseAPIGetUTF8Text, TessBaseAPIInit3, TessBaseAPIRecognize, TessBaseAPISetImage,
    TessBaseAPISetImage2, TessBaseAPISetRectangle, TessBaseAPISetSourceResolution,
    TessBaseAPISetVariable,
};
use self::thiserror::Error;
use crate::Text;
use leptonica_plumbing::Pix;
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;

/// Wrapper around [`tesseract::TessBaseAPI`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html)
#[derive(Debug)]
pub struct TessBaseAPI(*mut tesseract_sys::TessBaseAPI);

impl Drop for TessBaseAPI {
    fn drop(&mut self) {
        unsafe { TessBaseAPIDelete(self.0) }
    }
}

impl Default for TessBaseAPI {
    fn default() -> Self {
        Self::create()
    }
}

#[derive(Debug, Error)]
#[error("TessBaseApi failed to initialize")]
pub struct TessBaseAPIInitError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to set variable")]
pub struct TessBaseAPISetVariableError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to recognize")]
pub struct TessBaseAPIRecogniseError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_hocr_text returned null")]
pub struct TessBaseAPIGetHOCRTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_utf8_text returned null")]
pub struct TessBaseAPIGetUTF8TextError();

#[derive(Debug, Error, PartialEq)]
pub enum TessBaseAPISetImageSafetyError {
    #[error("Image dimensions exceed computer memory")]
    DimensionsExceedMemory(),
    #[error("Image dimensions exceed image size")]
    DimensionsExceedImageSize(),
    #[error("Image width exceeds bytes per line")]
    ImageWidthExceedsBytesPerLine(),
}

#[derive(Debug, Error)]
#[error("TessBaseApi get_alto_text returned null")]
pub struct TessBaseAPIGetAltoTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_tsv_text returned null")]
pub struct TessBaseAPIGetTsvTextError();

impl TessBaseAPI {
    pub fn create() -> TessBaseAPI {
        TessBaseAPI(unsafe { TessBaseAPICreate() })
    }

    /// Wrapper for [`Init-2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a965ef2ff51c440756519a3d6f755f34f)
    ///
    /// Start tesseract
    pub fn init_2(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
    ) -> Result<(), TessBaseAPIInitError> {
        let ret = unsafe {
            TessBaseAPIInit3(
                self.0,
                datapath.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseAPIInitError {})
        }
    }

    /// Wrapper for [`SetImage-2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a0c4c7f05fd58b3665b123232a05545ad)
    pub fn set_image_2(&mut self, pix: &Pix) {
        unsafe {
            TessBaseAPISetImage2(self.0, *pix.as_ref());
        }
    }

    /// Wrapper for [`SetImage-1`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#aa463622111f3b11d8fca5863709cc699)
    pub fn set_image(
        &mut self,
        image_data: &[u8],
        width: c_int,
        height: c_int,
        bytes_per_pixel: c_int,
        bytes_per_line: c_int,
    ) -> Result<(), TessBaseAPISetImageSafetyError> {
        let claimed_image_size: usize = (height * bytes_per_line)
            .try_into()
            .map_err(|_| TessBaseAPISetImageSafetyError::DimensionsExceedMemory())?;
        if claimed_image_size > image_data.len() {
            return Err(TessBaseAPISetImageSafetyError::DimensionsExceedImageSize());
        }
        match bytes_per_pixel {
            0 => {
                if width > bytes_per_line * 8 {
                    return Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
            _ => {
                if width * bytes_per_pixel > bytes_per_line {
                    return Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
        }
        unsafe {
            TessBaseAPISetImage(
                self.0,
                image_data.as_ptr(),
                width,
                height,
                bytes_per_pixel,
                bytes_per_line,
            );
        };
        Ok(())
    }
    /// Wrapper for [`SetSourceResolution`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a4ded6137507a4e8eb6ed4bea0b9648f4)
    ///
    /// Set the resolution of the source image in pixels per inch so font size information can be calculated in results. Call this after SetImage().
    pub fn set_source_resolution(&mut self, ppi: c_int) {
        unsafe {
            TessBaseAPISetSourceResolution(self.0, ppi);
        }
    }

    /// Wrapper for [`SetVariable`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a2e09259c558c6d8e0f7e523cbaf5adf5)
    ///
    /// Warning! Everytime you use a `name` that isn't recognized by Tesseract, a few bytes of memory are leaked.
    pub fn set_variable(
        &mut self,
        name: &CStr,
        value: &CStr,
    ) -> Result<(), TessBaseAPISetVariableError> {
        let ret = unsafe { TessBaseAPISetVariable(self.0, name.as_ptr(), value.as_ptr()) };
        match ret {
            1 => Ok(()),
            _ => Err(TessBaseAPISetVariableError {}),
        }
    }
    /// Wrapper for [`Recognize`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a0e4065c20b142d69a2324ee0c74ae0b0)
    ///
    /// Recognize the image. Returns `Ok(())` on success and `Err(())` otherwise.
    /// It is currently unclear to me what would make it error.
    ///
    /// It could take a progress argument (`monitor`). If there is appetite for this, let me know and I could try and implement it.
    pub fn recognize(&mut self) -> Result<(), TessBaseAPIRecogniseError> {
        let ret = unsafe { TessBaseAPIRecognize(self.0, ptr::null_mut()) };
        match ret {
            0 => Ok(()),
            _ => Err(TessBaseAPIRecogniseError {}),
        }
    }
    /// Wrapper for [`GetUTF8Text`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a115ef656f83352ba608b4f0bf9cfa2c4)
    ///
    /// Get the text out of an image.
    ///
    /// Can return an error (null pointer), but it is not clear to me what would cause this.
    ///
    /// This will implicitly call `recognize` if required.
    pub fn get_utf8_text(&mut self) -> Result<Text, TessBaseAPIGetUTF8TextError> {
        let ptr = unsafe { TessBaseAPIGetUTF8Text(self.0) };
        if ptr.is_null() {
            Err(TessBaseAPIGetUTF8TextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`GetUTF8Text`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a655f906bbf64dcd6f33ce633ecce997d)
    ///
    /// Get the text out of an image.
    ///
    /// Can return an error (null pointer), but it is not clear to me what would cause this.
    ///
    /// This will implicitly call `recognize` if required.
    pub fn get_hocr_text(&mut self, page: c_int) -> Result<Text, TessBaseAPIGetHOCRTextError> {
        let ptr = unsafe { TessBaseAPIGetHOCRText(self.0, page) };
        if ptr.is_null() {
            Err(TessBaseAPIGetHOCRTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetInputImage`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#ad2c023e46bf634305b3ae8cd0c091a65)
    pub fn get_input_image(&self) -> Option<leptonica_plumbing::BorrowedPix> {
        let ptr = unsafe { TessBaseAPIGetInputImage(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { leptonica_plumbing::BorrowedPix::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetSourceYResolution`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a2996381d53d41e486b7fb77e071df8ad)
    pub fn get_source_y_resolution(&self) -> c_int {
        unsafe { TessBaseAPIGetSourceYResolution(self.0) }
    }

    /// Wrapper for [`TessBaseAPISetRectangle`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#aeda62b939bbf06f79ec628932a4fed77)
    ///
    /// Restrict recognition to a sub-rectangle of the image. Call after SetImage. Each SetRectangle clears the recogntion results so multiple rectangles can be recognized with the same image.
    pub fn set_rectangle(&mut self, left: c_int, top: c_int, width: c_int, height: c_int) {
        unsafe { TessBaseAPISetRectangle(self.0, left, top, width, height) }
    }

    /// Wrapper for [`TessBaseAPIGetAltoText`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a37b6dad313c531901dcca9de5ccb37b3)
    ///
    /// Make an XML-formatted string with Alto markup from the internal data structures.
    pub fn get_alto_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseAPIGetAltoTextError> {
        let ptr = unsafe { TessBaseAPIGetAltoText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseAPIGetAltoTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetTsvText`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#ac53c7f530eca78b348d84ef4348103f5)
    ///
    /// Make a TSV-formatted string from the internal data structures. page_number is 0-based but will appear in the output as 1-based.
    pub fn get_tsv_text(&mut self, page_number: c_int) -> Result<Text, TessBaseAPIGetTsvTextError> {
        let ptr = unsafe { TessBaseAPIGetTsvText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseAPIGetTsvTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }
}

#[test]
fn set_image_1_safety_test() {
    use image::GenericImageView;
    let mut tess = TessBaseAPI::create();
    tess.init_2(None, None).unwrap();
    let img = image::open("image.png").unwrap();
    assert_eq!(
        tess.set_image(
            img.as_rgba8().unwrap(),
            img.width().try_into().unwrap(),
            img.height().try_into().unwrap(),
            4,
            (img.width() * 4).try_into().unwrap()
        ),
        Ok(())
    );
    assert_eq!(tess.set_image(&[0, 0, 0, 0], 2, 2, 1, 2), Ok(()));
    assert_eq!(
        tess.set_image(&[0, 0, 0], 2, 2, 1, 2),
        Err(TessBaseAPISetImageSafetyError::DimensionsExceedImageSize())
    );
    assert_eq!(
        tess.set_image(&[0, 0, 0, 0], 2, 2, 1, 1),
        Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
    assert_eq!(tess.set_image(&[0, 0, 0, 0], 16, 2, 0, 2), Ok(()));
    assert_eq!(
        tess.set_image(&[0, 0, 0, 0], 17, 2, 0, 2),
        Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
}

#[test]
fn set_variable_error_test() -> Result<(), Box<dyn std::error::Error>> {
    let fail = std::ffi::CString::new("fail")?;
    let mut tess = TessBaseAPI::create();
    tess.init_2(None, None)?;
    assert!(tess.set_variable(&fail, &fail).is_ok());
    Ok(())
}
