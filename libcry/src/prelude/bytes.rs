//! Define serialize and deserialize behavior.

use core::fmt::Debug;

pub trait FromToBytes {
    type Bytes: AsMut<[u8]> + AsRef<[u8]>;

    type Error: Debug;

    fn create_from_bytes(bytes: &Self::Bytes) -> Self;

    fn from_bytes(bytes: &Self::Bytes) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn to_bytes(&self) -> &Self::Bytes;
}
