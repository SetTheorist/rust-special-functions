use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use core::ops::{Shl,ShlAssign,Shr,ShrAssign};

use crate::traits::{*};

#[derive(Debug,Default,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
pub struct r64(pub f64);

impl std::fmt::Display for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "ρ")?;
    self.0.fmt(f)
  }
}

impl From<f64> for r64 { #[inline] fn from(x:f64) -> r64 { r64(x) } }
impl From<isize> for r64 { #[inline] fn from(x:isize) -> r64 { r64(x as f64) } }

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

add_ops!(Add, add; AddAssign, add_assign);
add_ops!(Sub, sub; SubAssign, sub_assign);
add_ops!(Mul, mul; MulAssign, mul_assign);
add_ops!(Div, div; DivAssign, div_assign);
add_ops!(Rem, rem; RemAssign, rem_assign);

impl PartialEq<isize> for r64 {
  fn eq(&self, rhs:&isize) -> bool {
    self.eq(&(ι(*rhs):r64))
  }
}
impl PartialEq<f64> for r64 {
  fn eq(&self, rhs:&f64) -> bool {
    self.eq(&(ι(*rhs):r64))
  }
}

impl Neg for r64 {
  type Output = r64;
  #[inline]
  fn neg(self) -> r64 { r64(-self.0) }
}

// TODO: ldexp style implementations
impl Shl<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shl(self, n:isize) -> r64 { self * (2.0_f64.powi(n as i32)) }
}
impl ShlAssign<isize> for r64 {
  #[inline]
  fn shl_assign(&mut self, n:isize) { *self *= 2.0_f64.powi(n as i32); }
}
impl Shr<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shr(self, n:isize) -> r64 { self / (2.0_f64.powi(n as i32)) }
}
impl ShrAssign<isize> for r64 {
  #[inline]
  fn shr_assign(&mut self, n:isize) { *self /= 2.0_f64.powi(n as i32); }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! lift1 {
  ($r:ident, $f:ident) => {
    #[inline]
    fn $r(self) -> Self { r64(self.0.$f()) }
  }
}

impl Constants for r64 {
  const E : r64 = r64(2.7182818284590452354);
  const PI : r64 = r64(3.1415926535897932385);
  const FRAC_1_PI : r64 = r64(0.31830988618379067154);
  const FRAC_PI_2 : r64 = r64(1.5707963267948966192);
  const SQRT2PI : r64 = r64(2.5066282746310005024);
  const FRAC_1_SQRT2PI : r64 = r64(0.39894228040143267794);
  const FRAC_1_SQRTPI : r64 = r64(0.56418958354775628695);
  const LOG2 : r64 = r64(0.69314718055994530942);
  const FRAC_1_LOG2 : r64 = r64(1.4426950408889634074);
  const FRAC_LOG2PI_2 : r64 = r64(0.91893853320467274178);
}

impl Base for r64 { }
impl Zero for r64 { const zero : r64 = r64(0.0); }
impl Addition for r64 { }
impl Subtraction for r64 { }
impl Additive for r64 { }
impl One for r64 { const one : r64 = r64(1.0); }
impl Multiplication for r64 { }
impl Division for r64 { }
impl Multiplicative for r64 { }
impl Embeds<isize> for r64 { }
impl Embeds<f64> for r64 { }
impl Field for r64 { }
impl Ordered for r64 {
  lift1!(floor, floor);
  lift1!(ceil, ceil);
  lift1!(round, round);
  lift1!(trunc, trunc);
  #[inline]
  fn rint(self) -> isize { self.0.round() as isize }
}
impl Normed for r64 {
  type NT = Self;
  const epsilon : Self = r64(f64::EPSILON);
  lift1!(abs, abs);
  lift1!(signum, signum);
  #[inline]
  fn fabs(self) -> f64 { self.abs().0 }

  fn mu(self) -> Self { self.abs() }
  const mu_epsilon : Self = Self::epsilon;
}
impl RealType for r64 { }
impl Bounded for r64 {
  const MIN_VALUE : r64 = r64(f64::MIN);
  const MAX_VALUE : r64 = r64(f64::MAX);
}
impl Roots for r64 {
  lift1!(sqrt, sqrt);
  lift1!(cbrt, cbrt);
  #[inline]
  fn nth_root(self, n:isize) -> Self {
    r64(self.0.powf(1.0/(n as f64)))
  }
}
impl Value for r64 { }

impl Power for r64 {
  fn pow(self, p:r64) -> r64 {
    r64(self.0.powf(p.0))
  }
}

////////////////////////////////////////////////////////////////////////////////

use crate::exp::{Exp};
impl Exp for r64 {
  lift1!(exp, exp);
}

use crate::log::{Log};
impl Log for r64 {
  lift1!(log, ln);
}

/*

pub fn exp_cf(x:r64) -> r64 {
  let terms = (1..).map(|n| if n%2==0{ (x,ι(2)) }else{ (-x,ι(n)) });
  1.0 / contfrac(ι(1), terms, 1e-16)
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
*/
