pub struct Curve25519PublicKey(pub(crate) vodozemac::Curve25519PublicKey);

impl Curve25519PublicKey {}

pub struct Ed25519PublicKey(pub(crate) vodozemac::Ed25519PublicKey);

pub struct Ed25519Signature(pub(crate) vodozemac::Ed25519Signature);
