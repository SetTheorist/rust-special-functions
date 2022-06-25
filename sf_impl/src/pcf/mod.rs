use crate::traits::*;

pub mod impls;
pub mod methods;

// NB Mathematica:
//    ParabolicCylinderD[n,z] = U(-n-1/2, z)
//    ParabolicCylinderD[-a-1/2,z] = U(a, z)
//    for n=0,1,2,...:
//      V(n+1/2,z) = \sqrt(2/π)*exp(z^2/4)*(-i)^n*2^(-n/2)*H_n(iz/\sqrt(2)),
//        H_n(z) = Hermite polynomials
// 
// TODO: maybe remove separate A generic
// (unless there are unique implementation strategies for A integral, for example)
pub trait PCF<A> : Sized {
  fn u(self, a:A) -> Self;
  fn v(self, a:A) -> Self;
  fn uv(self, a:A) -> (Self,Self);

  // D_n(z) = U(-n-1/2,z)
  fn d(self, a:A) -> Self;
}
#[inline] pub fn sf_pcf_u<A,V:PCF<A>>(a:A, z:V) -> V { z.u(a) }
#[inline] pub fn sf_pcf_v<A,V:PCF<A>>(a:A, z:V) -> V { z.v(a) }
#[inline] pub fn sf_pcf_uv<A,V:PCF<A>>(a:A, z:V) -> (V,V) { z.uv(a) }
#[inline] pub fn sf_pcf_d<A,V:PCF<A>>(a:A, z:V) -> V { z.d(a) }

