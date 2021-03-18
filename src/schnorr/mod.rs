use crate::keypair::{BarePublicKey, Keypair};
use crate::primitive::bytes::{Bytes, FromBytesRef};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use digest::Digest;

/// Schorr Signature
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Signature<P: DisLogPoint<Scalar = S>, S: ScalarNumber> {
    pub R: Point<P>,
    pub s: Scalar<S>,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber> Signature<P, S> {

    /// Create signature from keypair and message.
    #[allow(non_snake_case)]
    pub fn sign<D: Digest, M: AsRef<[u8]>>(sk: &Keypair<P, S>, message: &M) -> Signature<P, S> {
        let mut hasher_r = D::new();
        hasher_r.update(&sk.code);
        hasher_r.update(message.as_ref());
        let r = hasher_r.finalize();
        let r_scalar = Scalar::from_bytes_ref(&r).unwrap();
        let R = Point::basepoint() * &r_scalar;

        let mut hasher_s = D::new();
        hasher_s.update(R.to_bytes());
        hasher_s.update(sk.public.to_bytes());
        hasher_s.update(message.as_ref());
        let s_bytes = hasher_s.finalize();
        let a = Scalar::from_bytes_ref(&s_bytes).unwrap();

        let s = r_scalar + a * &sk.secret;

        Signature { R, s }
    }

    /// Verify signature 
    pub fn verify<D: Digest, M: AsRef<[u8]>>(&self, pk: &BarePublicKey<P>, message: &M) -> bool {
        let s_g = &self.s * Point::<P>::basepoint();

        let mut hasher = D::new();
        hasher.update(self.R.to_bytes());
        hasher.update(pk.public.to_bytes());
        hasher.update(message.as_ref());
        let s_bytes = hasher.finalize();
        let a = Scalar::from_bytes_ref(&s_bytes).unwrap();

        let rp = &self.R + a * &pk.public;

        s_g == rp
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ristretto255;
    use rand::RngCore;
    use sha3::Sha3_512;

    #[test]
    fn test_schnorr() {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let keypair =
            Keypair::<ristretto255::Point, ristretto255::Scalar>::new::<Sha3_512>(seed.into());
        let signature = Signature::sign::<Sha3_512, _>(&keypair, b"asda");
        let res = signature.verify::<Sha3_512, _>(&keypair.to_bare_public(), b"asda");
        assert!(res);
    }
}
