mod account;
mod sas;
mod session;

use pyo3::prelude::*;

#[pyclass]
pub struct OlmMessage {
    #[pyo3(get)]
    ciphertext: String,
    #[pyo3(get)]
    message_type: usize,
}

#[pymodule]
#[pyo3(name = "vodozemac")]
fn mymodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<account::Account>()?;
    m.add_class::<session::Session>()?;
    m.add_class::<OlmMessage>()?;
    m.add_class::<sas::Sas>()?;

    Ok(())
}
