use apollo_crypto_core as apollo;
use apollo_crypto_core::Base64;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use curve25519_dalek::edwards::{EdwardsPoint, CompressedEdwardsY};
use curve25519_dalek::scalar::Scalar;


#[pyclass]
struct KeyPair {
    key_pair: apollo::KeyPair
}

#[pymethods]
impl KeyPair {
    pub fn public_key(&self) -> String {
        self.key_pair.pk.pk.b64_serialize()
    }

    pub fn secret_key(&self) -> String {
        self.key_pair.sk.sk.b64_serialize()
    }
}

impl Into<apollo::KeyPair> for KeyPair {
    fn into(self) -> apollo::KeyPair {
        self.key_pair
    }
}

impl Into<KeyPair> for apollo::KeyPair {
    fn into(self) -> KeyPair {
        KeyPair { key_pair: self }
    }
}


#[pyclass]
struct KeyGenerator {
    key_generator: apollo::KeyGenerator
}

#[pymethods]
impl KeyGenerator {
    #[new]
    fn new() -> KeyGenerator {
        KeyGenerator {
            key_generator: apollo::KeyGenerator::new()
        }
    }

    pub fn generate(&self) -> KeyPair {
        self.key_generator.generate().into()
    }
}

#[pyclass]
struct ElGamal {
    el_gamal: apollo::ElGamal
}

#[pymethods]
impl ElGamal {
    #[new]
    fn new() -> ElGamal {
        ElGamal { el_gamal: apollo::ElGamal::new() }
    }

    pub fn generate_plaintexts(&self) -> (String, String) {
        let (p0, p1) = self.el_gamal.generate_plaintexts();
        (
            p0.b64_serialize(),
            p1.b64_serialize()
        )
    }

    pub fn encrypt(&self, pk: &str, message: &str) -> (String, String) {
        let public_key = apollo::PublicKey { pk: EdwardsPoint::b64_deserialize(pk) };
        let (c1, c2) = self.el_gamal.encrypt(&public_key, EdwardsPoint::b64_deserialize(&message));
        (
            c1.b64_serialize(),
            c2.b64_serialize()
        )
    }

    pub fn decrypt(&self, sk: &str, c1: &str, c2: &str) -> String {
        let secret_key = apollo::SecretKey { sk: Scalar::b64_deserialize(sk) };
        let message = self.el_gamal.decrypt(&secret_key, (EdwardsPoint::b64_deserialize(c1), EdwardsPoint::b64_deserialize(c2)));
        message.b64_serialize()
    }
}

#[pymodule]
fn apollo_crypto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>().unwrap();
    m.add_class::<KeyGenerator>().unwrap();
    m.add_class::<ElGamal>().unwrap();
    Ok(())
}
