use sf_impl::complex::*;
use sf_impl::real::*;

use sf_impl::zeta;

pub trait SFZeta {
  #[must_use] fn sf_zeta(self) -> Self;
  #[must_use] fn sf_zeta_m1(self) -> Self;
}

#[must_use]#[inline] pub fn sf_zeta<V:SFZeta>(x:V) -> V { x.sf_zeta() }
#[must_use]#[inline] pub fn sf_zeta_m1<V:SFZeta>(x:V) -> V { x.sf_zeta_m1() }

impl SFZeta for f64 {
  #[inline] fn sf_zeta(self) -> Self { zeta::sf_zeta(r64(self)).0 }
  #[inline] fn sf_zeta_m1(self) -> Self { zeta::sf_zeta_m1(r64(self)).0 }
}

pub trait SFHurwitzZeta {
  #[must_use] fn sf_hurwitz_zeta(self, a:Self) -> Self;
}

#[must_use]#[inline] pub fn sf_hurwitz_zeta<V:SFHurwitzZeta>(x:V, a:V) -> V { x.sf_hurwitz_zeta(a) }

/*
impl SFHurwitzZeta for f64 {
  #[inline] fn sf_hurwitz_zeta(self, a:Self) -> Self { zeta::sf_hurwitz_zeta(r64(self), r64(a)).0 }
}
*/