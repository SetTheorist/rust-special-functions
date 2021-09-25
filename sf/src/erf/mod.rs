use crate::traits::*;

pub mod impls;

pub trait Erf {
  fn erf(self) -> Self;
  fn erfc(self) -> Self;

  //fn inv_erf(self) -> Self;
  //fn inv_erfc(self) -> Self;
}
pub fn sf_erf<V:Erf>(z:V) -> V { z.erf() }
pub fn sf_erfc<V:Erf>(z:V) -> V { z.erfc() }
