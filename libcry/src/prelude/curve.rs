use core::ops::Neg;

use num_traits::{Inv, One, Zero};

use super::FromToBytes;

pub trait Curve {
    type Scalar: Zero + One + Inv + Neg + Scalar + FromToBytes + Clone;

    type Point: Zero + One + Neg + Point<Scalar = Self::Scalar> + FromToBytes + Clone;
}

pub trait Scalar {
    fn add(&mut self, o: &Self);

    fn mul(&mut self, o: &Self);
}

pub trait Point {
    type Scalar: Scalar;

    fn add(&mut self, o: &Self);

    fn mul(&mut self, o: &Self::Scalar);

    fn basepoint() -> Self;
}
