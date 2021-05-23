//! Define Point behavier.
//!
//! - Point operator on finite field $G_p$:
//!   - add two point, $A+B$
//!   - multiply scalar and point, $a*G$ or $aG$
//!   - negative point, $-A$
//!   - inverse point, $A^{-1}$
//!   - point one
//!   - point zero
//!   - basepoint

use super::bytes::{self, Bytes};
use super::scalar::{Scalar, ScalarNumber};
use core::cmp::{Eq, PartialEq};
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Point operator for this curve on finite finite $G_p$
pub trait DisLogPoint: Clone + Bytes + Debug {

    /// Point size in bytes.
    const SIZE: usize;

    /// This point type's associated scalar type.
    ///
    /// Beacuse point can't multiply by point, We need use scalar multiply for point.
    type Scalar: ScalarNumber;

    /// Get a point means zero.
    fn zero() -> Self;

    /// Get a point means one.
    fn one() -> Self;

    /// Get a base point for this curve.
    fn basepoint() -> Self;

    /// Add two point.
    fn add(&self, rhs: &Self) -> Self;

    /// Scalar multiply for point.
    fn mul(&self, rhs: &Self::Scalar) -> Self;

    /// Negative a point.
    fn neg(&self) -> Self;

    /// Two point is equal.
    fn eq(&self, o: &Self) -> bool;

    /// Subtraction two point.
    fn sub(&self, rhs: &Self) -> Self
    where
        Self: Sized,
    {
        self.add(&rhs.neg())
    }
}

/// Point wraper type.
///
/// This type support some operators based on overload.
/// - `A + B`
/// - `A += B`
/// - `A - B`
/// - `A -= B`
/// - `a * B` or `B * a`
/// - `A *= b`
/// - `A == B`
/// - `A != B`
#[derive(Debug, Clone, Eq)]
pub struct Point<P: DisLogPoint>(pub P);

impl<P: DisLogPoint> Point<P> {
    /// Get 0.
    pub fn zero() -> Self {
        Point(P::zero())
    }

    /// Get 1.
    pub fn one() -> Self {
        Point(P::one())
    }

    /// Get basepoint.
    pub fn basepoint() -> Self {
        Point(P::basepoint())
    }
}

impl<P: DisLogPoint> Bytes for Point<P> {
    type OutputSize = P::OutputSize;

    fn to_bytes(&self) -> bytes::Output<Self> {
        self.0.to_bytes()
    }

    fn from_bytes(data: bytes::Output<Self>) -> Self {
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
