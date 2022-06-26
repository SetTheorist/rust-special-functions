use crate::traits::*;

pub mod impls;

pub trait Erf {
  fn erf(self) -> Self;
  fn erfc(self) -> Self;
}
pub fn sf_erf<V:Erf>(z:V) -> V { z.erf() }
pub fn sf_erfc<V:Erf>(z:V) -> V { z.erfc() }

pub trait ErfInv {
  fn erf_inv(self) -> Self;
}
pub fn sf_erf_inv<V:ErfInv>(z:V) -> V { z.erf_inv() }
