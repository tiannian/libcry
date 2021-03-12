use super::Keypair;
use crate::primitive::bytes::{Bytes, self};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{ScalarNumber, Scalar};
use digest::Digest;
use generic_array::GenericArray;

pub struct PublicKey<P: DisLogPoint, S: ScalarNumber> {
    pub code: bytes::Output<S>,
    pub public: Point<P>,
}

impl<P: DisLogPoint, S: ScalarNumber> PublicKey<P, S> {
    pub fn from_keypair(keypair: &Keypair<P, S>) -> Self {
        PublicKey {
            code: keypair.code.clone(),
            public: keypair.public.clone(),
        }
    }

    pub fn derive<D: Digest>(&self, id:bytes::Output<S>) -> Self {
        let mut hasher = D::new();
        hasher.update(self.public.to_bytes());
        hasher.update(&id);
        hasher.update(&self.code);

        let result = hasher.finalize();
        let (c, s) = result.split_at(result.len() / 2);
        // here will clone, but it seems ok.
        let part = Scalar::from_bytes(GenericArray::clone_from_slice(s));
        let public_part = part * Point::basepoint();
        let public = &self.public + public_part;
        Self {
            code: bytes::Output::<S>::clone_from_slice(c),
            public,
        }
    }
}
