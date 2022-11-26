use crate::{
    prelude::{Curve, FromToBytes, Point},
    Code,
};

/// Public key
pub struct PublicKey<C: Curve> {
    pub(crate) key: C::Point,
}

impl<C: Curve> PublicKey<C> {
    /// Derive sub public key
    pub fn derive(&self, code: &Code<C>) -> Self {
        let scalar = C::Scalar::create_from_bytes(&code.code);

        let mut key = C::Point::basepoint();
        key.mul(&scalar);
        key.add(&self.key);

        Self { key }
    }
}
