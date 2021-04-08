//! Define sponge function behavior.

/// Define permutation function.
pub trait Sponge {
    /// Get operatalbe state size.
    fn r(&self) -> usize;

    /// Permute state.
    fn permute(&mut self);

    /// Squeeze data from state.
    fn squeeze(&mut self, bytes: &mut [u8]);

    /// Absorb data into state.
    fn absorb(&mut self, bytes: &[u8]);

    /// Set state all zero.
    fn clear(&mut self);
}
