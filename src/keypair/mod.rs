mod keypair;
pub use keypair::Keypair;

mod public_key;
pub use public_key::PublicKey;

// impl<P: DisLogPoint, S: ScalarNumber> Keypair<P, S> {
//     pub fn new<D: Digest>(seed: Scalar<S>) -> Self {
//         let hasher = D::new();
//
//     }
// }

