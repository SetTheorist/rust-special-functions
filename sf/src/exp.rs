use sf_impl::exp;
use sf_impl::complex::*;
use sf_impl::real::*;

#[inline] fn f(num_complex::Complex{re,im}:num_complex::Complex<f64>) -> c64 { c64::new(r64(re),r64(im)) }
#[inline] fn t(c64{re,im}:c64) -> num_complex::Complex<f64> { num_complex::Complex::new(re.0,im.0) }

pub trait SFExp {
  // TODO: does this must_use annotation in a trait actually do anything?
  #[must_use] fn sf_exp(self) -> Self;
  #[must_use] fn sf_exp_m1(self) -> Self;
  #[must_use] fn sf_exp_m1vx(self) -> Self;
  #[must_use] fn sf_expn(self, n:isize) -> Self;
  #[must_use] fn sf_exp_men(self, n:isize) -> Self;
  #[must_use] fn sf_exp_menx(self, n:isize) -> Self;
}

/// compute sf_exp(x) $` {} = \exp(x) = e^x `$ 
/// ```math
/// {} = \sum_{n=0}^\infty \frac{x^n}{n!}
/// ```
#[must_use]#[inline] pub fn sf_exp<V:SFExp>(x:V) -> V { x.sf_exp() }
#[must_use]#[inline] pub fn sf_exp_m1<V:SFExp>(x:V) -> V { x.sf_exp_m1() }
#[must_use]#[inline] pub fn sf_exp_m1vx<V:SFExp>(x:V) -> V { x.sf_exp_m1vx() }
#[must_use]#[inline] pub fn sf_expn<V:SFExp>(n:isize, x:V) -> V { x.sf_expn(n) }
#[must_use]#[inline] pub fn sf_exp_men<V:SFExp>(n:isize, x:V) -> V { x.sf_exp_men(n) }
#[must_use]#[inline] pub fn sf_exp_menx<V:SFExp>(n:isize, x:V) -> V { x.sf_exp_menx(n) }

impl SFExp for f64 {
  #[must_use]#[inline] fn sf_exp(self) -> Self { exp::sf_exp(r64(self)).0 }
  #[must_use]#[inline] fn sf_exp_m1(self) -> Self { exp::sf_exp_m1(r64(self)).0 }
  #[must_use]#[inline] fn sf_exp_m1vx(self) -> Self { exp::sf_exp_m1vx(r64(self)).0 }
  #[must_use]#[inline] fn sf_expn(self, n:isize) -> Self { exp::sf_expn(n, r64(self)).0 }
  #[must_use]#[inline] fn sf_exp_men(self, n:isize) -> Self { exp::sf_exp_men(n, r64(self)).0 }
  #[must_use]#[inline] fn sf_exp_menx(self, n:isize) -> Self { exp::sf_exp_menx(n, r64(self)).0 }
}


impl SFExp for num_complex::Complex<f64> {
  #[must_use]#[inline] fn sf_exp(self) -> Self { t(exp::sf_exp(f(self))) }
  #[must_use]#[inline] fn sf_exp_m1(self) -> Self { t(exp::sf_exp_m1(f(self))) }
  #[must_use]#[inline] fn sf_exp_m1vx(self) -> Self { t(exp::sf_exp_m1vx(f(self))) }
  #[must_use]#[inline] fn sf_expn(self, n:isize) -> Self { t(exp::sf_expn(n, f(self))) }
  #[must_use]#[inline] fn sf_exp_men(self, n:isize) -> Self { t(exp::sf_exp_men(n, f(self))) }
  #[must_use]#[inline] fn sf_exp_menx(self, n:isize) -> Self { t(exp::sf_exp_menx(n, f(self))) }
}
