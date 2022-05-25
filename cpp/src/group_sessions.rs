use super::ffi::DecryptedMessage;
use anyhow::{anyhow, Result};

pub struct GroupSession(vodozemac::megolm::GroupSession);

pub fn new_group_session() -> Box<GroupSession> {
    GroupSession::new().into()
}

pub struct MegolmMessage(vodozemac::megolm::MegolmMessage);

pub fn megolm_message_from_base64(message: &str) -> Result<Box<MegolmMessage>> {
    Ok(MegolmMessage(vodozemac::megolm::MegolmMessage::from_base64(message)?).into())
}

impl MegolmMessage {
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}

pub struct SessionKey(vodozemac::megolm::SessionKey);

pub fn session_key_from_base64(message: &str) -> Result<Box<SessionKey>> {
    Ok(SessionKey(vodozemac::megolm::SessionKey::from_base64(message)?).into())
}

impl SessionKey {
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}

pub struct ExportedSessionKey(vodozemac::megolm::ExportedSessionKey);

pub fn exported_session_key_from_base64(message: &str) -> Result<Box<ExportedSessionKey>> {
    Ok(ExportedSessionKey(vodozemac::megolm::ExportedSessionKey::from_base64(message)?).into())
}

impl ExportedSessionKey {
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}

impl GroupSession {
    fn new() -> Self {
        Self(vodozemac::megolm::GroupSession::new())
    }

    pub fn session_id(&self) -> String {
        self.0.session_id()
    }

    pub fn message_index(&self) -> u32 {
        self.0.message_index()
    }

    pub fn session_key(&self) -> Box<SessionKey> {
        SessionKey(self.0.session_key()).into()
    }

    pub fn encrypt(&mut self, plaintext: &str) -> Box<MegolmMessage> {
        MegolmMessage(self.0.encrypt(plaintext)).into()
    }

    pub fn pickle(&self, pickle_key: &[u8; 32]) -> String {
        self.0.pickle().encrypt(pickle_key)
    }
}

pub fn group_session_from_pickle(pickle: &str, pickle_key: &[u8; 32]) -> Result<Box<GroupSession>> {
    let pickle = vodozemac::megolm::GroupSessionPickle::from_encrypted(pickle, pickle_key)?;
    Ok(GroupSession(vodozemac::megolm::GroupSession::from_pickle(pickle)).into())
}

pub struct InboundGroupSession(vodozemac::megolm::InboundGroupSession);

pub fn new_inbound_group_session(session_key: &SessionKey) -> Box<InboundGroupSession> {
    InboundGroupSession::new(session_key).into()
}

pub fn import_inbound_group_session(session_key: &ExportedSessionKey) -> Box<InboundGroupSession> {
    InboundGroupSession::import(session_key).into()
}

pub fn inbound_group_session_from_pickle(
    pickle: &str,
    pickle_key: &[u8; 32],
) -> Result<Box<InboundGroupSession>> {
    let pickle = vodozemac::megolm::InboundGroupSessionPickle::from_encrypted(pickle, pickle_key)?;
    Ok(InboundGroupSession(vodozemac::megolm::InboundGroupSession::from_pickle(pickle)).into())
}

impl InboundGroupSession {
    fn new(session_key: &SessionKey) -> Self {
        Self(vodozemac::megolm::InboundGroupSession::new(&session_key.0))
    }

    fn import(session_key: &ExportedSessionKey) -> Self {
        Self(vodozemac::megolm::InboundGroupSession::import(
            &session_key.0,
        ))
    }

    pub fn session_id(&self) -> String {
        self.0.session_id()
    }

    pub fn first_known_index(&self) -> u32 {
        self.0.first_known_index()
    }

    pub fn export_at(&mut self, index: u32) -> Result<Box<ExportedSessionKey>> {
        self.0
            .export_at(index)
            .map(ExportedSessionKey)
            .map(Box::new)
            .ok_or_else(|| anyhow!("Unknown message index"))
    }

    pub fn decrypt(&mut self, message: &MegolmMessage) -> Result<DecryptedMessage> {
        let ret = self.0.decrypt(&message.0)?;

        Ok(DecryptedMessage {
            plaintext: ret.plaintext,
            message_index: ret.message_index,
        })
    }

    pub fn pickle(&self, pickle_key: &[u8; 32]) -> String {
        self.0.pickle().encrypt(pickle_key)
    }
}
