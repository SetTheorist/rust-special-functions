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
  fn eq(&self, rhs: &isize) -> bool { self.eq(&(ι(*rhs): r64)) }
}
impl PartialEq<f64> for r64 {
  fn eq(&self, rhs: &f64) -> bool { self.eq(&(ι(*rhs): r64)) }
}

impl const Neg for r64 {
  type Output = r64;
  #[inline]
  fn neg(self) -> r64 { r64(-self.0) }
}

// TODO: ldexp style implementation
impl Shl<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shl(self, n: isize) -> r64 { self * (2.0_f64.powi(n as i32)) }
}
// TODO: ldexp style implementation
impl ShlAssign<isize> for r64 {
  #[inline]
  fn shl_assign(&mut self, n: isize) { *self *= 2.0_f64.powi(n as i32); }
}
// TODO: ldexp style implementation
impl Shr<isize> for r64 {
  type Output = r64;
  #[inline]
  fn shr(self, n: isize) -> r64 { self / (2.0_f64.powi(n as i32)) }
}
// TODO: ldexp style implementation
impl ShrAssign<isize> for r64 {
  #[inline]
  fn shr_assign(&mut self, n: isize) { *self /= 2.0_f64.powi(n as i32); }
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

////////////////////////////////////////////////////////////////////////////////

mod tests {
  use super::r64;
  use crate::traits::*;

  /*
  use approx::{*};
  impl AbsDiffEq for r64 {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
      f64::default_epsilon()
    }
    fn abs_diff_eq(&self, other:&Self, epsilon:Self::Epsilon) -> bool {
      f64::abs_diff_eq(&self.0, &other.0, epsilon)
    }
  }
  impl RelativeEq for r64 {
    fn default_max_relative() -> Self::Epsilon {
      f64::default_max_relative()
    }
    fn relative_eq(&self, other:&Self, epsilon:Self::Epsilon, max_relative:Self::Epsilon) -> bool {
      f64::relative_eq(&self.0, &other.0, epsilon, max_relative)
    }
  }
  impl UlpsEq for r64 {
    fn default_max_ulps() -> u32 {
      f64::default_max_ulps()
    }
    fn ulps_eq(&self, other:&Self, epsilon:Self::Epsilon, max_ulps:u32) -> bool {
      f64::ulps_eq(&self.0, &other.0, epsilon, max_ulps)
    }
  }
  */

  #[test]
  fn basic_ops() {
    for ia in -100..=100 {
      for ib in -100..=100 {
        let a: f64 = (ia as f64) / 10.0;
        let b: f64 = (ib as f64) / 10.0;
        let ra: r64 = r64(a);
        let rb: r64 = r64(b);

        assert_eq!((a + b), (ra + rb).0);
        assert_eq!(r64(a + b), (ra + rb));
        assert_eq!((a - b), (ra - rb).0);
        assert_eq!(r64(a - b), (ra - rb));
        assert_eq!((a * b), (ra * rb).0);
        assert_eq!(r64(a * b), (ra * rb));
        if ia != 0 || ib != 0 {
          assert_eq!((a / b), (ra / rb).0);
          assert_eq!(r64(a / b), (ra / rb));
        }
        if ib != 0 {
          assert_eq!((a % b), (ra % rb).0);
          assert_eq!(r64(a % b), (ra % rb));
        }

        assert_eq!(ra.recip(), 1 / ra);
        assert_eq!(rb.recip(), 1 / rb);

        assert_eq!(-r64(a), r64(-a));
        assert_eq!(ra.sqr(), ra * ra);
        assert_eq!(ra << 0, ra);
        assert_eq!(ra << 1, ra * 2);
        assert_eq!(ra << 2, ra * 4);
        assert_eq!(ra >> 0, ra);
        assert_eq!(ra >> 1, ra / 2);
        assert_eq!(ra >> 2, ra / 4);
        assert_eq!(ra << -1, ra >> 1);
        assert_eq!(ra << -2, ra >> 2);
        assert_eq!(ra >> -1, ra << 1);
        assert_eq!(ra >> -2, ra << 2);

        assert_eq!(ra, ra);
        assert_ne!(ra, ra + 1);

        assert!(ra <= ra);
        assert!(ra <= ra + 1);
        assert!(!(ra + 1 <= ra));
        assert!(!(ra < ra));
        assert!(ra < ra + 1);
        assert!(!(ra + 1 < ra));
        assert!(ra >= ra);
        assert!(!(ra >= ra + 1));
        assert!(ra + 1 >= ra);
        assert!(!(ra > ra));
        assert!(!(ra > ra + 1));
        assert!(ra + 1 > ra);
      }
    }

    assert_eq!(r64(1.0), 1.0);
    assert_eq!(r64(1.0), 1);
    assert_ne!(r64(1.0), 2.0);
    assert_ne!(r64(1.0), 2);
  }

