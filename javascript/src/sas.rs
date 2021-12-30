use wasm_bindgen::prelude::*;

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

    pub fn public_key(&self) -> String {
        self.inner.public_key_encoded().to_owned()
    }

    pub fn diffie_hellman(self, key: &str) -> EstablishedSas {
        let sas = self
            .inner
            .diffie_hellman_with_raw(key)
            .expect("Invalid public key");

        EstablishedSas { inner: sas }
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
        self.inner.calculate_mac(input, info)
    }

    pub fn verify_mac(&self, input: &str, info: &str, tag: &str) {
        self.inner
            .verify_mac(input, info, tag)
            .expect("Mac was invalid");
    }
}

#[wasm_bindgen]
pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

#[wasm_bindgen]
impl SasBytes {
    pub fn emoji_indices(&self) -> Vec<u8> {
        self.inner.emoji_indices().to_vec()
    }

    pub fn decimals(&self) -> Vec<u16> {
        let (first, second, third) = self.inner.decimals();

        [first, second, third].to_vec()
    }
}
