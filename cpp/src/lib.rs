mod account;
mod sas;
mod session;
mod types;

use account::{account_from_pickle, new_account, Account, OlmMessage, olm_message_from_parts};
use sas::{new_sas, EstablishedSas, Mac, Sas, SasBytes};
use session::{session_from_pickle, Session};
use types::{
    curve_key_from_base64, ed25519_key_from_base64, Curve25519PublicKey, Ed25519PublicKey,
    Ed25519Signature,
};

#[cxx::bridge]
mod ffi {
    #[namespace = "olm"]
    struct OlmMessageParts {
        message_type: usize,
        ciphertext: String,
    }

    #[namespace = "olm"]
    pub struct InboundCreationResult {
        pub session: Box<Session>,
        pub plaintext: String,
    }

    #[namespace = "olm"]
    struct OneTimeKey {
        key_id: String,
        key: Box<Curve25519PublicKey>,
    }

    #[namespace = "olm"]
    #[derive(PartialEq)]
    struct SessionKeys {
        identity_key: Box<Curve25519PublicKey>,
        base_key: Box<Curve25519PublicKey>,
        one_time_key: Box<Curve25519PublicKey>,
    }

    #[namespace = "types"]
    extern "Rust" {
        type Curve25519PublicKey;
        fn curve_key_from_base64(key: &str) -> Result<Box<Curve25519PublicKey>>;
        fn to_base64(self: &Curve25519PublicKey) -> String;
        type Ed25519PublicKey;
        fn ed25519_key_from_base64(key: &str) -> Result<Box<Ed25519PublicKey>>;
        fn to_base64(self: &Ed25519PublicKey) -> String;
        type Ed25519Signature;
    }

    #[namespace = "olm"]
    extern "Rust" {
        type Account;
        fn new_account() -> Box<Account>;
        fn ed25519_key(self: &Account) -> Box<Ed25519PublicKey>;
        fn curve25519_key(self: &Account) -> Box<Curve25519PublicKey>;
        fn sign(self: &Account, message: &str) -> Box<Ed25519Signature>;
        fn generate_one_time_keys(self: &mut Account, count: usize);
        fn one_time_keys(self: &Account) -> Vec<OneTimeKey>;
        fn generate_fallback_key(self: &mut Account);
        fn fallback_key(self: &Account) -> Vec<OneTimeKey>;
        fn mark_keys_as_published(self: &mut Account);
        fn max_number_of_one_time_keys(self: &Account) -> usize;
        fn account_from_pickle(pickle: &str, pickle_key: &[u8; 32]) -> Result<Box<Account>>;
        fn pickle(self: &Account, pickle_key: &[u8; 32]) -> String;
        fn create_outbound_session(
            self: &Account,
            identity_key: &Curve25519PublicKey,
            one_time_key: &Curve25519PublicKey,
        ) -> Result<Box<Session>>;
        fn create_inbound_session(
            self: &mut Account,
            identity_key: &Curve25519PublicKey,
            message: &OlmMessage,
        ) -> Result<InboundCreationResult>;

        type Session;
        fn session_id(self: &Session) -> String;
        fn session_keys(self: &Session) -> SessionKeys;
        fn session_matches(self: &Session, message: &OlmMessage) -> bool;
        fn encrypt(self: &mut Session, plaintext: &str) -> Box<OlmMessage>;
        fn decrypt(self: &mut Session, message: &OlmMessage) -> Result<String>;
        fn session_from_pickle(pickle: &str, pickle_key: &[u8; 32]) -> Result<Box<Session>>;
        fn pickle(self: &Session, pickle_key: &[u8; 32]) -> String;

        type OlmMessage;
        fn to_parts(self: &OlmMessage) -> OlmMessageParts;
        fn olm_message_from_parts(parts: &OlmMessageParts) -> Result<Box<OlmMessage>>;
    }

    #[namespace = "sas"]
    extern "Rust" {
        type Mac;
        type Sas;
        fn new_sas() -> Box<Sas>;
        fn public_key(self: &Sas) -> Box<Curve25519PublicKey>;
        fn diffie_hellman(
            self: &mut Sas,
            other_public_key: &Curve25519PublicKey,
        ) -> Result<Box<EstablishedSas>>;

        type EstablishedSas;
        fn bytes(self: &EstablishedSas, info: &str) -> Box<SasBytes>;
        fn calculate_mac(self: &EstablishedSas, input: &str, info: &str) -> Box<Mac>;
        fn verify_mac(self: &EstablishedSas, input: &str, info: &str, mac: &Mac) -> Result<()>;

        type SasBytes;
        fn emoji_indices(self: &SasBytes) -> [u8; 7];
        fn decimals(self: &SasBytes) -> [u16; 3];
    }
}
