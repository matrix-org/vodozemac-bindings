use wasm_bindgen::prelude::*;

use crate::error_to_js;

use super::OlmMessage;

#[wasm_bindgen]
pub struct Session {
    pub(super) inner: vodozemac::olm::Session,
}

#[wasm_bindgen]
impl Session {
    pub fn pickle(&self, pickle_key: &[u8]) -> Result<String, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }

    pub fn from_pickle(pickle: &str, pickle_key: &[u8]) -> Result<Session, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;
        let pickle = vodozemac::olm::SessionPickle::from_encrypted(pickle, pickle_key)
            .map_err(error_to_js)?;

        let session = vodozemac::olm::Session::from_pickle(pickle);

        Ok(Self { inner: session })
    }

    pub fn from_libolm_pickle(pickle: &str, pickle_key: &[u8]) -> Result<Session, JsValue> {
        let session =
            vodozemac::olm::Session::from_libolm_pickle(pickle, pickle_key).map_err(error_to_js)?;

        Ok(Self { inner: session })
    }

    #[wasm_bindgen(getter)]
    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn session_matches(&self, message: &OlmMessage) -> bool {
        let message =
            vodozemac::olm::OlmMessage::from_parts(message.message_type, &message.ciphertext);

        match message {
            Ok(m) => {
                if let vodozemac::olm::OlmMessage::PreKey(m) = m {
                    self.inner.session_keys() == m.session_keys()
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn encrypt(&mut self, plaintext: &str) -> OlmMessage {
        let message = self.inner.encrypt(plaintext);

        let (message_type, ciphertext) = message.to_parts();

        OlmMessage {
            ciphertext,
            message_type,
        }
    }

    pub fn decrypt(&mut self, message: &OlmMessage) -> Result<String, JsValue> {
        let message =
            vodozemac::olm::OlmMessage::from_parts(message.message_type, &message.ciphertext)
                .map_err(error_to_js)?;

        Ok(self.inner.decrypt(&message).map_err(error_to_js)?)
    }
}
