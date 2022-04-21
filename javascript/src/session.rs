use wasm_bindgen::prelude::*;

use crate::error_to_js;

use super::OlmMessage;

#[wasm_bindgen]
pub struct Session {
    pub(super) inner: vodozemac::olm::Session,
}

#[wasm_bindgen]
impl Session {
    #[wasm_bindgen(getter)]
    pub fn session_id(&self) -> String {
        self.inner.session_id()
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
