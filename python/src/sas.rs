use pyo3::prelude::*;

#[pyclass]
pub struct Sas {
    inner: Option<vodozemac::sas::Sas>,
    public_key: String,
}

#[pymethods]
impl Sas {
    #[new]
    fn new() -> Self {
        let sas = vodozemac::sas::Sas::new();
        let public_key = sas.public_key_encoded().to_string();

        Self {
            inner: Some(sas),
            public_key,
        }
    }

    #[getter]
    fn public_key(&self) -> &str {
        &self.public_key
    }

    fn diffie_hellman(&mut self, key: &str) -> Option<EstablishedSas> {
        let sas = self.inner.take();

        sas.map(|s| {
            let sas = s.diffie_hellman_with_raw(key).expect("Invalid public key");

            EstablishedSas { inner: sas }
        })
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
        self.inner
            .verify_mac(input, info, tag)
            .expect("Mac was invalid");
    }
}

#[pyclass]
pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

#[pymethods]
impl SasBytes {
    #[getter]
    fn emoji_indices(&self) -> [u8; 7] {
        self.inner.emoji_indices()
    }

    #[getter]
    fn decimals(&self) -> (u16, u16, u16) {
        self.inner.decimals()
    }
}
