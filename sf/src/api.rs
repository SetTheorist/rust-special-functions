use crate::exp;
use crate::real::*;

pub trait SFExp {
  fn sf_exp(self) -> Self;
}
#[inline] pub fn sf_exp<V:SFExp>(x:V) -> V { x.sf_exp() }

impl SFExp for f64 {
  #[inline] fn sf_exp(self) -> Self { exp::sf_exp(r64(self)).0 }
}
