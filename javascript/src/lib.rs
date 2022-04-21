mod account;
mod sas;
mod session;

pub use account::Account;
pub use sas::{EstablishedSas, Sas, SasBytes};
pub use session::Session;

use wasm_bindgen::prelude::*;

fn error_to_js(error: impl std::error::Error) -> JsError {
    JsError::new(&error.to_string())
}

#[wasm_bindgen]
pub struct OlmMessage {
    ciphertext: String,
    message_type: usize,
}
