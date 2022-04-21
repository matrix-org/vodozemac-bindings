use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::error_to_js;

use super::{session::Session, OlmMessage};

#[wasm_bindgen]
pub struct Account {
    inner: vodozemac::olm::Account,
}

#[wasm_bindgen]
pub struct InboundCreationResult {
    session: Session,
    plaintext: String,
}

#[wasm_bindgen]
impl InboundCreationResult {
    #[wasm_bindgen(getter)]
    pub fn session(self) -> Session {
        self.session
    }

    #[wasm_bindgen(getter)]
    pub fn plaintext(&self) -> String {
        self.plaintext.clone()
    }
}

impl From<vodozemac::olm::InboundCreationResult> for InboundCreationResult {
    fn from(result: vodozemac::olm::InboundCreationResult) -> Self {
        Self {
            session: Session {
                inner: result.session,
            },
            plaintext: result.plaintext,
        }
    }
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: vodozemac::olm::Account::new(),
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn ed25519_key(&self) -> String {
        self.inner.ed25519_key_encoded().to_owned()
    }

    #[wasm_bindgen(method, getter)]
    pub fn curve25519_key(&self) -> String {
        self.inner.curve25519_key_encoded().to_owned()
    }

    #[wasm_bindgen(method, getter)]
    pub fn max_number_of_one_time_keys(&self) -> usize {
        self.inner.max_number_of_one_time_keys()
    }

    pub fn sign(&self, message: &str) -> String {
        self.inner.sign(message).to_base64()
    }

    #[wasm_bindgen(method, getter)]
    pub fn one_time_keys(&self) -> Result<JsValue, JsValue> {
        let keys = self.inner.one_time_keys_encoded();

        Ok(serde_wasm_bindgen::to_value(&keys)?)
    }

    pub fn generate_one_time_keys(&mut self, count: usize) {
        self.inner.generate_one_time_keys(count)
    }

    #[wasm_bindgen(method, getter)]
    pub fn fallback_key(&self) -> Result<JsValue, JsValue> {
        let keys: HashMap<String, String> = self
            .inner
            .fallback_key()
            .into_iter()
            .map(|(k, v)| (k.to_base64(), v.to_base64()))
            .collect();

        Ok(serde_wasm_bindgen::to_value(&keys)?)
    }

    pub fn generate_fallback_key(&mut self) {
        self.inner.generate_fallback_key()
    }

    pub fn mark_keys_as_published(&mut self) {
        self.inner.mark_keys_as_published()
    }

    pub fn create_outbound_session(
        &self,
        identity_key: &str,
        one_time_key: &str,
    ) -> Result<Session, JsValue> {
        let identity_key =
            vodozemac::Curve25519PublicKey::from_base64(identity_key).map_err(error_to_js)?;
        let one_time_key =
            vodozemac::Curve25519PublicKey::from_base64(one_time_key).map_err(error_to_js)?;
        let session = self
            .inner
            .create_outbound_session(identity_key, one_time_key);

        Ok(Session { inner: session })
    }

    pub fn create_inbound_session(
        &mut self,
        identity_key: &str,
        message: &OlmMessage,
    ) -> Result<InboundCreationResult, JsValue> {
        let identity_key =
            vodozemac::Curve25519PublicKey::from_base64(identity_key).map_err(error_to_js)?;

        let message =
            vodozemac::olm::OlmMessage::from_parts(message.message_type, &message.ciphertext)
                .map_err(error_to_js)?;

        if let vodozemac::olm::OlmMessage::PreKey(message) = message {
            Ok(self
                .inner
                .create_inbound_session(&identity_key, &message)
                .map_err(error_to_js)?
                .into())
        } else {
            Err(JsError::new("Invalid message type, expected a pre-key message").into())
        }
    }
}
