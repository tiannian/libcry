pub trait Bytes: Sized {
    const BYTES_LENGTH: usize;

    fn from_bytes(data: [u8; Self::BYTES_LENGTH]) -> Self;

    fn to_bytes(&self) -> [u8; Self::BYTES_LENGTH];
}

pub trait FromBytesWide: Bytes {
    fn from_bytes_wide(data: [u8; <Self as Bytes>::BYTES_LENGTH * 2]) -> Self;
}
