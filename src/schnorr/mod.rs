use crate::primitive::point::{Point, DisLogPoint};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use crate::keypair::{Keypair, BarePublicKey};
use digest::Digest;
use crate::primitive::bytes::Bytes;

#[allow(non_snake_case)]
pub struct Signature<P: DisLogPoint, S: ScalarNumber> {
    pub R: Point<P>,
    pub s: Scalar<S>,
}

impl<P: DisLogPoint, S: ScalarNumber> Signature<P, S> {
    #[allow(non_snake_case)]
    pub fn sign<D: Digest, M: AsRef<[u8]>>(sk: Keypair<P, S>, message: M) -> Signature<P, S> {
        let mut hasher_r = D::new();
        hasher_r.update(sk.code);
        hasher_r.update(message.as_ref());
        let r = hasher_r.finalize();
        let r_scalar = Scalar::<S>::from_bytes(r.as_ref());
        let R = Point::basepoint() * &r_scalar;
        
        let mut hasher_s = D::new();
        hasher_s.update(message.as_ref());
        hasher_s.update(sk.public.to_bytes());
        hasher_s.update(R.to_bytes());
        let s_bytes = hasher_s.finalize();
        let a = Scalar::from_bytes(s_bytes.as_ref());

        let s = r_scalar + a * sk.secret;

        Signature {
            R, s
        }
    }
    
    pub fn verify(&self, pk: BarePublicKey<P>) -> bool {

        true
    }
}

