use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use core::cmp::{PartialEq,PartialOrd};

use crate::traits::*;
use crate::real::*;

#[derive(Debug, Default, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct err64(pub f64,pub f64);

// we only compare the value part, not the error
impl PartialEq<err64> for err64 {
  fn eq(&self, b:&err64) -> bool {
    self.0.eq(&b.0)
  }
}

impl PartialEq<isize> for err64 {
  fn eq(&self, rhs: &isize) -> bool { self.eq(&(ι(*rhs):err64)) }
}

impl PartialEq<f64> for err64 {
  fn eq(&self, rhs: &f64) -> bool { self.eq(&(ι(*rhs):err64)) }
}

impl PartialEq<r64> for err64 {
  fn eq(&self, rhs: &r64) -> bool { self.eq(&(ι(*rhs):err64)) }
}

// we only compare the value part, not the error
impl PartialOrd<err64> for err64 {
  fn partial_cmp(&self, other: &err64) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl std::fmt::Display for err64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "ρ")?;
    std::fmt::Display::fmt(&self.0, f)?;
    write!(f, "+?")?;
    std::fmt::Display::fmt(&self.1, f)
  }
}

impl const From<f64> for err64 {
  #[inline]
  fn from(x: f64) -> err64 { err64(x,0.0) }
}
impl const From<r64> for err64 {
  #[inline]
  fn from(x: r64) -> err64 { err64(x.0,0.0) }
}
impl const From<isize> for err64 {
  #[inline]
  fn from(x: isize) -> err64 { err64(x as f64, (x - ((x as f64) as isize)) as f64) }
}

// can't make these const, due to failure of mul_add to be const
macro_rules! gen_ops {
  ($x:tt, $opt:ident, $op:ident; $opassignt:ident, $opassign:ident) => {
    #[automatically_derived]
    impl /*const*/ $opt<r64> for err64 {
      type Output = err64;
      #[inline]
      fn $op(self, b: r64) -> err64 { self.$op(err64::from(b)) }
    }
    #[automatically_derived]
    impl /*const*/ $opt<f64> for err64 {
      type Output = err64;
      #[inline]
      fn $op(self, b: f64) -> err64 { self.$op(err64::from(b)) }
    }
    #[automatically_derived]
    impl /*const*/ $opt<isize> for err64 {
      type Output = err64;
      #[inline]
      fn $op(self, b: isize) -> err64 { self.$op(err64::from(b)) }
    }
    #[automatically_derived]
    impl $opassignt<err64> for err64 {
      #[inline]
      fn $opassign(&mut self, b: err64) { *self = self.$op(b); }
    }
    #[automatically_derived]
    impl $opassignt<r64> for err64 {
      #[inline]
      fn $opassign(&mut self, b: r64) { *self = self.$op(b); }
    }
    #[automatically_derived]
    impl $opassignt<f64> for err64 {
      #[inline]
      fn $opassign(&mut self, b: f64) { *self = self.$op(b); }
    }
    #[automatically_derived]
    impl $opassignt<isize> for err64 {
      #[inline]
      fn $opassign(&mut self, b: isize) { *self = self.$op(b); }
    }
  }
}

const fn twosum(x:f64, y:f64) -> (f64,f64) {
  let z = x + y;
  let xp = z - y;
  let yp = z - xp;
  let d = (x - xp) + (y - yp);
  (z, d)
}

impl const Neg for err64 {
  type Output = err64;
  fn neg(self) -> err64 {
    err64(-self.0, -self.1)
  }
}

impl const Add<err64> for err64 {
  type Output = err64;
  fn add(self, b:err64) -> err64 {
    let (z,d) = twosum(self.0, b.0);
    err64(z, self.1+b.1+d)
  }
}
gen_ops!(+, Add, add; AddAssign, add_assign);

impl const Sub<err64> for err64 {
  type Output = err64;
  fn sub(self, b:err64) -> err64 {
    self + (-b)
  }
}
gen_ops!(-, Sub, sub; SubAssign, sub_assign);

// can't make const, as mul_add() isn't const
fn twomult(x:f64, y:f64) -> (f64,f64) {
    let z = x * y;
    let d = x.mul_add(y, -z);
    (z, d)
}

impl Mul<err64> for err64 {
  type Output = err64;
  fn mul(self, b:err64) -> err64 {
    let (z,d) = twomult(self.0, b.0);
    err64(z, self.1*b.0 + self.0*b.1 + d)
  }
}
gen_ops!(*, Mul, mul; MulAssign, mul_assign);

