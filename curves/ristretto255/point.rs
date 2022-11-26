/* #[derive(Clone, Debug, PartialEq, Eq)] */
/* pub struct Point(pub ristretto::RistrettoPoint); */
/*  */
/* impl Bytes for Point { */
/*     type OutputSize = U32; */
/*  */
/*     fn from_bytes(data: bytes::Output<Self>) -> Self { */
/*         let point = ristretto::CompressedRistretto::from_slice(&data); */
/*         let de_point = point.decompress().unwrap(); */
/*         Self(de_point) */
/*     } */
/*  */
/*     fn to_bytes(&self) -> bytes::Output<Self> { */
/*         let p = self.0.compress(); */
/*         p.to_bytes().into() */
/*     } */
/* } */
/*  */
/* impl DisLogPoint for Point { */
/*     const SIZE: usize = 32; */
/*  */
/*     type Scalar = Scalar; */
/*  */
/*     fn zero() -> Self { */
/*         Self(ristretto::RistrettoPoint::identity()) */
/*     } */
/*  */
/*     fn one() -> Self { */
/*         Self(ristretto::RistrettoPoint::identity()) */
/*     } */
/*  */
/*     fn basepoint() -> Self { */
/*         Self(RISTRETTO_BASEPOINT_POINT) */
/*     } */
/*  */
/*     fn add(&self, rhs: &Self) -> Self { */
/*         Self(self.0 + rhs.0) */
/*     } */
/*  */
/*     fn mul(&self, rhs: &Self::Scalar) -> Self { */
/*         Self(self.0 * rhs.0) */
/*     } */
/*  */
/*     fn neg(&self) -> Self { */
/*         Self(-self.0) */
/*     } */
/*  */
/*     fn eq(&self, o: &Self) -> bool { */
/*         self.0 == o.0 */
/*     } */
/* } */
