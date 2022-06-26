use sf_impl::complex::*;
use sf_impl::real::*;

use sf_impl::erf;


pub trait SFErf {
  #[must_use] fn sf_erf(self) -> Self;
  #[must_use] fn sf_erfc(self) -> Self;
}

#[must_use]#[inline] pub fn sf_erf<V:SFErf>(x:V) -> V { x.sf_erf() }
#[must_use]#[inline] pub fn sf_erfc<V:SFErf>(x:V) -> V { x.sf_erfc() }

impl SFErf for f64 {
  #[inline] fn sf_erf(self) -> Self { erf::sf_erf(r64(self)).0 }
  #[inline] fn sf_erfc(self) -> Self { erf::sf_erfc(r64(self)).0 }
}


pub trait SFErfInv {
  #[must_use] fn sf_erf_inv(self) -> Self;
}

#[must_use]#[inline] pub fn sf_erf_inv<V:SFErfInv>(x:V) -> V { x.sf_erf_inv() }

impl SFErfInv for f64 {
  #[inline] fn sf_erf_inv(self) -> Self { erf::sf_erf_inv(r64(self)).0 }
}