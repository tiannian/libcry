#![no_std]
#![feature(concat_idents)]
#![feature(trace_macros)]

//! Cry is a advance cryptography library
//! based on ECC and permutation.
//!
//! ## Primitive for Cry.
//!
//! Cry has two main cryptography primitive:
//! - Elliptic Curve
//! - Sponge Construction.
//!
//! We can use these two primitive to build all the
//! world of cryptography.
//!
//! ### Elliptic Curve
//!
//! Elliptic Curve has two type of data, point and
//! scalar number. These define on $G_p$, $p$ is a
//! prime number, also is number of elements.
//!
//! Point $G$ will persent by uppercase letter.
//! scalar number $a$ will persent by lowercase letter.
//!
//! All support operator:
//! - add two scalar, $a+b$
//! - multiply two scalar, $a*b$ or $ab$
//! - negative scalar, $-a$
//! - inverse scalar, $a^{-1}$
//! - scalar one
//! - scalar zero
//! - add two point, $A+B$
//! - multiply scalar and point, $a*G$ or $aG$
//! - negative point, $-A$
//! - inverse point, $A^{-1}$
//! - point one
//! - point zero
//!
//! ### Sponge Construction
//!
//! Sponge construction is a base of permutation cryptography.
//! It can absorb data, squeeze data, or blender state.
//!
//! We can use this to build hash function and symmetric encryption.

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod dh;
pub mod keccak;
pub mod keypair;
pub mod primitive;
pub mod ristretto255;
pub mod schnorr;
pub mod strobe;
