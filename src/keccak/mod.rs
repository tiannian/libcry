//! An implementation of the keccak-F[1600,800,400,200].

use crate::primitive::sponge::Sponge;
use byteorder::ByteOrder;
use byteorder::NativeEndian;
use core::convert::TryInto;
use unroll::unroll_for_loops;

macro_rules! keccakF {
    (
        $type: ty,
        $loop: expr,
        $state: expr,
        $RHO: expr,
        $PI: expr,
        $RC: expr
    ) => {{
        for i in 0..$loop {
            let mut array: [$type; 5] = [0; 5];

            // Theta
            for x in 0..5 {
                for y_count in 0..5 {
                    let y = y_count * 5;
                    array[x] ^= $state[x + y];
                }
            }

            for x in 0..5 {
                for y_count in 0..5 {
                    let y = y_count * 5;
                    $state[y + x] ^= array[(x + 4) % 5] ^ array[(x + 1) % 5].rotate_left(1);
                }
            }

            // Rho and pi
            let mut last = $state[1];
            for x in 0..24 {
                array[0] = $state[$PI[x]];
                $state[$PI[x]] = last.rotate_left($RHO[x]);
                last = array[0];
            }

            // Chi
            for y_step in 0..5 {
                let y = y_step * 5;

                for x in 0..5 {
                    array[x] = $state[y + x];
                }

                for x in 0..5 {
                    $state[y + x] = array[x] ^ ((!array[(x + 1) % 5]) & (array[(x + 2) % 5]));
                }
            }

            // Iota
            $state[0] ^= $RC[i];
        }
    }};
}

const PI: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

trait EndianExt {
    fn read_u8(buf: &[u8]) -> u8;

    fn write_u8(buf: &mut [u8], n: u8);
}

impl EndianExt for NativeEndian {
    fn read_u8(buf: &[u8]) -> u8 {
        buf[0]
    }

    fn write_u8(buf: &mut [u8], n: u8) {
        buf[0] = n;
    }
}

macro_rules! keccal_struct {
    (
        $struct_name: ident,
        $read_func_type: ident,
        $write_func_type: ident,
        $keccak_func_type: ident,
        $state_type: ty
    ) => {
        pub struct $struct_name<const R: usize> {
            state: [$state_type; 25],
            pos: usize,
        }

        impl<const R: usize> $struct_name<R> {
            pub fn new() -> Self {
                Self {
                    state: [0; 25],
                    pos: 0,
                }
            }
        }

        impl<const R: usize> Sponge for $struct_name<R> {
            fn position(&self) -> usize {
                self.pos
            }

            fn r(&self) -> usize {
                R * core::mem::size_of::<$state_type>()
            }

            fn n(&self) -> usize {
                25 * core::mem::size_of::<$state_type>()
            }

            fn permute(&mut self) {
                $keccak_func_type(&mut self.state);
            }

            fn absorb(&mut self, data: &[u8], more: bool) {
                let mut buf = [0; R];
                let type_len = core::mem::size_of::<$state_type>();
                for (o, chunk) in buf.iter_mut().zip(data.chunks_exact(type_len)) {
                    *o = NativeEndian::$read_func_type(chunk.try_into().unwrap());
                }
                for (d, i) in self.state[..R].iter_mut().zip(&buf) {
                    *d ^= *i;
                }
            }

            fn squeeze(&mut self, data: &mut [u8], more: bool) {
                for i in 0..R {
                    let p_buf = &mut data[i * 8..];
                    NativeEndian::$write_func_type(p_buf, self.state[i]);
                }
            }

            fn clear(&mut self) {
                self.state = [0; 25];
            }
        }
    };
}

keccal_struct!(KeccakF1600, read_u64, write_u64, keccakf1600, u64);
keccal_struct!(KeccakF800, read_u32, write_u32, keccakf800, u32);
keccal_struct!(KeccakF400, read_u16, write_u16, keccakf400, u16);
keccal_struct!(KeccakF200, read_u8, write_u8, keccakf200, u8);

/// keccak-f[1600]
#[unroll_for_loops]
pub fn keccakf1600(state: &mut [u64; 25]) {
    const RHO: [u32; 24] = [
        1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
    ];
    const RC: [u64; 24] = [
        1u64,
        0x8082u64,
        0x800000000000808au64,
        0x8000000080008000u64,
        0x808bu64,
        0x80000001u64,
        0x8000000080008081u64,
        0x8000000000008009u64,
        0x8au64,
        0x88u64,
        0x80008009u64,
        0x8000000au64,
        0x8000808bu64,
        0x800000000000008bu64,
        0x8000000000008089u64,
        0x8000000000008003u64,
        0x8000000000008002u64,
        0x8000000000000080u64,
        0x800au64,
        0x800000008000000au64,
        0x8000000080008081u64,
        0x8000000000008080u64,
        0x80000001u64,
        0x8000000080008008u64,
    ];
    keccakF!(u64, 24, state, RHO, PI, RC);
}

