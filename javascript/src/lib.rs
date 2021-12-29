mod account;
mod sas;
mod session;

pub use account::Account;
pub use sas::{EstablishedSas, Sas, SasBytes};
pub use session::Session;

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OlmMessage {
    ciphertext: JsString,
    message_type: usize,
}
