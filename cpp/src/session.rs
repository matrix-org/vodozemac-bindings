use super::OlmMessage;

pub struct Session {
    pub inner: vodozemac::olm::Session,
}

impl Session {
    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn encrypt(&mut self, plaintext: &str) -> Box<OlmMessage> {
        OlmMessage(self.inner.encrypt(plaintext)).into()
    }

    pub fn decrypt(&mut self, message: &OlmMessage) -> Result<String, anyhow::Error> {
        Ok(self.inner.decrypt(&message.0)?)
    }
}
