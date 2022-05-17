use wasm_bindgen::prelude::*;

use crate::error_to_js;

#[wasm_bindgen]
pub struct Sas {
    inner: vodozemac::sas::Sas,
}

#[wasm_bindgen]
impl Sas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: vodozemac::sas::Sas::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> String {
        self.inner.public_key().to_base64()
    }

    pub fn diffie_hellman(self, key: &str) -> Result<EstablishedSas, JsValue> {
        let sas = self
            .inner
            .diffie_hellman_with_raw(key)
            .map_err(error_to_js)?;

        Ok(EstablishedSas { inner: sas })
    }
}

#[wasm_bindgen]
pub struct EstablishedSas {
    inner: vodozemac::sas::EstablishedSas,
}

#[wasm_bindgen]
impl EstablishedSas {
    pub fn bytes(&self, info: &str) -> SasBytes {
        let bytes = self.inner.bytes(info);

        SasBytes { inner: bytes }
    }

    pub fn calculate_mac(&self, input: &str, info: &str) -> String {
        self.inner.calculate_mac(input, info).to_base64()
    }

    pub fn calculate_mac_invalid_base64(&self, input: &str, info: &str) -> String {
        self.inner.calculate_mac_invalid_base64(input, info)
    }

    pub fn verify_mac(&self, input: &str, info: &str, tag: &str) -> Result<(), JsValue> {
        let tag = vodozemac::sas::Mac::from_base64(tag).map_err(error_to_js)?;

        self.inner
            .verify_mac(input, info, &tag)
            .map_err(error_to_js)?;

        Ok(())
    }
}

#[wasm_bindgen]
pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

#[wasm_bindgen]
impl SasBytes {
    #[wasm_bindgen(getter)]
    pub fn emoji_indices(&self) -> Vec<u8> {
        self.inner.emoji_indices().to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn decimals(&self) -> Vec<u16> {
        let (first, second, third) = self.inner.decimals();

        [first, second, third].to_vec()
    }
}
