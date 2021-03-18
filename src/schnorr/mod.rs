//! Schnorr signature and schnorr multi-signature.

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

    #[allow(non_snake_case)]
    pub fn sign_multi_party<D: Digest, M: AsRef<[u8]>>(
        sk: &Keypair<P, S>,
        rs: &[Point<P>],
        ps: &[BarePublicKey<P>],
        message: &M,
    ) -> (Signature<P, S>, BarePublicKey<P>) {
        let mut hasher_r = D::new();
        hasher_r.update(&sk.code);
        hasher_r.update(message.as_ref());
        let r_i = hasher_r.finalize();
        let r_scalar = Scalar::from_bytes_ref(&r_i).unwrap();
        let R_i = Point::basepoint() * &r_scalar;

        let mut hasher_a_i = D::new();
        for p in ps {
            hasher_a_i.update(p.public.to_bytes());
        }
        hasher_a_i.update(sk.public.to_bytes());
        let a_i = hasher_a_i.finalize();
        let a_scalar = Scalar::from_bytes_ref(&a_i).unwrap();

        let mut R = R_i.clone();
        for r in rs {
            R += r;
        }

        let mut X = Point::zero();
        for p in ps {
            X += &p.public;
        }

        let mut hasher_c = D::new();
        hasher_c.update(X.to_bytes());
        hasher_c.update(R.to_bytes());
        hasher_c.update(message.as_ref());
        let c = hasher_c.finalize();
        let c_scalar = Scalar::from_bytes_ref(&c).unwrap();

        let s = r_scalar + c_scalar * a_scalar * &sk.secret;
        let sign = Signature { R, s };
        let pk = BarePublicKey { public: X };
        (sign, pk)
    }

    pub fn sign_multi_party_complete(&mut self, signatures: &[Self]) {
        for signature in signatures {
            assert_eq!(self.R, signature.R);
            self.s += &signature.s;
        }
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
