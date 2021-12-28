use super::ffi::{OlmMessage, OneTimeKey};
use super::Session;

pub struct Account {
    inner: vodozemac::olm::Account,
}

pub fn new_account() -> Box<Account> {
    Box::new(Account {
        inner: vodozemac::olm::Account::new(),
    })
}

impl Account {
    pub fn ed25519_key(&self) -> &str {
        self.inner.ed25519_key_encoded()
    }

    pub fn curve25519_key(&self) -> &str {
        self.inner.curve25519_key_encoded()
    }

    pub fn sign(&self, message: &str) -> String {
        self.inner.sign(message)
    }

    pub fn generate_one_time_keys(&mut self, count: usize) {
        self.inner.generate_one_time_keys(count)
    }

    pub fn one_time_keys(&self) -> Vec<OneTimeKey> {
        self.inner
            .one_time_keys_encoded()
            .into_iter()
            .map(|(key_id, key)| OneTimeKey { key_id, key })
            .collect()
    }

    pub fn generate_fallback_key(&mut self) {
        self.inner.generate_fallback_key()
    }

    pub fn fallback_key(&self) -> Vec<OneTimeKey> {
        self.inner
            .fallback_key()
            .into_iter()
            .map(|(key_id, key)| OneTimeKey {
                key_id: key_id.to_base64(),
                key,
            })
            .collect()
    }

    pub fn mark_keys_as_published(&mut self) {
        self.inner.mark_keys_as_published()
    }

    pub fn create_outbound_session(
        &self,
        identity_key: &str,
        one_time_key: &str,
    ) -> Result<Box<Session>, vodozemac::Curve25519KeyError> {
        let identity_key = vodozemac::Curve25519PublicKey::from_base64(identity_key)?;
        let one_time_key = vodozemac::Curve25519PublicKey::from_base64(one_time_key)?;

        let session = self
            .inner
            .create_outbound_session(identity_key, one_time_key);

        Ok(Box::new(Session { inner: session }))
    }

    pub fn create_inbound_session(
        &mut self,
        identity_key: &str,
        message: OlmMessage,
    ) -> Result<Box<Session>, anyhow::Error> {
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

        if let vodozemac::olm::OlmMessage::PreKey(m) = message {
            let identity_key = vodozemac::Curve25519PublicKey::from_base64(identity_key)?;

            let session = self.inner.create_inbound_session(&identity_key, &m)?;

            Ok(Box::new(Session { inner: session }))
        } else {
            anyhow::bail!("Invalid message type, a pre-key message is required")
        }
    }
}
