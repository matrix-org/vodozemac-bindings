mod account;
mod session;
mod sas;

use account::{Account, new_account};
use sas::{Sas, EstablishedSas, SasBytes, new_sas};
use session::Session;

#[cxx::bridge]
mod ffi {
    #[namespace = "olm"]
    struct OlmMessage {
        message_type: usize,
        ciphertext: String,
    }

    #[namespace = "olm"]
    struct OneTimeKey {
        key_id: String,
        key: String,
    }

    #[namespace = "olm"]
    extern "Rust" {
        type Account;
        fn new_account() -> Box<Account>;
        fn ed25519_key(self: &Account) -> &str;
        fn curve25519_key(self: &Account) -> &str;
        fn sign(self: &Account, message: &str) -> String;
        fn generate_one_time_keys(self: &mut Account, count: usize);
        fn one_time_keys(self: &Account) -> Vec<OneTimeKey>;
        fn generate_fallback_key(self: &mut Account);
        fn fallback_key(self: &Account) -> Vec<OneTimeKey>;
        fn mark_keys_as_published(self: &mut Account);
        fn create_outbound_session(
            self: &Account,
            identity_key: &str,
            one_time_key: &str,
        ) -> Result<Box<Session>>;
        fn create_inbound_session(
            self: &mut Account,
            identity_key: &str,
            message: OlmMessage,
        ) -> Result<Box<Session>>;

        type Session;
        fn session_id(self: &Session) -> String;
        fn encrypt(self: &mut Session, plaintext: &str) -> OlmMessage;
        fn decrypt(self: &mut Session, message: OlmMessage) -> Result<String>;
    }

    #[namespace = "sas"]
    extern "Rust" {
        type Sas;
        fn new_sas() -> Box<Sas>;
        fn public_key(self: &Sas) -> &str;
        fn diffie_hellman(self: &Sas, other_public_key: &str) -> Result<Box<EstablishedSas>>;

        type EstablishedSas;
        fn bytes(self: &EstablishedSas, info: &str) -> Box<SasBytes>;
        fn calculate_mac(self: &EstablishedSas, input: &str, info: &str) -> String;
        fn verify_mac(self: &EstablishedSas, input: &str, info: &str, mac: &str) -> Result<()>;

        type SasBytes;
        fn emoji_indices(self: &SasBytes) -> [u8; 7];
        fn decimals(self: &SasBytes) -> [u16; 3];
    }
}
