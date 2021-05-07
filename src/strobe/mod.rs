use crate::primitive::sponge::Sponge;

const OP_A: u8 = 1 << 0;
const OP_C: u8 = 1 << 1;
const OP_T: u8 = 1 << 2;
const OP_M: u8 = 1 << 3;
const OP_K: u8 = 1 << 4;

pub struct Strobe<S: Sponge> {
    pos: usize,
    pos_begin: usize,
    i0: Option<u8>,
    r: usize,
    initialized: bool,
    state: S,
}

impl<S: Sponge> Strobe<S> {
    pub fn new(mut state: S, proto: &[u8], security: usize) -> Self {
        let r = state.n() - security / 4;
        let pad_begin = [0x01, (r as u8), 0x01, 0x00, 0x01, 0x60];
        state.absorb(&pad_begin, true);
        state.absorb(b"STROBEv1.0.2", false);
        state.permute();
        Self {
            pos: 0,
            pos_begin: 0,
            i0: None,
            r,
            initialized: true,
            state,
        }
    }
}
