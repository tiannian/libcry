use crate::prelude::{Curve, FromToBytes};

pub type ScalarBytes<T> = <<T as Curve>::Scalar as FromToBytes>::Bytes;
pub type PointBytes<T> = <<T as Curve>::Point as FromToBytes>::Bytes;
