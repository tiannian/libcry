use curve25519_dalek::ristretto;
use curve25519_dalek::traits::Identity;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use crate::primitive::point::DisLogPoint;
use crate::primitive::bytes::{Bytes, self};
use generic_array::typenum::U32;
use super::scalar::Scalar;

#[derive(Clone)]
pub struct RistrettoPoint(pub ristretto::RistrettoPoint);

impl Bytes for RistrettoPoint {
    type OutputSize = U32;

    fn from_bytes(data: bytes::Output<Self>) -> Self {
        let point = ristretto::CompressedRistretto::from_slice(&data);
        let de_point = point.decompress().unwrap();
        Self(de_point)
    }

    fn to_bytes(&self) -> bytes::Output<Self> {
        let p = self.0.compress();
        p.to_bytes().into()
    }
}

impl DisLogPoint for RistrettoPoint {
    const SIZE: usize = 32;

    type Scalar = Scalar;

    fn zero() -> Self {
        RistrettoPoint(ristretto::RistrettoPoint::identity())
    }

    fn one() -> Self {
        RistrettoPoint(ristretto::RistrettoPoint::identity())
    }

    fn basepoint() -> Self {
        RistrettoPoint(RISTRETTO_BASEPOINT_POINT)
    }

    fn add(&self, rhs: &Self) -> Self {
        RistrettoPoint(self.0 + rhs.0)
    }

    fn mul(&self, rhs: &Self::Scalar) -> Self {
        RistrettoPoint(self.0 * rhs.0)
    }

    fn neg(&self) -> Self {
        RistrettoPoint(- self.0)
    }

    fn eq(&self, o: &Self) -> bool {
        self.0 == o.0
    }
}

