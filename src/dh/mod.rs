use crate::keypair::{BarePublicKey, Keypair};
use crate::primitive::point::{DisLogPoint, Point};
use crate::primitive::scalar::ScalarNumber;

pub struct SharedKey<P: DisLogPoint> {
    pub key: Point<P>,
}

impl<P: DisLogPoint> SharedKey<P> {
    pub fn new<S: ScalarNumber>(sk: Keypair<P, S>, pk: BarePublicKey<P>) -> Self {
        let key = pk.public * &sk.secret;
        Self { key }
    }
}