  #[test]
  fn rounding_ops() {
    assert_eq!(r64(0.5).floor(), r64(0.0));
    assert_eq!(r64(0.5).ceil(), r64(1.0));
    assert_eq!(r64(0.5).round(), r64(1.0));
    assert_eq!(r64(0.5).trunc(), r64(0.0));
    assert_eq!(r64(0.5).rint(), 1);

    assert_eq!(r64(1.0).floor(), r64(1.0));
    assert_eq!(r64(1.0).ceil(), r64(1.0));
    assert_eq!(r64(1.0).round(), r64(1.0));
    assert_eq!(r64(1.0).trunc(), r64(1.0));
    assert_eq!(r64(1.0).rint(), 1);

    assert_eq!(r64(1.1).floor(), r64(1.0));
    assert_eq!(r64(1.1).ceil(), r64(2.0));
    assert_eq!(r64(1.1).round(), r64(1.0));
    assert_eq!(r64(1.1).trunc(), r64(1.0));
    assert_eq!(r64(1.1).rint(), 1);

    assert_eq!(r64(1.5).floor(), r64(1.0));
    assert_eq!(r64(1.5).ceil(), r64(2.0));
    assert_eq!(r64(1.5).round(), r64(2.0));
    assert_eq!(r64(1.5).trunc(), r64(1.0));
    assert_eq!(r64(1.5).rint(), 2);

    assert_eq!(r64(1.9).floor(), r64(1.0));
    assert_eq!(r64(1.9).ceil(), r64(2.0));
    assert_eq!(r64(1.9).round(), r64(2.0));
    assert_eq!(r64(1.9).trunc(), r64(1.0));
    assert_eq!(r64(1.9).rint(), 2);

    assert_eq!(r64(-0.5).floor(), r64(-1.0));
    assert_eq!(r64(-0.5).ceil(), r64(0.0));
    assert_eq!(r64(-0.5).round(), r64(-1.0));
    assert_eq!(r64(-0.5).trunc(), r64(0.0));
    assert_eq!(r64(-0.5).rint(), -1);

    assert_eq!(r64(-1.0).floor(), r64(-1.0));
    assert_eq!(r64(-1.0).ceil(), r64(-1.0));
    assert_eq!(r64(-1.0).round(), r64(-1.0));
    assert_eq!(r64(-1.0).trunc(), r64(-1.0));
    assert_eq!(r64(-1.0).rint(), -1);

    assert_eq!(r64(-1.1).floor(), r64(-2.0));
    assert_eq!(r64(-1.1).ceil(), r64(-1.0));
    assert_eq!(r64(-1.1).round(), r64(-1.0));
    assert_eq!(r64(-1.1).trunc(), r64(-1.0));
    assert_eq!(r64(-1.1).rint(), -1);

    assert_eq!(r64(-1.5).floor(), r64(-2.0));
    assert_eq!(r64(-1.5).ceil(), r64(-1.0));
    assert_eq!(r64(-1.5).round(), r64(-2.0));
    assert_eq!(r64(-1.5).trunc(), r64(-1.0));
    assert_eq!(r64(-1.5).rint(), -2);

    assert_eq!(r64(-1.9).floor(), r64(-2.0));
    assert_eq!(r64(-1.9).ceil(), r64(-1.0));
    assert_eq!(r64(-1.9).round(), r64(-2.0));
    assert_eq!(r64(-1.9).trunc(), r64(-1.0));
    assert_eq!(r64(-1.9).rint(), -2);
  }

  #[test]
  fn power_ops() {
    assert_eq!(r64(10.0).recip(), 0.1);
    assert_eq!(r64(10.0).sqr(), 100);
    assert_eq!(r64(100.0).sqrt(), 10);
    assert_eq!(r64(1000.0).cbrt(), 10);

    for ia in 1..=100 {
      let a: f64 = (ia as f64) / 10.0;
      let ra: r64 = r64(a);

      assert_eq!(ra.sqr().sqrt(), ra);
      assert!((ra.sqrt().sqr() - ra).abs() < ra.abs() * r64::epsilon * 2);

      assert!((ra.pow(3_usize).cbrt() - ra).abs() < ra.abs() * r64::epsilon);
      assert!((ra.cbrt().pow(3_usize) - ra).abs() < ra.abs() * r64::epsilon * 3);

      assert_eq!(ra.pow(0_usize), r64::one);
      assert_eq!(ra.pow(0_isize), r64::one);
      assert_eq!(ra.pow(r64(0.0)), r64::one);

      assert_eq!(ra.pow(1_usize), ra);
      assert_eq!(ra.pow(1_isize), ra);
      assert_eq!(ra.pow(r64(1.0)), ra);

      assert_eq!(ra.pow(3_usize), (ra * ra) * (ra));
      assert_eq!(ra.pow(3_isize), (ra * ra) * (ra));
      assert_eq!(ra.pow(-3_isize), ((ra * ra) * (ra)).recip());
      assert!((ra.pow(r64(3.0)) - (ra * ra) * (ra)).abs() < r64::epsilon * 2 * (ra.pow(3_usize)));

      assert_eq!(ra.pow(4_usize), (ra * ra) * (ra * ra));
      assert_eq!(ra.pow(4_isize), (ra * ra) * (ra * ra));
      assert_eq!(ra.pow(-4_isize), ((ra * ra) * (ra * ra)).recip());
      assert!((ra.pow(r64(4.0)) - (ra * ra) * (ra * ra)).abs() < r64::epsilon * 2 * (ra.pow(4_usize)));
    }
  }
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
