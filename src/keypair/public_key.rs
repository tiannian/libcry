use super::BarePublicKey;
use super::Keypair;
use crate::primitive::bytes::{self, Bytes};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
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

    pub fn derive<D: Digest>(&self, id: bytes::Output<S>) -> Self {
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

    pub fn to_bare_public(&self) -> BarePublicKey<P> {
        BarePublicKey {
            public: self.public.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ristretto255;
    use rand::RngCore;
    use sha3::Sha3_512;

    #[test]
    fn test_key_derive() {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let keypair =
            Keypair::<ristretto255::Point, ristretto255::Scalar>::new::<Sha3_512>(seed.into());

        let root_public_key = keypair.to_public();

        let mut rng = rand::thread_rng();
        let mut id_array = [0u8; 32];
        rng.fill_bytes(&mut id_array);

        let sub_public_key = root_public_key.derive::<Sha3_512>(id_array.into());

        let sub_keypair = keypair.derive::<Sha3_512>(id_array.into());

        assert_eq!(sub_keypair.public, sub_public_key.public);
    }
}
