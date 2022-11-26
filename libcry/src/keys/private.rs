use crate::{
    prelude::{Curve, FromToBytes, Point, Scalar},
    Code, PublicKey,
};

/// Private key
pub struct PrivateKey<C: Curve> {
    pub(crate) key: C::Scalar,
}

impl<C: Curve> PrivateKey<C> {
    /// Generate private key from seed.
    pub fn from_seed(seed: &Code<C>) -> Self {
        let key = C::Scalar::create_from_bytes(&seed.code);

        Self { key }
    }

    /// Build public key.
    pub fn to_public(&self) -> PublicKey<C> {
        let mut key = C::Point::basepoint();

        key.mul(&self.key);

        PublicKey { key }
    }

    /// Derive sub key
    pub fn derive(&self, code: &Code<C>) -> Self {
        let mut key = C::Scalar::create_from_bytes(&code.code);

        key.add(&self.key);

        Self { key }
    }
}
