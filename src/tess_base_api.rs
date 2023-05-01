extern crate tesseract_sys;
extern crate thiserror;

use self::tesseract_sys::{
    TessBaseAPIAllWordConfidences, TessBaseAPICreate, TessBaseAPIDelete, TessBaseAPIGetAltoText,
    TessBaseAPIGetComponentImages, TessBaseAPIGetHOCRText, TessBaseAPIGetInputImage,
    TessBaseAPIGetLSTMBoxText, TessBaseAPIGetSourceYResolution, TessBaseAPIGetTsvText,
    TessBaseAPIGetUTF8Text, TessBaseAPIGetWordStrBoxText, TessBaseAPIInit2, TessBaseAPIInit3,
    TessBaseAPIInit5, TessBaseAPIMeanTextConf, TessBaseAPIRecognize, TessBaseAPISetImage,
    TessBaseAPISetImage2, TessBaseAPISetPageSegMode, TessBaseAPISetRectangle,
    TessBaseAPISetSourceResolution, TessBaseAPISetVariable, TessDeleteIntArray, TessOcrEngineMode,
    TessPageIteratorLevel, TessPageSegMode,
};
use self::thiserror::Error;
use crate::Text;
use leptonica_plumbing::Pix;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use std::ptr;
use std::slice;

/// Wrapper around [`tesseract::TessBaseAPI`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html)
#[derive(Debug)]
pub struct TessBaseApi(*mut tesseract_sys::TessBaseAPI);

unsafe impl Send for TessBaseApi {}

impl Drop for TessBaseApi {
    fn drop(&mut self) {
        unsafe { TessBaseAPIDelete(self.0) }
    }
}

impl Default for TessBaseApi {
    fn default() -> Self {
        Self::create()
    }
}

#[derive(Debug, Error)]
#[error("TessBaseApi failed to initialize")]
pub struct TessBaseApiInitError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to set variable")]
pub struct TessBaseApiSetVariableError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to recognize")]
pub struct TessBaseApiRecogniseError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_hocr_text returned null")]
pub struct TessBaseApiGetHocrTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_utf8_text returned null")]
pub struct TessBaseApiGetUtf8TextError();

#[derive(Debug, Error, PartialEq)]
pub enum TessBaseApiSetImageSafetyError {
    #[error("Image dimensions exceed computer memory")]
    DimensionsExceedMemory(),
    #[error("Image dimensions exceed image size")]
    DimensionsExceedImageSize(),
    #[error("Image width exceeds bytes per line")]
    ImageWidthExceedsBytesPerLine(),
}

#[derive(Debug, Error)]
#[error("TessBaseApi get_alto_text returned null")]
pub struct TessBaseApiGetAltoTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_tsv_text returned null")]
pub struct TessBaseApiGetTsvTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_lstm_box_text returned null")]
pub struct TessBaseApiGetLstmBoxTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_word_str_text returned null")]
pub struct TessBaseApiGetWordStrBoxTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_component_images returned null")]
pub struct TessBaseApiGetComponentImagesError();

#[derive(Debug, Error)]
#[error("TessBaseApi all_word_confidences returned null")]
pub struct TessBaseApiAllWordConfidencesError();

pub struct AllWordConfidences(*mut c_int, usize);

impl AllWordConfidences {
    pub fn as_slice(&self) -> &[c_int] {
        self
    }

    pub fn as_slice_mut(&mut self) -> &mut [c_int] {
        self
    }
}

impl Deref for AllWordConfidences {
    type Target = [c_int];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.0, self.1) }
    }
}

impl DerefMut for AllWordConfidences {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.0, self.1) }
    }
}

impl Drop for AllWordConfidences {
    fn drop(&mut self) {
        unsafe {
            TessDeleteIntArray(self.0);
        }
    }
}

impl TessBaseApi {
    pub fn create() -> Self {
        Self(unsafe { TessBaseAPICreate() })
    }

    /// Wrapper for [`Init-1`]https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a2be07b4c9449b8cfc43e9c26ee623050
    pub fn init_1(
        &mut self,
        data: &[u8],
        language: Option<&CStr>,
        oem: TessOcrEngineMode,
    ) -> Result<(), TessBaseApiInitError> {
        let ret = unsafe {
            TessBaseAPIInit5(
                self.0,
                data.as_ptr().cast(),
                data.len() as c_int,
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                oem,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                0,
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseApiInitError {})
        }
    }

