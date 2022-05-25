use super::Curve25519PublicKey;

pub struct Sas {
    inner: Option<vodozemac::sas::Sas>,
    public_key: vodozemac::Curve25519PublicKey,
}

pub fn new_sas() -> Box<Sas> {
    let sas = vodozemac::sas::Sas::new();
    let public_key = sas.public_key();

    Box::new(Sas {
        inner: Some(sas),
        public_key,
    })
}

impl Sas {
    pub fn public_key(&self) -> Box<Curve25519PublicKey> {
        Curve25519PublicKey(self.public_key).into()
    }

    pub fn diffie_hellman(
        &mut self,
        other_public_key: &Curve25519PublicKey,
    ) -> Result<Box<EstablishedSas>, anyhow::Error> {
        if let Some(sas) = self.inner.take() {
            let sas = sas.diffie_hellman(other_public_key.0)?;

            Ok(Box::new(EstablishedSas { inner: sas }))
        } else {
            anyhow::bail!("The sas object has been already used")
        }
    }
}

pub struct Mac(vodozemac::sas::Mac);

pub fn mac_from_base64(mac: &str) -> Result<Box<Mac>, anyhow::Error> {
    Ok(Mac(vodozemac::sas::Mac::from_base64(mac)?).into())
}

impl Mac {
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
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

    pub fn calculate_mac(&self, input: &str, info: &str) -> Box<Mac> {
        Mac(self.inner.calculate_mac(input, info)).into()
    }

    pub fn verify_mac(
        &self,
        input: &str,
        info: &str,
        tag: &Mac,
    ) -> Result<(), vodozemac::sas::SasError> {
        self.inner.verify_mac(input, info, &tag.0)
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
