pub struct Sas {
    inner: Option<vodozemac::sas::Sas>,
    public_key: String,
}

pub fn new_sas() -> Box<Sas> {
    let sas = vodozemac::sas::Sas::new();
    let public_key = sas.public_key_encoded().to_owned();

    Box::new(Sas {
        inner: Some(sas),
        public_key,
    })
}

impl Sas {
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn diffie_hellman(
        &mut self,
        other_public_key: &str,
    ) -> Result<Box<EstablishedSas>, anyhow::Error> {
        if let Some(sas) = self.inner.take() {
            let sas = sas.diffie_hellman_with_raw(other_public_key)?;

            Ok(Box::new(EstablishedSas { inner: sas }))
        } else {
            anyhow::bail!("The sas object has been already used")
        }
    }
}

pub struct EstablishedSas {
    inner: vodozemac::sas::EstablishedSas,
}

impl EstablishedSas {
    pub fn bytes(&self, info: &str) -> Box<SasBytes> {
        Box::new(SasBytes {
            inner: self.inner.bytes(info),
        })
    }

    pub fn calculate_mac(&self, input: &str, info: &str) -> String {
        self.inner.calculate_mac(input, info)
    }

    pub fn verify_mac(
        &self,
        input: &str,
        info: &str,
        tag: &str,
    ) -> Result<(), vodozemac::sas::SasError> {
        self.inner.verify_mac(input, info, tag)
    }
}

pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

impl SasBytes {
    pub fn emoji_indices(&self) -> [u8; 7] {
        self.inner.emoji_indices()
    }

    pub fn decimals(&self) -> [u16; 3] {
        let (first, second, third) = self.inner.decimals();

        [first, second, third]
    }
}
