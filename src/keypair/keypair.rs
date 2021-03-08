use crate::primitive::point::{Point, DisLogPoint};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use crate::primitive::bytes::{Bytes, self};
use digest::Digest;
use super::public_key::PublicKey;

pub struct Keypair<P: DisLogPoint, S: ScalarNumber> {
    pub(crate) seed: bytes::Output<S>,
    pub(crate) code: bytes::Output<S>,
    pub(crate) secret: Scalar<S>,
    pub(crate) public: Point<P>,
}

impl<P: DisLogPoint, S: ScalarNumber> Keypair<P, S> {
    pub fn new<D: Digest>(seed: bytes::Output<S>) -> Self {
        let mut hasher = D::new();
        hasher.update(seed.clone());
        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        let secret = Scalar::from_bytes(s);
        let public = Point::one() * &secret;
        Self {
            seed,
            code: bytes::Output::<S>::clone_from_slice(c),
            secret,
            public
        }
    }

    pub fn to_public(&self) -> PublicKey<P, S> {
        PublicKey::from_keypair(self)
    }
}

