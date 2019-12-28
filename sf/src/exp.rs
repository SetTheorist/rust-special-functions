use crate::util::{Kahan};

////////////////////////////////////////////////////////////////////////////////

pub fn sf_exp(x:f64) -> f64 {
  // positive
  if x<0.0 { return 1.0/sf_exp(-x); }
  // range-reduce
  let ln2 = 0.69314718055994530941723212145817656807_f64;
  let n = (x.abs()/ln2).floor();
  let r = x - n*ln2;
  // sum
  let s = exp__powser(r, 1.0);
  libm::ldexp(s, n as i32)
}

fn exp__powser(x:f64, t0:f64) -> f64 {
  let mut t = 1.0;
  let mut s = Kahan::new(t0);
  let mut n = 1;
  loop {
    let oldv = s.0;
    t *= x/(n as f64);
    s += t;
    if s.0==oldv { break; }
    n += 1;
  }
  s.0
}

pub fn sf_exp_m1(x:f64) -> f64 {
  if x < -0.5 || 0.70 < x { 
    sf_exp(x) - 1.0
  } else {
    exp__powser(x, 0.0)
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



