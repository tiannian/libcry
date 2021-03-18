//! Define serialize and deserialize behavior.

use generic_array::{ArrayLength, GenericArray};

pub type Output<D> = GenericArray<u8, <D as Bytes>::OutputSize>;

/// Convert data format between bytes and struct.
pub trait Bytes: Sized {
    type OutputSize: ArrayLength<u8>;

    fn from_bytes(data: Output<Self>) -> Self;

    fn to_bytes(&self) -> Output<Self>;
}

pub trait FromBytesRef: Sized {
    fn from_bytes_ref(data: &[u8]) -> Option<Self>;
}

// pub trait FromHash {
//     fn from_hash()
// }
//
