use super::{
    ffi::{InboundCreationResult, OlmMessageParts, OneTimeKey},
    Curve25519PublicKey, Ed25519PublicKey, Ed25519Signature, Session,
};

pub struct OlmMessage(pub(crate) vodozemac::olm::OlmMessage);

impl OlmMessage {
    pub fn to_parts(&self) -> OlmMessageParts {
        let (message_type, ciphertext) = self.0.clone().to_parts();

        OlmMessageParts {
            ciphertext,
            message_type,
        }
    }
}

pub fn olm_message_from_parts(parts: &OlmMessageParts) -> Result<Box<OlmMessage>, anyhow::Error> {
    Ok(OlmMessage(vodozemac::olm::OlmMessage::from_parts(
        parts.message_type,
        &parts.ciphertext,
    )?)
    .into())
}

pub struct Account(vodozemac::olm::Account);

pub fn new_account() -> Box<Account> {
    Account(vodozemac::olm::Account::new()).into()
}

pub fn account_from_pickle(
    pickle: &str,
    pickle_key: &[u8; 32],
) -> Result<Box<Account>, anyhow::Error> {
    let pickle = vodozemac::olm::AccountPickle::from_encrypted(pickle, pickle_key)?;
    Ok(Account(vodozemac::olm::Account::from_pickle(pickle)).into())
}

impl From<vodozemac::olm::InboundCreationResult> for InboundCreationResult {
    fn from(v: vodozemac::olm::InboundCreationResult) -> Self {
        Self {
            session: Session(v.session).into(),
            plaintext: v.plaintext,
        }
    }
}

impl Account {
    pub fn ed25519_key(&self) -> Box<Ed25519PublicKey> {
        Ed25519PublicKey(self.0.ed25519_key()).into()
    }

    pub fn curve25519_key(&self) -> Box<Curve25519PublicKey> {
        Curve25519PublicKey(self.0.curve25519_key()).into()
    }

    pub fn sign(&self, message: &str) -> Box<Ed25519Signature> {
        Ed25519Signature(self.0.sign(message)).into()
    }

    pub fn generate_one_time_keys(&mut self, count: usize) {
        self.0.generate_one_time_keys(count)
    }

    pub fn one_time_keys(&self) -> Vec<OneTimeKey> {
        self.0
            .one_time_keys()
            .into_iter()
            .map(|(key_id, key)| OneTimeKey {
                key_id: key_id.to_base64(),
                key: Curve25519PublicKey(key).into(),
            })
            .collect()
    }

    pub fn generate_fallback_key(&mut self) {
        self.0.generate_fallback_key()
    }

    pub fn fallback_key(&self) -> Vec<OneTimeKey> {
        self.0
            .fallback_key()
            .into_iter()
            .map(|(key_id, key)| OneTimeKey {
                key_id: key_id.to_base64(),
                key: Curve25519PublicKey(key).into(),
            })
            .collect()
    }

    pub fn mark_keys_as_published(&mut self) {
        self.0.mark_keys_as_published()
    }

    pub fn max_number_of_one_time_keys(&self) -> usize {
        self.0.max_number_of_one_time_keys()
    }

    pub fn create_outbound_session(
        &self,
        identity_key: &Curve25519PublicKey,
        one_time_key: &Curve25519PublicKey,
    ) -> Result<Box<Session>, vodozemac::KeyError> {
        let session = self
            .0
            .create_outbound_session(identity_key.0, one_time_key.0);

        Ok(Box::new(Session(session)))
    }

    pub fn create_inbound_session(
        &mut self,
        identity_key: &Curve25519PublicKey,
        message: &OlmMessage,
    ) -> Result<InboundCreationResult, anyhow::Error> {
        if let vodozemac::olm::OlmMessage::PreKey(m) = &message.0 {
            let result = self.0.create_inbound_session(identity_key.0, m)?;

            Ok(result.into())
        } else {
            anyhow::bail!("Invalid message type, a pre-key message is required")
        }
    }

    pub fn pickle(&self, pickle_key: &[u8; 32]) -> String {
        self.0.pickle().encrypt(pickle_key)
    }
}