impl Div<err64> for err64 {
  type Output = err64;
  fn div(self, b:err64) -> err64 {
    let z = self.0 / b.0;
    let num = self.1 - b.0.mul_add(z, -self.0);
    let den = b.0 + b.1;
    let d = num / den;
    err64(z, d)
  }
}
gen_ops!(/, Div, div; DivAssign, div_assign);

impl Rem<err64> for err64 {
  type Output = err64;
  fn rem(self, b:err64) -> err64 {
    let z = self.0 % b.0;
    err64(z, 0.0) // TODO TODO TODO
  }
}
gen_ops!(%, Rem, rem; RemAssign, rem_assign);

impl Shl<isize> for err64 {
  type Output = err64;
  #[inline]
  fn shl(self, n: isize) -> err64 { err64(self.0.ldexp(n), self.1.ldexp(n)) }
}
impl ShlAssign<isize> for err64 {
  #[inline]
  fn shl_assign(&mut self, n: isize) { *self = err64(self.0.ldexp(n), self.1.ldexp(n)); }
}
impl Shr<isize> for err64 {
  type Output = err64;
  #[inline]
  fn shr(self, n: isize) -> err64 { err64(self.0.ldexp(-n), self.1.ldexp(-n)) }
}
impl ShrAssign<isize> for err64 {
  #[inline]
  fn shr_assign(&mut self, n: isize) { *self = err64(self.0.ldexp(-n), self.1.ldexp(-n)); }
}

////////////////////////////////////////////////////////////////////////////////

fn sqrt(x:err64) -> err64 {
  let z = x.0.sqrt();
  let num = x.1 + (-z).mul_add(z, x.0);
  let den = z + z;
  let d = num / den;
  err64(z, d)
}

////////////////////////////////////////////////////////////////////////////////

impl Base for err64 {}
impl Zero for err64 { const zero : err64 = err64(0.0, 0.0); }
impl Addition for err64 {}
impl Subtraction for err64 {}
impl Additive for err64 {}
impl One for err64 { const one : err64 = err64(1.0, 0.0); }
impl Multiplication for err64 {}
impl Division for err64 {}
impl Multiplicative for err64 {}
impl Embeds<isize> for err64 {}
impl Embeds<f64> for err64 {}
impl Embeds<r64> for err64 {}
impl Field for err64 {}
impl Ordered for err64 {
  #[inline]
  fn rint(self) -> isize { self.0.round() as isize }
  fn floor(self) -> Self { err64(self.0.floor(), 0.0) } // TODO: check errorbounds..
  fn ceil(self) -> Self { err64(self.0.ceil(), 0.0) } // TODO: check errorbounds..
  fn round(self) -> Self { err64(self.0.round(), 0.0) } // TODO: check errorbounds..
  fn trunc(self) -> Self { err64(self.0.trunc(), 0.0) } // TODO: check errorbounds..
}
impl Normed for err64 {
  type NT = Self;
  const epsilon: Self = err64(f64::EPSILON, 0.0);
  fn abs(self) -> Self { err64(self.0.abs(), self.1.abs()) }
  fn vabs(self) -> Self { err64(self.0.abs(), self.1.abs()) }
  fn signum(self) ->  Self { err64(self.0.signum(), 0.0) } // TODO: check if error band crosses zero?
  #[inline]
  fn fabs(self) -> f64 { self.0.abs() }
  fn mu(self) -> Self { self.abs() }
}
impl Bounded for err64 {
  const MIN_VALUE: err64 = err64(f64::MIN, 0.0);
  const MAX_VALUE: err64 = err64(f64::MAX, 0.0);
}
impl Roots for err64 {
  fn sqrt(self) -> Self { sqrt(self) }
  fn cbrt(self) -> Self { err64(self.0.cbrt(),0.0) } // TODO
  #[inline]
  fn nth_root(self, n: isize) -> Self { err64(self.0.powf(1.0 / (n as f64)), 0.0) } // TODO
}
impl Value for err64 {}

use sf_hex_float::hexf;
macro_rules! Err64 { ($x:tt) => { hexf!(:2:err64:$x) } }

impl Constants for err64 {
  const nan: Self = err64(f64::NAN,f64::NAN);

  // $e^1$
  // 2.7182818284590452353602874713526624977572470937000
  const E: Self = Err64!("2.b7e151628aed2a6abf7158809cf4f3c762e7160f4");

  // $e^{-1}$
  // 0.36787944117144232159552377016146086744581113103177
  const FRAC_1_E: Self = Err64!("0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab");

