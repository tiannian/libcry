use crate::keypair::{BarePublicKey, Keypair};
use crate::primitive::bytes::{self, Bytes};
use crate::primitive::point::DisLogPoint;
use crate::primitive::scalar::ScalarNumber;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedKey<P: DisLogPoint> {
    pub key: bytes::Output<P>,
}

impl<P: DisLogPoint<Scalar = S>, S: ScalarNumber> SharedKey<P> {
    pub fn new(sk: &Keypair<P, S>, pk: &BarePublicKey<P>) -> Self {
        let key = &pk.public * &sk.secret;
        Self {
            key: key.to_bytes(),
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
    fn test_dh() {
        let mut rng = rand::thread_rng();
        let mut seed1 = [0u8; 32];
        rng.fill_bytes(&mut seed1);
        let keypair1 =
            Keypair::<ristretto255::Point, ristretto255::Scalar>::new::<Sha3_512>(seed1.into());

        let mut seed2 = [0u8; 32];
        rng.fill_bytes(&mut seed2);
        let keypair2 =
            Keypair::<ristretto255::Point, ristretto255::Scalar>::new::<Sha3_512>(seed2.into());

        let secret1 = SharedKey::new(&keypair1, &keypair2.to_bare_public());
        let secret2 = SharedKey::new(&keypair2, &keypair1.to_bare_public());

        // println!("{:?}", secret1);
        // println!("{:?}", secret2);

        assert_eq!(secret1, secret2);
    }
}
