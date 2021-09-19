use crate::traits::*;
//use crate::kahan::{Kahan};
//use crate::algorithm::{sum_series,sum_series_};

////////////////////////////////////////////////////////////////////////////////

pub trait Exp: Value {
  // $e^x = \sum_{k=0}^\infty\frac{x^k}{k!}$
  fn exp(self) -> Self;

  // $e^x-1$
  fn exp_m1(self) -> Self { self.exp() - ι(1): Self }

  // $\frac{e^x-1}{x}$
  fn exp_m1vx(self) -> Self { self.exp_m1() / self }

  // $\sum_{k=0}^n\frac{x^k}{k!}$
  fn expn(self, _n: isize) -> Self { unimplemented!() } // TODO

  // $ e^x - \sum_{k=0}^n\frac{x^k}{k!}$
  fn exp_men(self, _n: isize) -> Self { unimplemented!() } // TODO

  // $ \frac{e^x - \sum_{k=0}^n\frac{x^k}{k!}}{x^n}$
  fn exp_menx(self, _n: isize) -> Self { unimplemented!() } // TODO
}

#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_exp<V: Exp>(x: V) -> V { x.exp() }

////////////////////////////////////////////////////////////////////////////////

pub mod impls {
  use crate::algorithm::{contfrac_modlentz, sum_series, sum_series_};
  use crate::traits::*;

  #[inline]
  pub fn exp_power_series_terms<V: Value>(x: V) -> impl Iterator<Item = V> {
    (1..).scan(ι(1): V, move |s, n| {
      let o = *s;
      *s *= x / n;
      Some(o)
    })
  }

  #[inline]
  pub fn exp_power_series<V: Value>(x: V, n0: usize) -> V {
    let terms = exp_power_series_terms(x).skip(n0);
    sum_series(terms, V::mu_epsilon)
  }

  #[inline]
  pub fn exp_power_series_<V: Value>(x: V, n0: usize) -> V {
    let terms = exp_power_series_terms(x).skip(n0);
    sum_series_(terms, V::mu_epsilon)
  }

  #[inline]
  pub fn exp_continued_fraction<V: Value>(x: V) -> V {
    let terms = (1..).map(|n| if n % 2 == 0 { (x, ι(2)) } else { (-x, ι(n)) });
    contfrac_modlentz(ι(1), terms, V::mu_epsilon).recip()
  }

  #[inline]
  pub fn range_reduce_ln2<V: RealValue + Ordered>(x: V) -> (V, isize) {
    // range-reduce
    let ln2: V = ι(0.69314718055994530941723212145817656807_f64); // TODO: use constants
    let n: isize = (x.abs() / ln2).floor().rint();
    let r: V = x - ln2 * n;
    (r, n)
  }
}

/*
use crate::embed::*;
use crate::value::{Value,RealValue,ComplexValue};
use crate::trig::{Trig};
use num::{Zero};
use num::complex::{Complex};

////////////////////////////////////////////////////////////////////////////////

#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_exp_m1<V:Exp>(x:V) -> V { x.exp_m1() }
#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_exp_m1vx<V:Exp>(x:V) -> V { x.exp_m1vx() }
#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_expn<V:Exp>(x:V, n:isize) -> V { x.expn(n) }
#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_exp_men<V:Exp>(x:V, n:isize) -> V { x.exp_men(n) }
#[must_use = "method returns a new number and does not mutate the original value"]
pub fn sf_exp_menx<V:Exp>(x:V, n:isize) -> V { x.exp_menx(n) }

////////////////////////////////////////////////////////////////////////////////

impl Exp for f64 {
  fn exp(self) -> Self { sf_exp_real(self) }
  fn exp_m1(self) -> Self { sf_exp_m1_real(self) }
}

impl Exp for Complex<f64> {
  fn exp(self) -> Self { sf_exp_complex(self) }
  fn exp_m1(self) -> Self { sf_exp_complex(self)-1.0 }
}

impl Ln for f64 {
  fn ln(self) -> Self { sf_ln_real(self) }
  fn ln_1p(self) -> Self { sf_ln_1p_real(self) }
}

////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////

fn sf_exp_real<V:RealValue>(x:V) -> V
{
  // positive real-part
  if x<V::zero() { return V::one()/sf_exp_real(-x); }
  // range-reduce
  let ln2 = ι(0.69314718055994530941723212145817656807_f64);
  let n = (x.vabs()/ln2).floor();
  let r = x - ι(n*ln2);
  // sum
  let s = exp__powser(r, V::one());
  s.ldexp(n.dabs() as i32)
}

fn sf_exp_complex<V:ComplexValue>(x:V) -> V where V::RT:Trig {
  let er = sf_exp_real(x.real());
  if x.imag()!=V::RT::zero() {
    let (eic,eis) = x.imag().cos_sin();
    V::make_complex(er*eic, er*eis)
  } else {
    ι(er)
  }
}

pub fn sf_exp_m1_real<V:RealValue>(x:V) -> V {
  if x.dabs() < 0.70 {
    exp__powser(x, V::zero())
  } else {
    sf_exp_real(x) - V::one()
  }
}

*/
////////////////////////////////////////////////////////////////////////////////
