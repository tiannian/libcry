use crate::{
    prelude::{Curve, FromToBytes, Point},
    PrivateKey, PublicKey,
};

/// Shared key generate by DH.
pub struct SharedKey<C: Curve> {
    pub(crate) key: C::Point,
}

impl<C: Curve> AsRef<[u8]> for SharedKey<C> {
    fn as_ref(&self) -> &[u8] {
        self.key.to_bytes().as_ref()
    }
}

impl<C: Curve> SharedKey<C> {
    pub fn exchange(sk: &PrivateKey<C>, pk: PublicKey<C>) -> Self {
        let mut key = pk.key;
        key.mul(&sk.key);

        Self { key }
    }
}
