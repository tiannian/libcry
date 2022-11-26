use crate::{prelude::Curve, ScalarBytes};

/// Seed or code for generate key.
pub struct Code<C: Curve> {
    pub(crate) code: ScalarBytes<C>,
}

impl<C: Curve> Code<C> {
    pub fn new(code: ScalarBytes<C>) -> Self {
        Self { code }
    }
}
