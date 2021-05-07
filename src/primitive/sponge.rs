//! Define sponge function behavior.

/// Define permutation function.
pub trait Sponge {
    /// Get operatalbe state size.
    fn r(&self) -> usize;

    /// Get total state size.
    fn n(&self) -> usize;

    /// Get postion of state.
    fn position(&self) -> usize;

    /// Permute state.
    fn permute(&mut self);

    /// Squeeze data from state.
    fn squeeze(&mut self, bytes: &mut [u8], more: bool);

    /// Absorb data into state.
    fn absorb(&mut self, bytes: &[u8], more: bool);

    /// Set state all zero.
    fn clear(&mut self);
}
