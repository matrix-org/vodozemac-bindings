#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Curve25519PublicKey(pub(crate) vodozemac::Curve25519PublicKey);

impl Curve25519PublicKey {
    pub fn from_base64(key: &str) -> Result<Box<Curve25519PublicKey>, anyhow::Error> {
        Ok(Curve25519PublicKey(vodozemac::Curve25519PublicKey::from_base64(key)?).into())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}

pub fn curve_key_from_base64(key: &str) -> Result<Box<Curve25519PublicKey>, anyhow::Error> {
    Curve25519PublicKey::from_base64(key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ed25519PublicKey(pub(crate) vodozemac::Ed25519PublicKey);

impl Ed25519PublicKey {
    pub fn from_base64(key: &str) -> Result<Box<Ed25519PublicKey>, anyhow::Error> {
        Ok(Ed25519PublicKey(vodozemac::Ed25519PublicKey::from_base64(key)?).into())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}

pub fn ed25519_key_from_base64(key: &str) -> Result<Box<Ed25519PublicKey>, anyhow::Error> {
    Ed25519PublicKey::from_base64(key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ed25519Signature(pub(crate) vodozemac::Ed25519Signature);
