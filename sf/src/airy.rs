use sf_impl::complex::*;
use sf_impl::real::*;

use sf_impl::airy;

//#[inline] fn f(num_complex::Complex{re,im}:num_complex::Complex<f64>) -> c64 { c64::new(r64(re),r64(im)) }
//#[inline] fn t(c64{re,im}:c64) -> num_complex::Complex<f64> { num_complex::Complex::new(re.0,im.0) }

pub trait SFAiry : Sized {
  #[must_use] fn sf_airy_ai(self) -> Self;
  #[must_use] fn sf_airy_bi(self) -> Self;
  #[must_use] fn sf_airy_aibi(self) -> (Self,Self);
}

#[must_use]#[inline] pub fn sf_airy_ai<V:SFAiry>(x:V) -> V { x.sf_airy_ai() }
#[must_use]#[inline] pub fn sf_airy_bi<V:SFAiry>(x:V) -> V { x.sf_airy_bi() }
#[must_use]#[inline] pub fn sf_airy_aibi<V:SFAiry>(x:V) -> (V,V) { x.sf_airy_aibi() }

impl SFAiry for f64 {
  #[inline] fn sf_airy_ai(self) -> Self { airy::sf_airy_ai(r64(self)).0 }
  #[inline] fn sf_airy_bi(self) -> Self { airy::sf_airy_bi(r64(self)).0 }
  #[inline] fn sf_airy_aibi(self) -> (Self,Self) { let (ai,bi) = airy::sf_airy_aibi(r64(self)); (ai.0, bi.0) }
}