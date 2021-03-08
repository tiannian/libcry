use crate::primitive::point::{Point, DisLogPoint};
use crate::primitive::scalar::{Scalar, ScalarNumber};
use crate::primitive::bytes::{Output, self};
use super::Keypair;

pub struct PublicKey<P: DisLogPoint, S: ScalarNumber> {
    pub(crate) code: Output<S>,
    pub(crate) public: Point<P>,
}

impl<P: DisLogPoint, S: ScalarNumber> PublicKey<P, S> {
    pub fn from_keypair(keypair: &Keypair<P, S>) -> Self {
        PublicKey {
            code: keypair.code.clone(),
            public: keypair.public.clone(),
        }
    }
}

