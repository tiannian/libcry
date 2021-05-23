use super::{BarePublicKey, PublicKey};
use crate::primitive::bytes::{self, Bytes};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use digest::Digest;
use generic_array::GenericArray;

#[derive(Clone, Debug)]
pub struct Keypair<P: DisLogPoint, S: ScalarNumber> {
    pub seed: bytes::Output<S>,
    pub code: bytes::Output<S>,
    pub secret: Scalar<S>,
    pub public: Point<P>,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber> Keypair<P, S> {
    pub fn new<D: Digest>(seed: bytes::Output<S>) -> Self {
        assert_eq!(D::output_size(), S::SIZE * 2);
        let mut hasher = D::new();
        hasher.update(seed.clone());
        Keypair::new_from_hash(seed, hasher)
    }

    fn new_from_hash<D: Digest>(seed: bytes::Output<S>, hasher: D) -> Self {
        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        // here will clone, but it seems ok.
        let secret = Scalar::from_bytes(GenericArray::clone_from_slice(s));
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

    pub fn to_bare_public(&self) -> BarePublicKey<P> {
        BarePublicKey::from_keypair(self)
    }

    pub fn derive<D: Digest>(&self, id: bytes::Output<S>) -> Self {
        let mut hasher = D::new();
        hasher.update(self.public.to_bytes());
        hasher.update(&id);
        hasher.update(&self.code);

        let seed = GenericArray::default();

        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        // here will clone, but it seems ok.
        let part = Scalar::from_bytes(GenericArray::clone_from_slice(s));
        let secret = part + &self.secret;
        let public = Point::basepoint() * &secret;
        Self {
            seed,
            code: bytes::Output::<S>::clone_from_slice(c),
            secret,
            public,
        }
    }
}
