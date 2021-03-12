use crate::keypair::{BarePublicKey, Keypair};
use crate::primitive::bytes::Bytes;
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use digest::Digest;

#[allow(non_snake_case)]
pub struct Signature<P: DisLogPoint<Scalar = S>, S: ScalarNumber> {
    pub R: Point<P>,
    pub s: Scalar<S>,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber> Signature<P, S> {
    #[allow(non_snake_case)]
    pub fn sign<D: Digest, M: AsRef<[u8]>>(sk: Keypair<P, S>, message: M) -> Signature<P, S> {
        let mut hasher_r = D::new();
        hasher_r.update(sk.code);
        hasher_r.update(message.as_ref());
        let r = hasher_r.finalize();
        let r_scalar = Scalar::<S>::from_bytes(r.as_ref());
        let R = Point::basepoint() * &r_scalar;

        let mut hasher_s = D::new();
        hasher_s.update(R.to_bytes());
        hasher_s.update(sk.public.to_bytes());
        hasher_s.update(message.as_ref());
        let s_bytes = hasher_s.finalize();
        let a = Scalar::from_bytes(s_bytes.as_ref());

        let s = r_scalar + a * sk.secret;

        Signature { R, s }
    }

    pub fn verify<D: Digest, M: AsRef<[u8]>>(&self, pk: BarePublicKey<P>, message: M) -> bool {
        let s_g = &self.s * Point::<P>::basepoint();

        let mut hasher = D::new();
        hasher.update(self.R.to_bytes());
        hasher.update(pk.public.to_bytes());
        hasher.update(message.as_ref());
        let s_bytes = hasher.finalize();
        let a = Scalar::<S>::from_bytes(s_bytes.as_ref());

        let rp = &self.R + a * pk.public;

        s_g == rp
    }
}
