use sf_impl::complex::*;
use sf_impl::real::*;

use sf_impl::polylog;

pub trait SFDilog {
  #[must_use] fn sf_dilog(self) -> Self;
}

#[must_use]#[inline] pub fn sf_dilog<V:SFDilog>(x:V) -> V { x.sf_dilog() }

impl SFDilog for f64 {
  #[inline] fn sf_dilog(self) -> Self { polylog::sf_dilog(r64(self)).0 }
}