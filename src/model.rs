//! TensorFlow Lite [`Model`] loader.
//!
//! # Examples
//!
//! ```
//! use tflitec::model::Model;
//! use std::path::MAIN_SEPARATOR;
//!
//! let model = Model::new(&format!("tests{}add.bin", MAIN_SEPARATOR))?;
//! # Ok::<(), tflitec::Error>(())
//! ```
use tflitec_sys::bindings::{
    TfLiteModel, TfLiteModelCreate, TfLiteModelCreateFromFile, TfLiteModelDelete,
};
use crate::{Error, ErrorKind, Result};
use std::ffi::CString;
use std::fmt::{Debug, Formatter};

/// A TensorFlow Lite model used by the [`Interpreter`][crate::interpreter::Interpreter] to perform inference.
pub struct Model {
    /// The underlying [`TfLiteModel`] C pointer.
    pub(crate) model_ptr: *mut TfLiteModel,
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model").finish()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    /// Creates a new instance with the given `filepath`.
    ///
    /// # Arguments
    ///
    /// * `filepath`: The local file path to a TensorFlow Lite model.
    ///
    /// # Errors
    ///
    /// Returns error if TensorFlow Lite C fails to read model from file.
    pub fn new(filepath: &str) -> Result<Model> {
        let model_ptr = unsafe {
            let path = CString::new(filepath).unwrap();
            TfLiteModelCreateFromFile(path.as_ptr())
        };
        if model_ptr.is_null() {
            Err(Error::new(ErrorKind::FailedToLoadModel))
        } else {
            Ok(Model { model_ptr })
        }
    }

    /// Creates a new instance with the given bytes `buffer`.
    /// # Arguments
    ///
    /// * `buffer`: The bytes buffer of the TensorFlow lite model.
    ///
    /// # Errors
    ///
    /// Returns error if TensorFlow Lite C fails to read model from file.
    pub fn with_buffer(buffer: &[u8]) -> Result<Model> {
        let model_ptr = unsafe {
            let buffer_ptr = buffer.as_ptr() as *const ::std::os::raw::c_void;
            let size = buffer.len();
            TfLiteModelCreate(buffer_ptr, size as u64)
        };
        if model_ptr.is_null() {
            Err(Error::new(ErrorKind::FailedToLoadModel))
        } else {
            Ok(Model { model_ptr })
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { TfLiteModelDelete(self.model_ptr) }
    }
}
