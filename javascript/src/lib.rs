mod account;
mod sas;
mod session;

pub use account::Account;
pub use sas::{EstablishedSas, Sas, SasBytes};
pub use session::Session;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OlmMessage {
    ciphertext: String,
    message_type: usize,
}
