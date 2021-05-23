//! Define scalar behavior.
//!
//! Support operator:
//! - Scalar operator on finite field $G_p$:
//!   - add two scalar, $a+b$
//!   - multiply two scalar, $a*b$ or $ab$
//!   - negative scalar, $-a$
//!   - inverse scalar, $a^{-1}$
//!   - scalar one
//!   - scalar zero
//!
use super::bytes::{self, Bytes, FromBytesRef};
use super::point::{DisLogPoint, Point};
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Scalar operator for this curve on finite finite $G_p$
pub trait ScalarNumber: FromBytesRef + Bytes + Debug + Clone {
    const SIZE: usize;

    /// Get 0.
    fn zero() -> Self;

    /// Get 1.
    fn one() -> Self;

    /// Invert a scalar.
    fn invert(&self) -> Self;

    /// Reduce a scalar by $p$ in $G_p$
    ///
    /// Because scalar's range in bytes always bigger than $p$,
    /// to get a vaild scalar on $G_p$, use `reduce` method.
    fn reduce(&self) -> Self;

    /// Negative a scalar.
    fn neg(&self) -> Self;

    /// Add two scalar.
    fn add(&self, _rhs: &Self) -> Self;

    /// Multiply two scalar.
    fn mul(&self, _rhs: &Self) -> Self;

    /// Subtraction two scalar.
    fn sub(&self, _rhs: &Self) -> Self
    where
        Self: Sized,
    {
        self.add(&_rhs.neg())
    }
}

/// Scalar wraper types.
///
/// This type support some operators based on overload.
/// - `a + b`
/// - `a += b`
/// - `a - b`
/// - `a -= b`
/// - `a * b`
/// - `a *= b`
/// - `a == b`
/// - `a != b`
#[derive(Debug, Clone)]
pub struct Scalar<S: ScalarNumber>(pub S);

impl<S: ScalarNumber> Scalar<S> {
    /// Get 0.
    pub fn zero() -> Self {
        Scalar(S::zero())
    }

    /// Get 1.
    pub fn one() -> Self {
        Scalar(S::one())
    }
}

impl<S: ScalarNumber> Bytes for Scalar<S> {
    type OutputSize = S::OutputSize;

    fn to_bytes(&self) -> bytes::Output<Self> {
        self.0.to_bytes()
    }

    fn from_bytes(data: bytes::Output<Self>) -> Self {
        Self(S::from_bytes(data))
    }
}

impl<S: ScalarNumber> FromBytesRef for Scalar<S> {
    fn from_bytes_ref(data: &[u8]) -> Option<Self> {
        Some(Self(S::from_bytes_ref(data)?))
    }
}

macro_rules! impl_scalar {
    ($op:ident, $opf:ident, $t:ty, $lt:ty, $rt:ty) => {
        impl<'a, 'b, S: ScalarNumber> $op<$rt> for $lt {
            type Output = $t;
            fn $opf(self, rhs: $rt) -> $t {
                Scalar(self.0.$opf(&rhs.0))
            }
        }
    };
}

macro_rules! impl_scalar_point {
    ($op:ident, $opf:ident, $t:ty, $lt:ty, $rt:ty) => {
        impl<'a, 'b, S: ScalarNumber, P: DisLogPoint<Scalar = S>> $op<$rt> for $lt {
            type Output = $t;
            fn $opf(self, rhs: $rt) -> $t {
                Point(rhs.0.$opf(&self.0))
            }
        }
    };
}

impl_scalar!(Add, add, Scalar<S>, &'a Scalar<S>, &'b Scalar<S>);
impl_scalar!(Sub, sub, Scalar<S>, &'a Scalar<S>, &'b Scalar<S>);
impl_scalar!(Mul, mul, Scalar<S>, &'a Scalar<S>, &'b Scalar<S>);
impl_scalar!(Add, add, Scalar<S>, Scalar<S>, &'b Scalar<S>);
impl_scalar!(Sub, sub, Scalar<S>, Scalar<S>, &'b Scalar<S>);
impl_scalar!(Mul, mul, Scalar<S>, Scalar<S>, &'b Scalar<S>);
impl_scalar!(Add, add, Scalar<S>, &'a Scalar<S>, Scalar<S>);
impl_scalar!(Sub, sub, Scalar<S>, &'a Scalar<S>, Scalar<S>);
impl_scalar!(Mul, mul, Scalar<S>, &'a Scalar<S>, Scalar<S>);
impl_scalar!(Add, add, Scalar<S>, Scalar<S>, Scalar<S>);
impl_scalar!(Sub, sub, Scalar<S>, Scalar<S>, Scalar<S>);
impl_scalar!(Mul, mul, Scalar<S>, Scalar<S>, Scalar<S>);

impl_scalar_point!(Mul, mul, Point<P>, &'a Scalar<S>, &'b Point<P>);
impl_scalar_point!(Mul, mul, Point<P>, &'a Scalar<S>, Point<P>);
impl_scalar_point!(Mul, mul, Point<P>, Scalar<S>, &'b Point<P>);
impl_scalar_point!(Mul, mul, Point<P>, Scalar<S>, Point<P>);

macro_rules! impl_scalar_assign {
    ($op:ident, $opf:ident, $opf_a:ident, $lt:ty, $rt:ty) => {
        impl<'a, 'b, S: ScalarNumber> $op<$rt> for $lt {
            fn $opf_a(&mut self, rhs: $rt) {
                self.0 = self.0.$opf(&rhs.0)
            }
        }
    };
}

impl_scalar_assign!(AddAssign, add, add_assign, Scalar<S>, &'b Scalar<S>);
impl_scalar_assign!(SubAssign, sub, sub_assign, Scalar<S>, &'b Scalar<S>);
impl_scalar_assign!(MulAssign, mul, mul_assign, Scalar<S>, &'b Scalar<S>);
impl_scalar_assign!(AddAssign, add, add_assign, Scalar<S>, Scalar<S>);
impl_scalar_assign!(SubAssign, sub, sub_assign, Scalar<S>, Scalar<S>);
impl_scalar_assign!(MulAssign, mul, mul_assign, Scalar<S>, Scalar<S>);

impl<S: ScalarNumber> Neg for Scalar<S> {
    type Output = Scalar<S>;
    fn neg(self) -> Scalar<S> {
        Scalar(self.0.neg())
    }
}
