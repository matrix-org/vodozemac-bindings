use crate::error::*;
use pyo3::prelude::*;

#[pyclass]
pub struct SessionKey {
    inner: vodozemac::megolm::SessionKey,
}

#[pymethods]
impl SessionKey {
    #[new]
    pub fn from_base64(session_key: &str) -> Result<Self, SessionKeyDecodeError> {
        Ok(Self {
            inner: vodozemac::megolm::SessionKey::from_base64(session_key)?,
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

#[pyclass]
pub struct ExportedSessionKey {
    inner: vodozemac::megolm::ExportedSessionKey,
}

#[pymethods]
impl ExportedSessionKey {
    #[new]
    pub fn from_base64(session_key: &str) -> Result<Self, SessionKeyDecodeError> {
        Ok(Self {
            inner: vodozemac::megolm::ExportedSessionKey::from_base64(session_key)?,
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}
