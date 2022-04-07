use wasm_bindgen::prelude::*;

use super::OlmMessage;

#[wasm_bindgen]
pub struct Session {
    pub(super) inner: vodozemac::olm::Session,
}

#[wasm_bindgen]
impl Session {
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

    pub fn decrypt(&mut self, message: &OlmMessage) -> String {
        let message =
            vodozemac::olm::OlmMessage::from_parts(message.message_type, &message.ciphertext)
                .unwrap();

        self.inner.decrypt(&message).unwrap()
    }
}
