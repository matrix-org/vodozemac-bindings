use wasm_bindgen::prelude::*;

use crate::error_to_js;

use vodozemac::megolm::{ExportedSessionKey, MegolmMessage, SessionKey};

#[wasm_bindgen]
pub struct GroupSession {
    pub(super) inner: vodozemac::megolm::GroupSession,
}

#[wasm_bindgen]
impl GroupSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: vodozemac::megolm::GroupSession::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    #[wasm_bindgen(getter)]
    pub fn session_key(&self) -> String {
        self.inner.session_key().to_base64()
    }

    #[wasm_bindgen(getter)]
    pub fn message_index(&self) -> u32 {
        self.inner.message_index()
    }

    pub fn encrypt(&mut self, plaintext: &str) -> String {
        self.inner.encrypt(plaintext).to_base64()
    }

    pub fn pickle(&self, pickle_key: &[u8]) -> Result<String, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }

    pub fn from_pickle(pickle: &str, pickle_key: &[u8]) -> Result<GroupSession, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;
        let pickle = vodozemac::megolm::GroupSessionPickle::from_encrypted(pickle, pickle_key)
            .map_err(error_to_js)?;

        let session = vodozemac::megolm::GroupSession::from_pickle(pickle);

        Ok(Self { inner: session })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct DecryptedMessage {
    pub plaintext: String,
    pub message_index: u32,
}

#[wasm_bindgen]
pub struct InboundGroupSession {
    pub(super) inner: vodozemac::megolm::InboundGroupSession,
}

#[wasm_bindgen]
impl InboundGroupSession {
    #[wasm_bindgen(constructor)]
    pub fn new(session_key: &str) -> Result<InboundGroupSession, JsValue> {
        let key = SessionKey::from_base64(session_key).map_err(error_to_js)?;

        Ok(Self {
            inner: vodozemac::megolm::InboundGroupSession::new(&key),
        })
    }

    pub fn import(session_key: &str) -> Result<InboundGroupSession, JsValue> {
        let key = ExportedSessionKey::from_base64(session_key).map_err(error_to_js)?;

        Ok(Self {
            inner: vodozemac::megolm::InboundGroupSession::import(&key),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    #[wasm_bindgen(getter)]
    pub fn first_known_index(&self) -> u32 {
        self.inner.first_known_index()
    }

    pub fn export_at(&mut self, index: u32) -> Option<String> {
        self.inner.export_at(index).map(|k| k.to_base64())
    }

    pub fn decrypt(&mut self, ciphertext: &str) -> Result<DecryptedMessage, JsValue> {
        let message = MegolmMessage::from_base64(ciphertext).map_err(error_to_js)?;
        let ret = self.inner.decrypt(&message).map_err(error_to_js)?;

        Ok(DecryptedMessage {
            plaintext: ret.plaintext,
            message_index: ret.message_index,
        })
    }

    pub fn pickle(&self, pickle_key: &[u8]) -> Result<String, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }

    pub fn from_pickle(pickle: &str, pickle_key: &[u8]) -> Result<InboundGroupSession, JsValue> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| JsError::new("Invalid pickle key length, expected 32 bytes"))?;
        let pickle =
            vodozemac::megolm::InboundGroupSessionPickle::from_encrypted(pickle, pickle_key)
                .map_err(error_to_js)?;

        let session = vodozemac::megolm::InboundGroupSession::from_pickle(pickle);

        Ok(Self { inner: session })
    }

    pub fn from_libolm_pickle(
        pickle: &str,
        pickle_key: &[u8],
    ) -> Result<InboundGroupSession, JsValue> {
        let inner = vodozemac::megolm::InboundGroupSession::from_libolm_pickle(pickle, pickle_key)
            .map_err(error_to_js)?;

        Ok(Self { inner })
    }
}
