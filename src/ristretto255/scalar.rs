use crate::primitive::scalar::ScalarNumber;
use curve25519_dalek::scalar;
use crate::primitive::bytes::{Bytes, self};
use generic_array::typenum::U32;

pub struct Scalar(pub scalar::Scalar);

impl Bytes for Scalar {
    type OutputSize = U32;

    fn to_bytes(&self) -> bytes::Output<Self> {
        self.0.to_bytes().into()
    }

    fn from_bytes(data: &[u8]) -> Self {

    }

}

impl ScalarNumber for Scalar {

}

