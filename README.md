# libcry

libcry is a advance cryptography library based on ECC and permutation.

## Algorithms and features

### Core

- [X] Curve
  - [X] Basic
  - [ ] Pairing
- [X] Bytes
- [ ] Sponge
- [ ] Polynomial

### Curves

- [X] Ristretto25519: [Ristretto Group](https://ristretto.group/ristretto.html)
- [ ] Spec256k1
- [ ] ED25519
- [ ] Curve25519
- [ ] BLS12

### Permutation

- [X] [Keccak](https://keccak.team/keccak.html)

### Key Exchange

- [X] DH

### Hash

- [ ] SHA3
- [ ] Keccak
- [ ] Shake

### MAC

- [ ] HMAC

### Protocol

- [ ] [Strobe](https://strobe.sourceforge.io)
- [ ] Noise

### APIs

- [X] Keypair and keyderive: [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)

### Secret Share

- [ ] SSS
- [ ] VSS

### Digital Signature

- [X] [Schnorr](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)
- [X] [MuSig](https://eprint.iacr.org/2018/068)
- [ ] ECDSA
- [ ] BLS
- [ ] Threshold signature for Schnorr

### Proof

- [X] [Pederson](https://link.springer.com/content/pdf/10.1007%2F3-540-46766-1_9.pdf#page=3)

