use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};

use crate::embed::*;

#[derive(Debug,Default,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
pub struct r64(pub f64);

impl Embed<f64> for r64 { fn embed(x:f64) -> r64 { r64(x) } }
impl Embed<isize> for r64 { fn embed(x:isize) -> r64 { r64(x as f64) } }

////////////////////////////////////////////////////////////////////////////////

macro_rules! add_ops {
  ($opt:ident, $op:ident; $opassignt:ident, $opassign:ident) => {
    impl $opt<r64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b:r64) -> r64 { r64(self.0.$op(b.0)) }
    }
    impl $opt<f64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b:f64) -> r64 { r64(self.0.$op(b)) }
    }
    impl $opt<r64> for f64 {
      type Output = r64;
      #[inline]
      fn $op(self, b:r64) -> r64 { r64(self.$op(b.0)) }
    }
    impl $opt<isize> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b:isize) -> r64 { r64(self.0.$op(b as f64)) }
    }
    impl $opt<r64> for isize {
      type Output = r64;
      #[inline]
      fn $op(self, b:r64) -> r64 { r64((self as f64).$op(b.0)) }
    }

    impl $opassignt<r64> for r64 {
      #[inline]
      fn $opassign(&mut self, b:r64) { *self = self.$op(b); }
    }
    impl $opassignt<f64> for r64 {
      #[inline]
      fn $opassign(&mut self, b:f64) { *self = self.$op(b); }
    }
    impl $opassignt<isize> for r64 {
      #[inline]
      fn $opassign(&mut self, b:isize) { *self = self.$op(b); }
    }
  }
}

impl Neg for r64 {
  type Output = r64;
  #[inline]
  fn neg(self) -> r64 { r64(-self.0) }
}

add_ops!(Add, add; AddAssign, add_assign);
add_ops!(Sub, sub; SubAssign, sub_assign);
add_ops!(Mul, mul; MulAssign, mul_assign);
add_ops!(Div, div; DivAssign, div_assign);
add_ops!(Rem, rem; RemAssign, rem_assign);

fn abs(x:r64) -> r64 { r64(x.0.abs()) }

pub fn eps(x:r64) -> r64 {
  let mut t : r64 = ι(1);
  let mut s = r64(1.0);
  for n in 1..1000 {
    let oldv = s;
    t *= x/n;
    s += t;
    if abs(s-oldv) <= 1e-16*abs(s) { break; }
  }
  s
}

#[inline]
pub fn sumit<I:Iterator<Item=r64>>(mut it:I,eps:f64) -> r64 {
  let mut sum = r64(0.0);
  for t in it {
    let old = sum;
    sum += t;
    if abs(sum - old) <= abs(sum)*eps { break; }
  }
  sum
}

// given the sequence (ai,bi) evaluates the continued fraction
// b0 + a1/(b1 + a2/(b2 + a3/(b3 + ...)))
#[inline]
pub fn contfrac<I:Iterator<Item=(r64,r64)>>(mut it:I, eps:f64) -> r64 {
  let zeta = ι(eps*eps);
  let (_a0,b0) = it.next().unwrap();
  let mut fj = b0; if b0==ι(0) {fj=zeta;}
  let mut cj = fj;
  let mut dj = ι(0);
  let mut n = 1;
  for (aj,bj) in it {
    dj = bj + aj*dj; if dj==ι(0) {dj=zeta;}
    cj = bj + aj/cj; if cj==ι(0) {cj=zeta;}
    dj = 1 / dj;
    let deltaj = cj * dj;
    fj *= deltaj;
    if (deltaj - 1).0.abs() < eps {print!("~{}~",n);break;}
    n += 1;
  }
  fj
}

// ln(1+x) for |arg(1+x)|<pi
pub fn ln_1p_cf(x:r64) -> r64 {
  let terms = (1..).map(|n|(x*(n/2)*(n/2),ι(n)));
  x / contfrac(terms, 1e-16)
}

pub fn exp_cf(x:r64) -> r64 {
  let terms = (0..).map(|n| if n%2==0{ (x,ι(2)) }else{ (-x,ι(n)) });
  1.0 / (-1.0 + contfrac(terms, 1e-16))
}
pub fn exp_cf2(x:r64) -> r64 {
  let terms = (1..).map(|n| if n%2==0{ (x,ι(2)) }else{ (-x,ι(n)) });
  let terms = std::iter::once((ι(0),ι(1))).chain(terms);
  1.0 / contfrac(terms, 1e-16)
}

pub fn eps2(x:r64) -> r64 {
  let terms = (1..).scan(r64(1.0),|s,n|{*s*=x/n;Some(*s)});
  1+sumit(terms,1e-16)
}

pub fn dss(x:r64) -> r64 {
  let mut res = x;
  let mut term = x;
  for n in 1..1000 {
    let old = res;
    term *= x*x / n;
    res += term / (2*n+1);
    if res==old { print!("[{}]",n);break; }
  }
  res * eps(-x*x)
}

pub fn erf_ss(x:r64) -> r64 {
  let tqp = r64(1.1283791670955125738961589031215451716881012586579977136881714434); // 2/sqrt(pi)
  let terms = (1..1000).scan(x,|s,n|{*s*=2*x*x/(2*n+1);Some(*s)});
  (x+sumit(terms,1e-16)) * eps2(-x*x) * tqp
}
