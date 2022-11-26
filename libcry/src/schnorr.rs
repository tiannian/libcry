use crate::prelude::Curve;

/// Schnorr signature
pub struct Signature<C: Curve> {
    pub r: C::Point,
    pub s: C::Scalar,
}

impl<C: Curve> Signature<C> {}
