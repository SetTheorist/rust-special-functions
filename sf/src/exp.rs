use crate::util::{Kahan};
use crate::value::{Value,ι};
use num::complex::{Complex};
use num::traits::real::{Real};
use num::{Zero};

////////////////////////////////////////////////////////////////////////////////

pub trait Exp : Value {
  fn exp(self) -> Self;
  fn exp_m1(self) -> Self { self.exp() - ι(1.0) }
}
pub fn exp<E:Exp>(x:E) -> E { x.exp() }
pub fn exp_m1<E:Exp>(x:E) -> E { x.exp_m1() }

impl Exp for f64 {
  fn exp(self) -> Self { sf_exp_real(self) }
  fn exp_m1(self) -> Self { sf_exp_m1_real(self) }
}

impl Exp for Complex<f64> {
  fn exp(self) -> Self { sf_exp_complex(self) }
  fn exp_m1(self) -> Self { sf_exp_complex(self)-1.0 }
}

trait Ln {
  fn ln(self) -> Self;
  fn ln_1p(self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

fn exp__powser<V:Value>(x:V, t0:V) -> V {
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

pub fn sf_ln(x:f64) -> f64 {
  x.ln()
}

pub fn sf_ln_p1(x:f64) -> f64 {
  if x>0.25 {
    sf_ln(1.0 + x)
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



