//! Define serialize and deserialize behavior.

use generic_array::{ArrayLength, GenericArray};

pub type Output<D> = GenericArray<u8, <D as Bytes>::OutputSize>;

/// Convert data format between bytes and struct.
pub trait Bytes: Sized {
    type OutputSize: ArrayLength<u8>;

    /// Generate A type from bytes.
    fn from_bytes(data: Output<Self>) -> Self;

    /// Convert A type to bytes.
    fn to_bytes(&self) -> Output<Self>;
}

pub trait FromBytesRef: Sized {
    fn from_bytes_ref(data: &[u8]) -> Option<Self>;
}

