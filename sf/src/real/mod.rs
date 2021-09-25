use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::traits::*;

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
    impl const $opt<r64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64(self.0 $x b.0) }
    }
    impl const $opt<f64> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: f64) -> r64 { r64(self.0 $x b) }
    }
    impl const $opt<r64> for f64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64(self $x b.0) }
    }
    impl const $opt<isize> for r64 {
      type Output = r64;
      #[inline]
      fn $op(self, b: isize) -> r64 { r64(self.0 $x (b as f64)) }
    }
    impl const $opt<r64> for isize {
      type Output = r64;
      #[inline]
      fn $op(self, b: r64) -> r64 { r64((self as f64) $x b.0) }
    }
    impl $opassignt<r64> for r64 {
      #[inline]
      fn $opassign(&mut self, b: r64) { *self = self.$op(b); }
    }
    impl $opassignt<f64> for r64 {
      #[inline]
      fn $opassign(&mut self, b: f64) { *self = self.$op(b); }
    }
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

macro_rules! lift1 {
  ($r:ident, $f:ident) => {
    #[inline]
    fn $r(self) -> Self { r64(self.0.$f()) }
  };
}

impl Constants for r64 {
  const E: r64 = r64(2.7182818284590452354);
  const PI: r64 = r64(3.1415926535897932385);
  const FRAC_1_PI: r64 = r64(0.31830988618379067154);
  const FRAC_PI_2: r64 = r64(1.5707963267948966192);
  const SQRT2PI: r64 = r64(2.5066282746310005024);
  const FRAC_1_SQRT2PI: r64 = r64(0.39894228040143267794);
  const FRAC_1_SQRTPI: r64 = r64(0.56418958354775628695);
  const LOG2: r64 = r64(0.69314718055994530942);
  const FRAC_1_LOG2: r64 = r64(1.4426950408889634074);
  const FRAC_LOG2PI_2: r64 = r64(0.91893853320467274178);
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
impl Embeds<r64> for r64 {}
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
  const mu_epsilon: Self = Self::epsilon;
}
impl RealType for r64 {}
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
  const nan: Self = r64(f64::NAN);
  const infinity: Self = r64(f64::INFINITY);
  const neg_infinity: Self = r64(f64::NEG_INFINITY);
  const neg_zero: Self = r64(unsafe{std::mem::transmute(0x8000_0000_0000_0000_u64)});
}

////////////////////////////////////////////////////////////////////////////////

use crate::exp::Exp;
impl Exp for r64 {
  lift1!(exp, exp);
}

use crate::log::Log;
impl Log for r64 {
  lift1!(log, ln);
}
