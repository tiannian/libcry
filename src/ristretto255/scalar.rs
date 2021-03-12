use crate::primitive::bytes::{self, Bytes, FromWideByte};
use crate::primitive::scalar::ScalarNumber;
use curve25519_dalek::scalar;
use generic_array::typenum::{U32, U64};

#[derive(Debug, Clone)]
pub struct Scalar(pub scalar::Scalar);

impl Bytes for Scalar {
    type OutputSize = U32;

    fn to_bytes(&self) -> bytes::Output<Self> {
        self.0.to_bytes().into()
    }

    fn from_bytes(data: bytes::Output<Self>) -> Self {
        Self(scalar::Scalar::from_bytes_mod_order(data.into()))
    }
}

impl FromWideByte for Scalar {
    type FromSize = U64;

    fn from_wide_bytes(data: bytes::Output<Self>) -> Self {
        let d: [u8; 64] = data.into();
        Self(scalar::Scalar::from_bytes_mod_order_wide(data.as_slice()))
    }
}

impl ScalarNumber for Scalar {
    const SIZE: usize = 32;

    fn zero() -> Self {
        Self(scalar::Scalar::zero())
    }

    fn one() -> Self {
        Self(scalar::Scalar::one())
    }

    fn invert(&self) -> Self {
        Self(self.0.invert())
    }

    fn reduce(&self) -> Self {
        Self(self.0.reduce())
    }

    fn neg(&self) -> Self {
        Self(-self.0)
    }

    fn add(&self, _rhs: &Self) -> Self {
        Self(self.0 + _rhs.0)
    }

    fn mul(&self, _rhs: &Self) -> Self {
        Self(self.0 * _rhs.0)
    }

    fn sub(&self, _rhs: &Self) -> Self {
        Self(self.0 - _rhs.0)
    }
}
