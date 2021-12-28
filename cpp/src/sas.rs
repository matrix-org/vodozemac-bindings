pub struct Sas {
    inner: vodozemac::sas::Sas,
}

pub fn new_sas() -> Box<Sas> {
    Box::new(Sas {
        inner: vodozemac::sas::Sas::new(),
    })
}

impl Sas {
    pub fn public_key(&self) -> &str {
        self.inner.public_key_encoded()
    }

    pub fn diffie_hellman(
        &self,
        other_public_key: &str,
    ) -> Result<Box<EstablishedSas>, vodozemac::sas::PublicKeyError> {
        self.inner
            .diffie_hellman_with_raw(other_public_key)
            .map(|s| Box::new(EstablishedSas { inner: s }))
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
