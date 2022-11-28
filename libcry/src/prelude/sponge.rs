pub trait Sponge {
    fn absorb(&mut self, block: &[u8]);

    fn apply_f(&mut self);

    fn squeeze(&self) -> &[u8];
}