/// keccak-f[800]
#[unroll_for_loops]
pub fn keccakf800(state: &mut [u32; 25]) {
    const RHO: [u32; 24] = [
        1, 3, 6, 10, 15, 21, 28, 4, 13, 23, 2, 14, 27, 9, 24, 8, 25, 11, 30, 18, 7, 29, 20, 12,
    ];

    const RC: [u32; 22] = [
        0x1u32,
        0x8082u32,
        0x808au32,
        0x80008000u32,
        0x808bu32,
        0x80000001u32,
        0x80008081u32,
        0x8009u32,
        0x8au32,
        0x88u32,
        0x80008009u32,
        0x8000000au32,
        0x8000808bu32,
        0x8bu32,
        0x8089u32,
        0x8003u32,
        0x8002u32,
        0x80u32,
        0x800au32,
        0x8000000au32,
        0x80008081u32,
        0x8080u32,
    ];
    keccakF!(u32, 22, state, RHO, PI, RC);
}

/// keccak-f[400]
#[unroll_for_loops]
pub fn keccakf400(state: &mut [u16; 25]) {
    const RHO: [u32; 24] = [
        1, 3, 6, 10, 15, 5, 12, 4, 13, 7, 2, 14, 11, 9, 8, 8, 9, 11, 14, 2, 7, 13, 4, 12,
    ];

    const RC: [u16; 20] = [
        0x1u16, 0x8082u16, 0x808au16, 0x8000u16, 0x808bu16, 0x1u16, 0x8081u16, 0x8009u16, 0x8au16,
        0x88u16, 0x8009u16, 0xau16, 0x808bu16, 0x8bu16, 0x8089u16, 0x8003u16, 0x8002u16, 0x80u16,
        0x800au16, 0xau16,
    ];
    keccakF!(u16, 20, state, RHO, PI, RC);
}

/// keccak-f[200]
#[unroll_for_loops]
pub fn keccakf200(state: &mut [u8; 25]) {
    const RHO: [u32; 24] = [
        1, 3, 6, 2, 7, 5, 4, 4, 5, 7, 2, 6, 3, 1, 0, 0, 1, 3, 6, 2, 7, 5, 4, 4,
    ];

    const RC: [u8; 18] = [
        0x1u8, 0x82u8, 0x8au8, 0x0u8, 0x8bu8, 0x1u8, 0x81u8, 0x9u8, 0x8au8, 0x88u8, 0x9u8, 0xau8,
        0x8bu8, 0x8bu8, 0x89u8, 0x3u8, 0x2u8, 0x80u8,
    ];

    keccakF!(u8, 18, state, RHO, PI, RC);
}

