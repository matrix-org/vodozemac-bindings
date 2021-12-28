use pyo3::prelude::*;

use super::OlmMessage;

#[pyclass]
pub struct Session {
    pub(super) inner: vodozemac::olm::Session,
}

#[pymethods]
impl Session {
    #[getter]
    fn session_id(&self) -> String {
        self.inner.session_id()
    }

    fn encrypt(&mut self, plaintext: &str) -> OlmMessage {
        let message = self.inner.encrypt(plaintext);

        let (message_type, ciphertext) = message.to_tuple();

        OlmMessage {
            ciphertext,
            message_type,
        }
    }

    fn decrypt(&mut self, message: &OlmMessage) -> String {
        let message = vodozemac::olm::OlmMessage::from_type_and_ciphertext(
            message.message_type,
            message.ciphertext.to_owned(),
        )
        .unwrap();

        self.inner.decrypt(&message).unwrap()
    }
}
