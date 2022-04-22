#![allow(clippy::new_without_default)]

mod account;
mod group_sessions;
mod sas;
mod session;

pub use account::Account;
pub use sas::{EstablishedSas, Sas, SasBytes};
pub use session::Session;

use wasm_bindgen::prelude::*;

fn error_to_js(error: impl std::error::Error) -> JsError {
    JsError::new(&error.to_string())
}

#[wasm_bindgen(getter_with_clone, setter)]
pub struct OlmMessage {
    pub ciphertext: String,
    pub message_type: usize,
}

#[wasm_bindgen]
impl OlmMessage {
    #[wasm_bindgen(constructor)]
    pub fn new(message_type: usize, ciphertext: String) -> Self {
        Self {
            ciphertext,
            message_type,
        }
    }
}
