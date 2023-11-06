pub struct ResultIterator(*mut tesseract_sys::TessResultIterator);

impl Drop for ResultIterator {
    fn drop(&mut self) {
        unsafe { tesseract_sys::TessResultIteratorDelete(self.0) }
    }
}

impl AsRef<*mut tesseract_sys::TessResultIterator> for ResultIterator {
    fn as_ref(&self) -> &*mut tesseract_sys::TessResultIterator {
        &self.0
    }
}

impl ResultIterator {
    pub fn new(raw: *mut tesseract_sys::TessResultIterator) -> Self {
        Self(raw)
    }
}
