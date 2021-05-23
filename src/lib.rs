#![no_std]

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
//! Point on EC will persent by uppercase letter, $G$.
//! scalar number will persent by lowercase letter $a$.
//!
//! ### Sponge Construction
//!
//! Sponge construction is a base of permutation cryptography.
//! It can absorb data, squeeze data, or blender state.
//!
//! We can use this to build hash function and symmetric encryption.
//!
//! ## Cryptography toolkit
//!
//! This crate support some algorithm based primitive abstraction.
//! All algorithms list here:
//! - keypair: Addition section for key deriving.
//! - keyderive: Derive child keys for all type of keys.
//! - strobe: A framework for symmetric cryptography and hash function.
//! - DH: Key exchange based on asymmetric cryptography.
//! - Schnorr: Non-interactive short zero-knowledge proof, for digital signature.
//! - MuSig: Signature aggregated based on schnorr, this algorithm from bitcoin community.
//! - Pederson: Pederson commitment.
//! - SSS: Shamir's Secret Sharing based on Lagrange interpolation.
//! - VSS: Verifiable Secret Sharing based on asymmetric.
//! - DKG: Distributed Key Generation.
//! - Threshold: Threshold Schnorr Signature.
//! - VRF: Verifiable Random Function

// For testing
#[cfg(test)]
#[macro_use]
extern crate std;

pub mod dh;
pub mod keccak;
pub mod key;
pub mod primitive;
pub mod ristretto255;
pub mod schnorr;
pub mod strobe;