  // $\pi$
  // 3.1415926535897932384626433832795028841971693993751
  const PI: Self = Err64!("3.243f6a8885a308d313198a2e03707344a40938223");

  // $1/\pi$
  // 0.31830988618379067153776752674502872406891929148091
  const FRAC_1_PI: Self = Err64!("0.517cc1b727220a94fe13abe8fa9a6ee06db14acca");

  // $\pi/2$
  // 1.5707963267948966192313216916397514420985846996876
  const FRAC_PI_2: Self = Err64!("1.921fb54442d18469898cc51701b839a252049c111");

  // $\sqrt(\pi)$
  // 1.7724538509055160272981674833411451827975494561224
  const SQRTPI: Self = Err64!("1.c5bf891b4ef6aa79c3b0520d5db9383fe3921546f");

  // $\sqrt(2\pi)$
  // 2.5066282746310005024157652848110452530069867406099
  const SQRT2PI: Self = Err64!("2.81b263fec4e0b2caf9483f5ce459dc5f19f3ea641");

  // $1/\sqrt(2\pi)$
  // 0.39894228040143267793994605993438186847585863116493
  const FRAC_1_SQRT2PI: Self = Err64!("0.662114cf50d942343f2cf1402eae38bfd3829f305");

  // $1/\sqrt(\pi)$
  // 0.56418958354775628694807945156077258584405062932900
  const FRAC_1_SQRTPI: Self = Err64!("0.906eba8214db688d71d48a7f6bfec3441409a0ebb");

  // $\log(2)$
  // 0.69314718055994530941723212145817656807550013436026
  const LOG2: Self = Err64!("0.b17217f7d1cf79abc9e3b39803f2f6af40f343267");

  // $1/\log(2)$
  // 1.4426950408889634073599246810018921374266459541530
  const FRAC_1_LOG2: Self = Err64!("1.71547652b82fe1777d0ffda0d23a7d11d6aef551c");

  // $\log(2\pi)/2 = \log(\sqrt{2\pi})$
  // 0.91893853320467274178032973640561763986139747363778
  const FRAC_LOG2PI_2: Self = Err64!("0.eb3f8e4325f5a53494bc900144192023cfb08f8d1");

  // Euler's gamma $\gamma$
  // 0.57721566490153286060651209008240243104215933593992
  const EULER_GAMMA: Self = Err64!("0.93c467e37db0c7a4d1be3f810152cb56a1cecc3af");
}

impl Classify for err64 {
  #[inline] fn is_nan(self) -> bool { self.0.is_nan() }
  #[inline] fn is_infinite(self) -> bool { self.0.is_infinite() }
  #[inline] fn is_finite(self) -> bool { self.0.is_finite() }
  #[inline] fn is_zero(self) -> bool { self.0 == 0.0 }
  #[inline] fn is_negzero(self) -> bool { self.0 == 0.0 && 1.0_f64.copysign(self.0) < 0.0}
  #[inline] fn is_real(self) -> bool { true }
  #[inline] fn is_imag(self) -> bool { false }
  #[inline] fn is_negreal(self) -> bool { self.0 < 0.0 }
  #[inline] fn is_posreal(self) -> bool { self.0 > 0.0 }
  #[inline] fn is_nonnegreal(self) -> bool { self.0 >= 0.0 }
  #[inline] fn is_nonposreal(self) -> bool { self.0 <= 0.0 }
  #[inline] fn is_int(self) -> bool { self == self.trunc() }
  #[inline] fn is_posint(self) -> bool { self.is_int() && self.is_posreal() }
  #[inline] fn is_negint(self) -> bool { self.is_int() && self.is_negreal() }
  #[inline] fn is_nonposint(self) -> bool { self.is_int() && self.is_nonposreal() }
  #[inline] fn is_nonnegint(self) -> bool { self.is_int() && self.is_nonnegreal() }
  #[inline] fn is_evenint(self) -> bool {
    const last_odd : f64 = 9007199254740991_f64; // 2^53-1
    self.is_int()
    && (self.0.abs() > last_odd
        || (self.0.abs().trunc() as i64)%2 == 0)
  }
  #[inline] fn is_oddint(self) -> bool {
    const last_odd : f64 = 9007199254740991_f64; // 2^53-1
    self.is_int()
    && self.0.abs() <= last_odd
    && (self.0.abs().trunc() as i64)%2 == 1
  }
  #[inline] fn is_halfint(self) -> bool { (self.0*2.0).is_int() }
}