use pyo3::prelude::*;

#[pyclass]
pub struct Sas {
    inner: vodozemac::sas::Sas,
}

#[pymethods]
impl Sas {
    #[new]
    fn new() -> Self {
        Self {
            inner: vodozemac::sas::Sas::new(),
        }
    }

    fn public_key(&self) -> &str {
        self.inner.public_key_encoded()
    }

    fn diffie_hellman(&self, key: &str) -> EstablishedSas {
        let sas = self
            .inner
            .diffie_hellman_with_raw(key)
            .expect("Invalid public key");

        EstablishedSas { inner: sas }
    }
}

#[pyclass]
pub struct EstablishedSas {
    inner: vodozemac::sas::EstablishedSas,
}

#[pymethods]
impl EstablishedSas {
    fn bytes(&self, info: &str) -> SasBytes {
        let bytes = self.inner.bytes(info);

        SasBytes { inner: bytes }
    }

    fn calculate_mac(&self, input: &str, info: &str) -> String {
        self.inner.calculate_mac(input, info)
    }

    fn verify_mac(&self, input: &str, info: &str, tag: &str) {
        self.inner.verify_mac(input, info, tag).expect("Mac was invalid");
    }
}

#[pyclass]
pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

#[pymethods]
impl SasBytes {
    fn emoji_indices(&self) -> [u8; 7] {
        self.inner.emoji_indices()
    }

    fn decimals(&self) -> (u16, u16, u16) {
        self.inner.decimals()
    }
}
