use crate::value::{Value,RealValue};
use crate::exp::{Exp,sf_exp};
use crate::embed::{ι};

pub trait Dawson {
  fn dawson(self) -> Self;
}

pub fn dawson<V:RealValue>(x:V) -> V {
  unimplemented!()
}

// maybe use for small |z|:
// exp(-z^2)*erf(I*z)*sqrt(pi)/(2*I)

pub fn dawson_contfrac<V:Value>(x:V) -> V {
  let zeta : V = ι(1e-100);
  let eps = V::epsilon();
  let mut fj : V = ι(1);
  let mut cj = fj;
  let mut dj = ι(0);
  for j in 1..1000 {
    let aj : V = x*x*ι((if j%2==0{-1}else{1})*2*j);
    let bj : V = ι(2*j+1);
    dj = bj + aj*dj; if dj==ι(0) {dj=zeta;}
    cj = bj + aj/cj; if cj==ι(0) {cj=zeta;}
    dj = dj.recip();
    let deltaj = cj*dj;
    fj *= deltaj;
    if (deltaj - ι(1)).dabs() < eps { print!("({})",j);break; }
  }
  x / fj
}

pub fn dawson_contfrac2<V:Value>(x:V) -> V {
  let zeta : V = ι(1e-100);
  let eps = V::epsilon();
  let mut fj = x*x*ι(2) + ι(1);
  let mut cj = fj;
  let mut dj = ι(0);
  for j in 1..1000 {
    let aj = x*x*ι(-4*j);
    let bj = x*x*ι(2) + ι(2*j+1);
    dj = bj + aj*dj; if dj==ι(0) {dj=zeta;}
    cj = bj + aj/cj; if cj==ι(0) {cj=zeta;}
    dj = dj.recip();
    let deltaj = cj*dj;
    fj *= deltaj;
    if (deltaj - ι(1)).dabs() < eps { print!("<{}>",j);break; }
  }
  x / fj
}

pub fn dawson_seres<V:Value+Exp>(x:V) -> V {
  let mut res = x;
  let mut term = x;
  for n in 1..1000 {
    let old = res;
    term *= x*x / ι(n);
    res += term / ι(2*n+1);
    if res==old { print!("[{}]",n);break; }
  }
  res * sf_exp(-x*x)
}

// doesn't include suggested optimizations
pub fn dawson_rybicki<V:Value+Exp>(x:V) -> V {
  let h : V = ι(0.1);
  let mut n = 1;
  let mut res : V = num::Zero::zero();
  loop {
    let old = res;
    { let n = ι(n);
      res += (sf_exp(-(x-h*n)*(x-h*n)) - sf_exp(-(x+h*n)*(x+h*n))) / n;
    }
    if res==old { print!("{{{}}}",n);break; }
    n += 2; if n>999 { break; }
  }
  res / ι(1.7724538509055160272981674833411451827975494561223871282138077898) // sqrt(pi)
}

// bessel series (in terms of spherical bessel i1() functions

