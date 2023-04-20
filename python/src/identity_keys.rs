use crate::error::*;
use pyo3::prelude::*;

#[pyclass]
pub struct Ed25519PublicKey {
    inner: vodozemac::Ed25519PublicKey,
}

#[pymethods]
impl Ed25519PublicKey {
    #[new]
    pub fn from_base64(key: &str) -> Result<Self, SessionKeyDecodeError> {
        Ok(Self {
            inner: vodozemac::Ed25519PublicKey::from_base64(key).unwrap(),
        })
    }

    pub fn verify_signature(
        &self,
        message: &str,
        signature: &Ed25519Signature,
    ) -> Result<(), KeyError> {
        self.inner
            .verify(message.as_bytes(), &signature.inner)
            .unwrap();

        Ok(())
    }
}

#[pyclass]
pub struct Ed25519Signature {
    inner: vodozemac::Ed25519Signature,
}

#[pymethods]
impl Ed25519Signature {
    #[new]
    pub fn from_base64(session_key: &str) -> Result<Self, SessionKeyDecodeError> {
        Ok(Self {
            inner: vodozemac::Ed25519Signature::from_base64(session_key).unwrap(),
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

#[pyclass]
pub struct Curve25519PublicKey {
    inner: vodozemac::Curve25519PublicKey,
}

#[pymethods]
impl Curve25519PublicKey {
    #[new]
    pub fn from_base64(key: &str) -> Result<Self, SessionKeyDecodeError> {
        Ok(Self {
            inner: vodozemac::Curve25519PublicKey::from_base64(key).unwrap(),
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}
