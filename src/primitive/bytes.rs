use generic_array::{ArrayLength, GenericArray};

pub type Output<D> = GenericArray<u8, <D as Bytes>::OutputSize>;

pub trait Bytes: Sized {
    type OutputSize: ArrayLength<u8>;

    fn from_bytes(data: Output<Self>) -> Self;

    fn to_bytes(&self) -> Output<Self>;
}

pub trait FromWideByte: Bytes {
    type FromSize: ArrayLength<u8>;

    fn from_wide_bytes(data: Output<Self>) -> Self;
}