#[cfg(test)]
mod tests {
    use super::{keccakf1600, keccakf200, keccakf400, keccakf800};
    #[test]
    fn test_keccakf1600() {
        let mut data = [0u64; 25];

        keccakf1600(&mut data);
        assert_eq!(
            data,
            [
                0xF1258F7940E1DDE7,
                0x84D5CCF933C0478A,
                0xD598261EA65AA9EE,
                0xBD1547306F80494D,
                0x8B284E056253D057,
                0xFF97A42D7F8E6FD4,
                0x90FEE5A0A44647C4,
                0x8C5BDA0CD6192E76,
                0xAD30A6F71B19059C,
                0x30935AB7D08FFC64,
                0xEB5AA93F2317D635,
                0xA9A6E6260D712103,
                0x81A57C16DBCF555F,
                0x43B831CD0347C826,
                0x01F22F1A11A5569F,
                0x05E5635A21D9AE61,
                0x64BEFEF28CC970F2,
                0x613670957BC46611,
                0xB87C5A554FD00ECB,
                0x8C3EE88A1CCF32C8,
                0x940C7922AE3A2614,
                0x1841F924A2C509E4,
                0x16F53526E70465C2,
                0x75F644E97F30A13B,
                0xEAF1FF7B5CECA249,
            ]
        );

        // data.permute();
        keccakf1600(&mut data);
        assert_eq!(
            data,
            [
                0x2D5C954DF96ECB3C,
                0x6A332CD07057B56D,
                0x093D8D1270D76B6C,
                0x8A20D9B25569D094,
                0x4F9C4F99E5E7F156,
                0xF957B9A2DA65FB38,
                0x85773DAE1275AF0D,
                0xFAF4F247C3D810F7,
                0x1F1B9EE6F79A8759,
                0xE4FECC0FEE98B425,
                0x68CE61B6B9CE68A1,
                0xDEEA66C4BA8F974F,
                0x33C43D836EAFB1F5,
                0xE00654042719DBD9,
                0x7CF8A9F009831265,
                0xFD5449A6BF174743,
                0x97DDAD33D8994B40,
                0x48EAD5FC5D0BE774,
                0xE3B8C8EE55B7B03C,
                0x91A0226E649E42E9,
                0x900E3129E7BADD7B,
                0x202A9EC5FAA3CCE8,
                0x5B3402464E1C3DB6,
                0x609F4E62A44C1059,
                0x20D06CD26A8FBF5C,
            ]
        );
        // assert_eq!(data.parameter().nbytes, 200);
    }
    #[test]
    fn test_keccakf800() {
        let mut data = [0u32; 25];

        keccakf800(&mut data);
        assert_eq!(
            data,
            [
                0xE531D45D, 0xF404C6FB, 0x23A0BF99, 0xF1F8452F, 0x51FFD042, 0xE539F578, 0xF00B80A7,
                0xAF973664, 0xBF5AF34C, 0x227A2424, 0x88172715, 0x9F685884, 0xB15CD054, 0x1BF4FC0E,
                0x6166FA91, 0x1A9E599A, 0xA3970A1F, 0xAB659687, 0xAFAB8D68, 0xE74B1015, 0x34001A98,
                0x4119EFF3, 0x930A0E76, 0x87B28070, 0x11EFE996,
            ]
        );

        // data.permute();

        keccakf800(&mut data);
        assert_eq!(
            data,
            [
                0x75BF2D0D, 0x9B610E89, 0xC826AF40, 0x64CD84AB, 0xF905BDD6, 0xBC832835, 0x5F8001B9,
                0x15662CCE, 0x8E38C95E, 0x701FE543, 0x1B544380, 0x89ACDEFF, 0x51EDB5DE, 0x0E9702D9,
                0x6C19AA16, 0xA2913EEE, 0x60754E9A, 0x9819063C, 0xF4709254, 0xD09F9084, 0x772DA259,
                0x1DB35DF7, 0x5AA60162, 0x358825D5, 0xB3783BAB,
            ]
        );
        // assert_eq!(data.parameter().nbytes, 100);
    }
    #[test]
    fn test_keccakf400() {
        let mut data = [0u16; 25];

        keccakf400(&mut data);
        assert_eq!(
            data,
            [
                0x09F5, 0x40AC, 0x0FA9, 0x14F5, 0xE89F, 0xECA0, 0x5BD1, 0x7870, 0xEFF0, 0xBF8F,
                0x0337, 0x6052, 0xDC75, 0x0EC9, 0xE776, 0x5246, 0x59A1, 0x5D81, 0x6D95, 0x6E14,
                0x633E, 0x58EE, 0x71FF, 0x714C, 0xB38E,
            ]
        );

        // data.permute();

        keccakf400(&mut data);
        assert_eq!(
            data,
            [
                0xE537, 0xD5D6, 0xDBE7, 0xAAF3, 0x9BC7, 0xCA7D, 0x86B2, 0xFDEC, 0x692C, 0x4E5B,
                0x67B1, 0x15AD, 0xA7F7, 0xA66F, 0x67FF, 0x3F8A, 0x2F99, 0xE2C2, 0x656B, 0x5F31,
                0x5BA6, 0xCA29, 0xC224, 0xB85C, 0x097C,
            ]
        );
        // assert_eq!(data.parameter().nbytes, 50);
    }
    #[test]
    fn test_keccakf200() {
        let mut data = [0u8; 25];

        keccakf200(&mut data);
        assert_eq!(
            data,
            [
                0x3C, 0x28, 0x26, 0x84, 0x1C, 0xB3, 0x5C, 0x17, 0x1E, 0xAA, 0xE9, 0xB8, 0x11, 0x13,
                0x4C, 0xEA, 0xA3, 0x85, 0x2C, 0x69, 0xD2, 0xC5, 0xAB, 0xAF, 0xEA,
            ]
        );

        // data.permute();

        keccakf200(&mut data);
        assert_eq!(
            data,
            [
                0x1B, 0xEF, 0x68, 0x94, 0x92, 0xA8, 0xA5, 0x43, 0xA5, 0x99, 0x9F, 0xDB, 0x83, 0x4E,
                0x31, 0x66, 0xA1, 0x4B, 0xE8, 0x27, 0xD9, 0x50, 0x40, 0x47, 0x9E,
            ]
        );
        // assert_eq!(data.parameter().nbytes, 25);
    }
}
