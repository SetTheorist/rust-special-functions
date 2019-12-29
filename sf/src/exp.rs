use crate::util::{Kahan};
use crate::value::{Embed,Value};
use num::traits::real::{Real};
use num::{Zero};

////////////////////////////////////////////////////////////////////////////////

pub fn sf_exp<V:Value+std::fmt::Debug>(x:V) -> V {
  // positive
  if x.real()<V::RT::zero() { return V::one()/sf_exp(-x); }
  // range-reduce
  let ln2 = V::RT::embed(0.69314718055994530941723212145817656807_f64);
  let n = (x.rabs()/ln2).floor();
  let r = x - V::from_real(n*ln2);
  // sum
  let s = exp__powser(r, V::one());
  s.ldexp(n.dabs() as i32)
}

fn exp__powser<V:Value>(x:V, t0:V) -> V {
  let mut t = V::one();
  let mut s = t0;
  let mut n = 1;
  loop {
    let oldv = s;
    t *= x/V::embed(n);
    s += t;
    if (s-oldv).dabs() <= V::epsilon()*s.dabs() { break; }
    n += 1;
  }
  s
}

/*
pub fn sf_exp_m1(x:f64) -> f64 {
  if x < -0.5 || 0.70 < x { 
    sf_exp(x) - 1.0
  } else {
    exp__powser(x, 0.0)
  }
}
*/

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



