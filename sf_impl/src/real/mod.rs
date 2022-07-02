use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::traits::*;

mod tests;

// TODO: make r32 & c32 also?

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
pub struct r64(pub f64);

impl std::fmt::Display for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "ρ")?;
    std::fmt::Display::fmt(&self.0, f)
  }
}

impl std::fmt::LowerExp for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "ρ")?;
    std::fmt::LowerExp::fmt(&self.0, f)
  }
}

impl std::fmt::UpperExp for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "ρ")?;
    std::fmt::UpperExp::fmt(&self.0, f)
  }
}

// TODO: ignores formatting specifiers
impl std::fmt::LowerHex for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let b : u64 = unsafe{std::mem::transmute(self.0)};
    write!(f, "ρ")?;
    write!(f, "{:01x}", b>>63)?;
    write!(f, ":")?;
    write!(f, "{:03x}", (b>>52)&0x7FF)?;
    write!(f, ":")?;
    write!(f, "{:013x}", b&0x000F_FFFF_FFFF_FFFF)
  }
}

// TODO: ignores formatting specifiers
impl std::fmt::UpperHex for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let b : u64 = unsafe{std::mem::transmute(self.0)};
    write!(f, "ρ")?;
    write!(f, "{:01X}", b>>63)?;
    write!(f, ":")?;
    write!(f, "{:03X}", (b>>52)&0x7FF)?;
    write!(f, ":")?;
    write!(f, "{:013X}", b&0x000F_FFFF_FFFF_FFFF)
  }
}

// TODO: ignores formatting specifiers
impl std::fmt::Binary for r64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let b : u64 = unsafe{std::mem::transmute(self.0)};
    write!(f, "ρ")?;
    write!(f, "{:01b}", b>>63)?;
    write!(f, ":")?;
    write!(f, "{:011b}", (b>>52)&0x7FF)?;
    write!(f, ":")?;
    write!(f, "{:52b}", b&0x000F_FFFF_FFFF_FFFF)
  }
}

impl const From<f64> for r64 {
  #[inline]
  fn from(x: f64) -> r64 { r64(x) }
}
impl const From<isize> for r64 {
  #[inline]
  fn from(x: isize) -> r64 { r64(x as f64) }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! add_ops {
  ($x:tt, $opt:ident, $op:ident; $opassignt:ident, $opassign:ident) => {
    #[automatically_derived]
    impl const $opt<r64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64(self.0 $x b.0) }
    }
    #[automatically_derived]
    impl const $opt<f64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: f64) -> r64 { r64(self.0 $x b) }
    }
    #[automatically_derived]
    impl const $opt<r64> for f64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64(self $x b.0) }
    }
    #[automatically_derived]
    impl const $opt<isize> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: isize) -> r64 { r64(self.0 $x (b as f64)) }
    }
    #[automatically_derived]
    impl const $opt<r64> for isize {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64((self as f64) $x b.0) }
    }
    #[automatically_derived]
    impl $opassignt<r64> for r64 {
      #[inline]
      fn $opassign(&mut self, b: r64) { *self = self.$op(b); }
    }
    #[automatically_derived]
    impl $opassignt<f64> for r64 {
      #[inline]
      fn $opassign(&mut self, b: f64) { *self = self.$op(b); }
    }
    #[automatically_derived]
    impl $opassignt<isize> for r64 {
      #[inline]
      fn $opassign(&mut self, b: isize) { *self = self.$op(b); }
    }
  };
}

add_ops!(+, Add, add; AddAssign, add_assign);
add_ops!(-, Sub, sub; SubAssign, sub_assign);
add_ops!(*, Mul, mul; MulAssign, mul_assign);
add_ops!(/, Div, div; DivAssign, div_assign);
add_ops!(%, Rem, rem; RemAssign, rem_assign);

impl PartialEq<isize> for r64 {
  fn eq(&self, rhs: &isize) -> bool { self.eq(&(ι(*rhs):r64)) }
}
impl PartialEq<f64> for r64 {
  fn eq(&self, rhs: &f64) -> bool { self.eq(&(ι(*rhs):r64)) }
}

impl const Neg for r64 {
  type Output = r64;
  #[inline]
  fn neg(self) -> r64 { r64(-self.0) }
}

