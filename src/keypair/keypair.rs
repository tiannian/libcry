use super::public_key::PublicKey;
use crate::primitive::bytes::{self, Bytes};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use digest::Digest;

pub struct Keypair<P: DisLogPoint, S: ScalarNumber> {
    pub seed: bytes::Output<S>,
    pub code: bytes::Output<S>,
    pub secret: Scalar<S>,
    pub public: Point<P>,
}

impl<P: DisLogPoint, S: ScalarNumber> Keypair<P, S> {
    pub fn new<D: Digest>(seed: bytes::Output<S>) -> Self {
        let mut hasher = D::new();
        hasher.update(seed.clone());
        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        let secret = Scalar::from_bytes(s);
        let public = Point::basepoint() * &secret;
        Self {
            seed,
            code: bytes::Output::<S>::clone_from_slice(c),
            secret,
            public,
        }
    }

    pub fn to_public(&self) -> PublicKey<P, S> {
        PublicKey::from_keypair(self)
    }
}
