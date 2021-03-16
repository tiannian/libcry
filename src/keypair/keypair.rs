// use super::{BarePublicKey, PublicKey};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use core::convert::TryInto;
use digest::Digest;

#[derive(Clone, Debug)]
pub struct Keypair<P: DisLogPoint, S: ScalarNumber>
where
    [u8; S::BYTES_LENGTH]: Sized,
{
    pub seed: [u8; S::BYTES_LENGTH],
    pub code: [u8; S::BYTES_LENGTH],
    pub secret: Scalar<S>,
    pub public: Point<P>,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber> Keypair<P, S>
where
    [u8; S::BYTES_LENGTH]: Sized,
{
    pub fn new<D: Digest>(seed: [u8; S::BYTES_LENGTH]) -> Self {
        assert_eq!(D::output_size(), S::SIZE * 2);
        let mut hasher = D::new();
        hasher.update(seed.clone());
        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        // here will clone, but it seems ok.
        let secret = Scalar::from_bytes(s.try_into().unwrap());
        let public = Point::basepoint() * &secret;
        Self {
            seed,
            code: c.try_into().unwrap(),
            secret,
            public,
        }
    }

    //     pub fn to_public(&self) -> PublicKey<P, S> {
    //     PublicKey::from_keypair(self)
    // }
    //
    // pub fn to_bare_public(&self) -> BarePublicKey<P> {
    //     BarePublicKey::from_keypair(self)
    // }
}
