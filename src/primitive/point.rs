//! Define Point.

use super::scalar::{Scalar, ScalarNumber};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::cmp::{PartialEq, Eq};
use super::bytes::{Bytes, self};

/// Point trait.
pub trait DisLogPoint: Clone + Bytes {
    const SIZE: usize;

    type Scalar: ScalarNumber;

    fn zero() -> Self;

    fn one() -> Self;

    fn basepoint() -> Self;

    fn add(&self, rhs: &Self) -> Self;

    fn mul(&self, rhs: &Self::Scalar) -> Self;

    fn neg(&self) -> Self;

    fn eq(&self, o: &Self) -> bool;

    fn sub(&self, rhs: &Self) -> Self
    where
        Self: Sized,
    {
        self.add(&rhs.neg())
    }
}

/// Point.
#[derive(Clone, Eq)]
pub struct Point<P: DisLogPoint>(pub P);

impl<P: DisLogPoint> Point<P> {
    pub fn zero() -> Self {
        Point(P::zero())
    }

    pub fn one() -> Self {
        Point(P::one())
    }

    pub fn basepoint() -> Self {
        Point(P::basepoint())
    }
}

impl<P: DisLogPoint> Bytes for Point<P> {
    type OutputSize = P::OutputSize;

    fn to_bytes(&self) -> bytes::Output<Self> {
        self.0.to_bytes()
    }

    fn from_bytes(data: &[u8]) -> Self {
        Self(P::from_bytes(data))
    }
}

impl<P: DisLogPoint> PartialEq for Point<P> {
    fn eq(&self, o: &Self) -> bool {
        self.0.eq(&o.0)
    }
}

macro_rules! impl_point {
    ($op:ident, $opf:ident, $t:ty, $lt:ty, $rt:ty) => {
        impl<'a, 'b, P: DisLogPoint> $op<$rt> for $lt {
            type Output = $t;
            fn $opf(self, rhs: $rt) -> $t {
                Point(self.0.$opf(&rhs.0))
            }
        }
    };
}

macro_rules! impl_scalar_point {
    ($op:ident, $opf:ident, $t:ty, $lt:ty, $rt:ty) => {
        impl<'a, 'b, S: ScalarNumber, P: DisLogPoint<Scalar = S>> $op<$rt> for $lt {
            type Output = $t;
            fn $opf(self, rhs: $rt) -> $t {
                Point(self.0.$opf(&rhs.0))
            }
        }
    };
}

impl_point!(Add, add, Point<P>, &'a Point<P>, &'b Point<P>);
impl_point!(Sub, sub, Point<P>, &'a Point<P>, &'b Point<P>);
impl_point!(Add, add, Point<P>, Point<P>, &'b Point<P>);
impl_point!(Sub, sub, Point<P>, Point<P>, &'b Point<P>);
impl_point!(Add, add, Point<P>, &'a Point<P>, Point<P>);
impl_point!(Sub, sub, Point<P>, &'a Point<P>, Point<P>);
impl_point!(Add, add, Point<P>, Point<P>, Point<P>);
impl_point!(Sub, sub, Point<P>, Point<P>, Point<P>);

impl_scalar_point!(Mul, mul, Point<P>, &'a Point<P>, &'b Scalar<S>);
impl_scalar_point!(Mul, mul, Point<P>, Point<P>, &'b Scalar<S>);
impl_scalar_point!(Mul, mul, Point<P>, &'a Point<P>, Scalar<S>);
impl_scalar_point!(Mul, mul, Point<P>, Point<P>, Scalar<S>);

macro_rules! impl_point_assign {
    ($op:ident, $opf:ident, $opf_a:ident, $lt:ty, $rt:ty) => {
        impl<'a, 'b, P: DisLogPoint> $op<$rt> for $lt {
            fn $opf_a(&mut self, rhs: $rt) {
                self.0 = self.0.$opf(&rhs.0)
            }
        }
    };
}

macro_rules! impl_point_scalar_assign {
    ($op:ident, $opf:ident, $opf_a:ident, $lt:ty, $rt:ty) => {
        impl<'a, 'b, P: DisLogPoint<Scalar = S>, S: ScalarNumber> $op<$rt> for $lt {
            fn $opf_a(&mut self, rhs: $rt) {
                self.0 = self.0.$opf(&rhs.0)
            }
        }
    };
}

impl_point_assign!(AddAssign, add, add_assign, Point<P>, &'b Point<P>);
impl_point_assign!(SubAssign, sub, sub_assign, Point<P>, &'b Point<P>);
impl_point_scalar_assign!(MulAssign, mul, mul_assign, Point<P>, &'b Scalar<S>);

impl<P: DisLogPoint> Neg for Point<P> {
    type Output = Point<P>;

    fn neg(self) -> Self {
        Point(self.0.neg())
    }
}