    /// Wrapper for [`Init-2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a965ef2ff51c440756519a3d6f755f34f)
    ///
    /// Start tesseract
    pub fn init_2(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
    ) -> Result<(), TessBaseApiInitError> {
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
            Err(TessBaseApiInitError {})
        }
    }

    /// Wrapper for [`Init-4`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a6d0956a66158ead4e3a86c7f50dad56e)
    pub fn init_4(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
        oem: TessOcrEngineMode,
    ) -> Result<(), TessBaseApiInitError> {
        let ret = unsafe {
            TessBaseAPIInit2(
                self.0,
                datapath.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                oem,
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseApiInitError {})
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
    ) -> Result<(), TessBaseApiSetImageSafetyError> {
        let claimed_image_size: usize = (height * bytes_per_line)
            .try_into()
            .map_err(|_| TessBaseApiSetImageSafetyError::DimensionsExceedMemory())?;
        if claimed_image_size > image_data.len() {
            return Err(TessBaseApiSetImageSafetyError::DimensionsExceedImageSize());
        }
        match bytes_per_pixel {
            0 => {
                if width > bytes_per_line * 8 {
                    return Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
            _ => {
                if width * bytes_per_pixel > bytes_per_line {
                    return Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine());
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
    ) -> Result<(), TessBaseApiSetVariableError> {
        let ret = unsafe { TessBaseAPISetVariable(self.0, name.as_ptr(), value.as_ptr()) };
        match ret {
            1 => Ok(()),
            _ => Err(TessBaseApiSetVariableError {}),
        }
    }

    /// Wrapper for [`SetPageSegMode`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a15a7a9c1afbba3078a55b4566de891ab)
    ///
    /// Set the current page segmentation mode
    pub fn set_page_seg_mode(&mut self, mode: TessPageSegMode) {
        unsafe { TessBaseAPISetPageSegMode(self.0, mode) };
    }

    /// Wrapper for [`Recognize`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a0e4065c20b142d69a2324ee0c74ae0b0)
    ///
    /// Recognize the image. Returns `Ok(())` on success and `Err(())` otherwise.
    /// It is currently unclear to me what would make it error.
    ///
    /// It could take a progress argument (`monitor`). If there is appetite for this, let me know and I could try and implement it.
    pub fn recognize(&mut self) -> Result<(), TessBaseApiRecogniseError> {
        let ret = unsafe { TessBaseAPIRecognize(self.0, ptr::null_mut()) };
        match ret {
            0 => Ok(()),
            _ => Err(TessBaseApiRecogniseError {}),
        }
    }
    /// Wrapper for [`GetUTF8Text`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a115ef656f83352ba608b4f0bf9cfa2c4)
    ///
    /// Get the text out of an image.
    ///
    /// Can return an error (null pointer), but it is not clear to me what would cause this.
    ///
    /// This will implicitly call `recognize` if required.
    pub fn get_utf8_text(&mut self) -> Result<Text, TessBaseApiGetUtf8TextError> {
        let ptr = unsafe { TessBaseAPIGetUTF8Text(self.0) };
        if ptr.is_null() {
            Err(TessBaseApiGetUtf8TextError {})
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
    pub fn get_hocr_text(&mut self, page: c_int) -> Result<Text, TessBaseApiGetHocrTextError> {
        let ptr = unsafe { TessBaseAPIGetHOCRText(self.0, page) };
        if ptr.is_null() {
            Err(TessBaseApiGetHocrTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetInputImage`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#ad2c023e46bf634305b3ae8cd0c091a65)
    pub fn get_input_image(
        &self,
    ) -> Option<leptonica_plumbing::memory::BorrowedFrom<leptonica_plumbing::Pix>> {
        let ptr = unsafe { TessBaseAPIGetInputImage(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                leptonica_plumbing::memory::BorrowedFrom::new(
                    leptonica_plumbing::Pix::new_from_pointer(ptr),
                )
            })
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
    ) -> Result<Text, TessBaseApiGetAltoTextError> {
        let ptr = unsafe { TessBaseAPIGetAltoText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetAltoTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetTsvText`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#ac53c7f530eca78b348d84ef4348103f5)
    ///
    /// Make a TSV-formatted string from the internal data structures. page_number is 0-based but will appear in the output as 1-based.
    pub fn get_tsv_text(&mut self, page_number: c_int) -> Result<Text, TessBaseApiGetTsvTextError> {
        let ptr = unsafe { TessBaseAPIGetTsvText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetTsvTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetLSTMBoxText`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a60205153043d51a977f1f4fb1923da18)
    ///
    /// Make a box file for LSTM training from the internal data structures. Constructs coordinates in the original image - not just the rectangle. page_number is a 0-based page index that will appear in the box file.
    pub fn get_lstm_box_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseApiGetLstmBoxTextError> {
        let ptr = unsafe { TessBaseAPIGetLSTMBoxText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetLstmBoxTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIGetWordStrBoxText`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#ab9938845c9b52434ee32a2225aad81cf)
    ///
    /// The recognized text is returned as a char* which is coded in the same format as a WordStr box file used in training. page_number is a 0-based page index that will appear in the box file. Returned string must be freed with the delete [] operator.
    ///
    /// Create a UTF8 box file with WordStr strings from the internal data structures. page_number is a 0-base page index that will appear in the box file.
    pub fn get_word_str_box_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseApiGetWordStrBoxTextError> {
        let ptr = unsafe { TessBaseAPIGetWordStrBoxText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetWordStrBoxTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    /// Wrapper for [`TessBaseAPIMeanTextConf`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a20c2c34197abc55043cb23be4e332ad0)
    ///
    /// Returns the (average) confidence value between 0 and 100.
    ///
    /// Returns the average word confidence for Tesseract page result.
    pub fn mean_text_conf(&self) -> c_int {
        unsafe { TessBaseAPIMeanTextConf(self.0) }
    }

    /// Wrapper for [`TessBaseAPIAllWordConfidences`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a7e35b5ec11f2e38e00b9fe1126cb5c66)
    ///
    /// Returns a slice of confidences for each word in the result.
    pub fn all_word_confidences(
        &self,
    ) -> Result<AllWordConfidences, TessBaseApiAllWordConfidencesError> {
        let ptr = unsafe { TessBaseAPIAllWordConfidences(self.0) };
        if ptr.is_null() {
            Err(TessBaseApiAllWordConfidencesError {})
        } else {
            let mut end = ptr;
            unsafe {
                while *end != -1 {
                    end = end.add(1);
                }
                let len = end.offset_from(ptr);
                Ok(AllWordConfidences(ptr, len as usize))
            }
        }
    }

    /// Wrapper for [`GetComponentImages 1/2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#ad74ae1266a5299734ec6f5225b6cb5c1)
    ///
    /// Get the given level kind of components (block, textline, word etc.) as a leptonica-style Boxa, Pixa pair, in reading order.
    pub fn get_component_images_1(
        &self,
        level: TessPageIteratorLevel,
        text_only: c_int,
    ) -> Result<
        leptonica_plumbing::memory::RefCountedExclusive<leptonica_plumbing::Boxa>,
        TessBaseApiGetComponentImagesError,
    > {
        let ptr = unsafe {
            TessBaseAPIGetComponentImages(
                self.0,
                level,
                text_only,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if ptr.is_null() {
            Err(TessBaseApiGetComponentImagesError {})
        } else {
            Ok(unsafe {
                leptonica_plumbing::memory::RefCountedExclusive::new(
                    leptonica_plumbing::Boxa::new_from_pointer(ptr),
                )
            })
        }
    }
}

#[test]
fn set_image_1_safety_test() {
    use image::GenericImageView;
    let mut tess = TessBaseApi::create();
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
        Err(TessBaseApiSetImageSafetyError::DimensionsExceedImageSize())
    );
    assert_eq!(
        tess.set_image(&[0, 0, 0, 0], 2, 2, 1, 1),
        Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
    assert_eq!(tess.set_image(&[0, 0, 0, 0], 16, 2, 0, 2), Ok(()));
    assert_eq!(
        tess.set_image(&[0, 0, 0, 0], 17, 2, 0, 2),
        Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
}

#[test]
fn set_variable_error_test() -> Result<(), Box<dyn std::error::Error>> {
    let fail = std::ffi::CString::new("fail")?;
    let mut tess = TessBaseApi::create();
    tess.init_2(None, None)?;
    assert!(tess.set_variable(&fail, &fail).is_err());
    Ok(())
}
