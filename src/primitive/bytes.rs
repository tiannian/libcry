use generic_array::{GenericArray, ArrayLength};

pub type Output<D> = GenericArray<u8, <D as Bytes>::OutputSize>;

pub trait Bytes: Sized {
    type OutputSize: ArrayLength<u8>;

    fn from_bytes(data: &[u8]) -> Self;

    fn to_bytes(&self) -> Output<Self>;
}
