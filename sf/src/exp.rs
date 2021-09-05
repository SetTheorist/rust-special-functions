use crate::embed::*;
use crate::kahan::{Kahan};
use crate::traits::{Float};
use crate::value::{Value};
use num::{Zero};
use num::complex::{Complex};
use num::traits::real::{Real};

////////////////////////////////////////////////////////////////////////////////

pub trait Exp : Value {
  // $e^x = \sum_{k=0}^\infty\frac{x^k}{k!}$
  fn exp(self) -> Self;

  // $e^x-1$
  fn exp_m1(self) -> Self { self.exp() - ι(1.0) }

  // $\frac{e^x-1}{x}$
  fn exp_m1vx(self) -> Self { self.exp_m1()/self }

  // $\sum_{k=0}^n\frac{x^k}{k!}$
  fn expn(self,_n:isize) -> Self { unimplemented!() } // TODO

  // $ e^x - \sum_{k=0}^n\frac{x^k}{k!}$
  fn exp_men(self,_n:isize) -> Self { unimplemented!() } // TODO

  // $ \frac{e^x - \sum_{k=0}^n\frac{x^k}{k!}}{x^n}$
  fn exp_menx(self,_n:isize) -> Self { unimplemented!() } // TODO
}
pub fn exp<E:Exp>(x:E) -> E { x.exp() }
pub fn exp_m1<E:Exp>(x:E) -> E { x.exp_m1() }

////////////////////////////////////////////////////////////////////////////////

pub trait Ln : Value {
  // $\ln(x)$
  fn ln(self) -> Self;

  // $\ln(1+x)$
  fn ln_1p(self) -> Self;

  // $\log_2(x)$
  fn log2(self) -> Self { unimplemented!() } // TODO
  // $\log_b(x)$
  fn logb(self,_b:Self) -> Self { unimplemented!() } // TODO
}

pub fn ln<L:Ln>(x:L) -> L { x.ln() }
pub fn ln_1p<L:Ln>(x:L) -> L { x.ln_1p() }

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

#[inline]
pub fn sum_iter<V:Value,I:Iterator<Item=V>>(mut it:I,eps:V::RT) -> V {
  let mut sum = V::ZERO;
  while let Some(t) = it.next() {
    let old = sum;
    sum += t;
    if (sum - old).rabs() <= sum.rabs()*eps { break; }
  }
  sum
}

pub fn exp__powser2<V:Value>(x:V, t0:V) -> V {
  let terms = (1..).scan(ι(1),|s,n|{*s*=x/ι(n);Some(*s)});
  t0+sum_iter(terms,V::RT::EPSILON)
}

pub fn exp__powserk<V:Value>(x:V, t0:V) -> V {
  let mut t = V::one();
  let mut s = Kahan::new(t0);
  let mut n = 1;
  loop {
    let oldv = s;
    t *= x/ι(n);
    s += t;
    if (s-oldv).0.dabs() <= V::epsilon()*s.0.dabs() { break; }
    n += 1;
  }
  s.0
}
pub fn exp__powser<V:Value>(x:V, t0:V) -> V {
  let mut t = V::one();
  let mut s = t0;
  let mut n = 1;
  loop {
    let oldv = s;
    t *= x/ι(n);
    s += t;
    if (s-oldv).dabs() <= V::epsilon()*s.dabs() { break; }
    n += 1;
  }
  s
}

////////////////////////////////////////////////////////////////////////////////

fn sf_exp_real<V:Value>(x:V) -> V
{
  // positive real-part
  if x.real()<V::RT::zero() { return V::one()/sf_exp_real(-x); }
  // range-reduce
  let ln2 = ι(0.69314718055994530941723212145817656807_f64);
  let n = (x.rabs()/ln2).floor();
  let r = x - ι(n*ln2);
  // sum
  let s = exp__powser(r, V::one());
  s.ldexp(n.dabs() as i32)
}

fn sf_exp_complex<V:Value>(x:V) -> V {
  let er = sf_exp_real(x.real());
  if x.imag()!=V::RT::zero() {
    let eic = x.imag().cos();
    let eis = x.imag().sin();
    V::complex_retract(er*eic, er*eis)
  } else {
    V::complex_retract(er, ι(0.0))
  }
}

pub fn sf_exp_m1_real<V:Value>(x:V) -> V {
  if x.dabs() < 0.70 {
    exp__powser(x, V::zero())
  } else {
    sf_exp_real(x) - V::one()
  }
}

////////////////////////////////////////////////////////////////////////////////

pub fn sf_ln_real(x:f64) -> f64 {
  x.ln()
}

pub fn sf_ln_1p_real(x:f64) -> f64 {
  if x>0.25 {
    sf_ln_real(1.0 + x)
  } else {
    let xx = x/(x+2.0);
    let x2 = xx.powi(2);
    let mut s = Kahan::default();
    let mut t = xx;
    let mut n = 0;
    loop {
      let oldv = s.0;
      s += t;
      if oldv==s.0 { break; }
      n += 1;
      t *= x2 / ((2*n+1) as f64);
    }
    2.0 * s.0
  }
}



