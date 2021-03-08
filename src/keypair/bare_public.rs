use super::Keypair;
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::ScalarNumber;

pub struct BarePublicKey<P: DisLogPoint> {
    pub public: Point<P>,
}

impl<P: DisLogPoint> BarePublicKey<P> {
    pub fn from_keypair<S: ScalarNumber>(keypair: &Keypair<P, S>) -> Self {
        BarePublicKey {
            public: keypair.public.clone(),
        }
    }
}
