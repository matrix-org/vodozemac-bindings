use super::ffi::OlmMessage;

pub struct Session {
    pub inner: vodozemac::olm::Session,
}

impl Session {
    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn encrypt(&mut self, plaintext: &str) -> OlmMessage {
        let message = self.inner.encrypt(plaintext);

        let (message_type, ciphertext) = message.to_tuple();

        OlmMessage {
            message_type,
            ciphertext,
        }
    }

    pub fn decrypt(&mut self, message: OlmMessage) -> Result<String, anyhow::Error> {
        let message = vodozemac::olm::OlmMessage::from_type_and_ciphertext(
            message.message_type,
            message.ciphertext,
        )
        .map_err(|_| {
            anyhow::anyhow!(
                "Invalid message type, got {}, expected 0 or 1",
                message.message_type
            )
        })?;

        Ok(self.inner.decrypt(&message)?)
    }
}
