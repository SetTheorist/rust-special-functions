use sf_impl::complex::*;
use sf_impl::real::*;

use sf_impl::gamma;

pub trait SFGamma {
  #[must_use] fn sf_gamma(self) -> Self;
  #[must_use] fn sf_lngamma(self) -> Self;
  #[must_use] fn sf_digamma(self) -> Self;
  #[must_use] fn sf_beta(self, b:Self) -> Self;
}

#[must_use]#[inline] pub fn sf_gamma<V:SFGamma>(x:V) -> V { x.sf_gamma() }
#[must_use]#[inline] pub fn sf_lngamma<V:SFGamma>(x:V) -> V { x.sf_lngamma() }
#[must_use]#[inline] pub fn sf_digamma<V:SFGamma>(x:V) -> V { x.sf_digamma() }
#[must_use]#[inline] pub fn sf_beta<V:SFGamma>(x:V, b:V) -> V { x.sf_beta(b) }

impl SFGamma for f64 {
  #[inline] fn sf_gamma(self) -> Self { gamma::sf_gamma(r64(self)).0 }
  #[inline] fn sf_lngamma(self) -> Self { gamma::sf_lngamma(r64(self)).0 }
  #[inline] fn sf_digamma(self) -> Self { gamma::sf_digamma(r64(self)).0 }
  #[inline] fn sf_beta(self, b:Self) -> Self { gamma::sf_beta(r64(self), r64(b)).0 }
}