impl Shl<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shl(self, n: isize) -> r64 { r64(self.0.ldexp(n)) }
}
impl ShlAssign<isize> for r64 {
  #[inline]
  fn shl_assign(&mut self, n: isize) { *self = r64(self.0.ldexp(n)); }
}
impl Shr<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shr(self, n: isize) -> r64 { r64(self.0.ldexp(-n)) }
}
impl ShrAssign<isize> for r64 {
  #[inline]
  fn shr_assign(&mut self, n: isize) { *self = r64(self.0.ldexp(-n)); }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> std::iter::Sum<&'a Self> for r64 {
  fn sum<I>(iter: I) -> Self where I: Iterator<Item=&'a Self> {
    iter.fold(r64::zero, |a,b|a+*b)
  }
}
impl std::iter::Sum<Self> for r64 {
  fn sum<I>(iter: I) -> Self where I: Iterator<Item=Self> {
    iter.fold(r64::zero, |a,b|a+b)
  }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! lift1 {
  ($r:ident, $f:ident) => {
    #[inline]
    fn $r(self) -> Self { r64(self.0.$f()) }
  };
}

impl Constants for r64 {
  const nan: Self = r64(f64::NAN);

  const E: r64              = r64(2.7182818284590452354);
  const FRAC_1_E: r64       = r64(0.3678794411714423215);
  const PI: r64             = r64(3.1415926535897932385);
  const SQRTPI: r64         = r64(1.7724538509055160273);
  const SQRT2PI: r64        = r64(2.5066282746310005024);
  const FRAC_1_PI: r64      = r64(0.31830988618379067154);
  const FRAC_PI_2: r64      = r64(1.5707963267948966192);
  const FRAC_1_SQRT2PI: r64 = r64(0.39894228040143267794);
  const FRAC_1_SQRTPI: r64  = r64(0.56418958354775628695);
  const LOG2: r64           = r64(0.69314718055994530942);
  const FRAC_1_LOG2: r64    = r64(1.4426950408889634074);
  const FRAC_LOG2PI_2: r64  = r64(0.91893853320467274178);
  const EULER_GAMMA: r64    = r64(0.57721566490153286061);
}

impl Base for r64 {}
impl Zero for r64 { const zero: r64 = r64(0.0); }
impl Addition for r64 {}
impl Subtraction for r64 {}
impl Additive for r64 {}
impl One for r64 { const one: r64 = r64(1.0); }
impl Multiplication for r64 {}
impl Division for r64 {}
impl Multiplicative for r64 {}
impl Embeds<isize> for r64 {}
impl Embeds<f64> for r64 {}
impl Field for r64 {}
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
  const epsilon: Self = r64(f64::EPSILON);
  lift1!(abs, abs);
  lift1!(vabs, abs);
  lift1!(signum, signum);
  #[inline]
  fn fabs(self) -> f64 { self.abs().0 }
  fn mu(self) -> Self { self.abs() }
}
impl RealType for r64 {
  type CT = crate::complex::c64;
}
impl Bounded for r64 {
  const MIN_VALUE: r64 = r64(f64::MIN);
  const MAX_VALUE: r64 = r64(f64::MAX);
}
impl Roots for r64 {
  lift1!(sqrt, sqrt);
  lift1!(cbrt, cbrt);
  #[inline]
  fn nth_root(self, n: isize) -> Self { r64(self.0.powf(1.0 / (n as f64))) }
}
impl Value for r64 {}

impl Power for r64 {
  fn pow(self, p: r64) -> r64 { r64(self.0.powf(p.0)) }
}

impl Classify for r64 {
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

impl Float for r64 {
  #[inline]
  fn frexp(self) -> (Self, isize) {
    let (a,b) = self.0.frexp();
    (r64(a),b)
  }
  #[inline]
  fn ilogb(self) -> isize {
    self.0.ilogb()
  }
  #[inline]
  fn ldexp(self, n:isize) -> Self {
    r64(self.0.ldexp(n))
  }
  #[inline]
  fn copysign(self, x:Self) -> Self {
    r64(self.0.copysign(x.0))
  }
  lift1!(next_up, next_up);
  lift1!(next_dn, next_dn);
  #[inline]
  fn identical(self, rhs:Self) -> bool {
    self.0.identical(rhs.0)
  }
  const infinity: Self = r64(f64::INFINITY);
  const neg_infinity: Self = r64(f64::NEG_INFINITY);
  const neg_zero: Self = r64(unsafe{std::mem::transmute(0x8000_0000_0000_0000_u64)});
}

////////////////////////////////////////////////////////////////////////////////

use crate::log::Log;
impl Log for r64 {
  lift1!(log, ln);
}