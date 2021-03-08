use super::Keypair;
use crate::primitive::bytes;
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::ScalarNumber;

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
}
