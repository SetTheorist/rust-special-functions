use crate::real::r64;
use crate::traits::*;
use crate::exp::*;
use crate::trig::Trig;

pub mod ops;
pub use ops::*;

// TODO: Make generic on any RealValue type?
// (probably not feasible until type specialization exists)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub struct c64 {
  pub re: r64,
  pub im: r64,
}

impl c64 {
  const I: c64 = c64 { re: r64::zero, im: r64::one };

  #[inline]
  pub fn new(re: r64, im: r64) -> c64 { c64 { re, im } }
}

impl std::fmt::Display for c64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.re, f)?;
    if self.im >= ι(0) { write!(f, "+")?; }
    std::fmt::Display::fmt(&self.im, f)?;
    write!(f, "ⅈ")
    //write!(f, "î")
  }
}

impl std::fmt::LowerExp for c64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::LowerExp::fmt(&self.re, f)?;
    if self.im >= ι(0) { write!(f, "+")?; }
    std::fmt::LowerExp::fmt(&self.im, f)?;
    write!(f, "ⅈ")
    //write!(f, "î")
  }
}

impl std::fmt::UpperExp for c64 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::UpperExp::fmt(&self.re, f)?;
    if self.im >= ι(0) { write!(f, "+")?; }
    std::fmt::UpperExp::fmt(&self.im, f)?;
    write!(f, "ⅈ")
    //write!(f, "î")
  }
}

impl const From<r64> for c64 {
  #[inline]
  fn from(x: r64) -> c64 { c64 { re:x, im:r64::zero } }
}
impl const From<f64> for c64 {
  #[inline]
  fn from(x: f64) -> c64 { c64 { re:r64::from(x), im:r64::zero } }
}
impl From<isize> for c64 {
  #[inline]
  fn from(x: isize) -> c64 { c64 { re: ι(x), im: ι(0) } }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! from_r64 {
  ($c:ident) => {
    const $c:c64 = c64 { re:r64::$c, im:r64::zero };
  }
}
impl Constants for c64 {
  const nan: c64 = c64 { re: r64::nan, im: r64::nan };
  from_r64!(E);
  from_r64!(FRAC_1_E);
  from_r64!(PI);
  from_r64!(FRAC_1_PI);
  from_r64!(SQRTPI);
  from_r64!(SQRT2PI);
  from_r64!(FRAC_PI_2);
  from_r64!(FRAC_1_SQRTPI);
  from_r64!(FRAC_1_SQRT2PI);
  from_r64!(LOG2);
  from_r64!(FRAC_1_LOG2);
  from_r64!(FRAC_LOG2PI_2);
  from_r64!(EULER_GAMMA);
}

impl Classify for c64 {
  #[inline] fn is_nan(self) -> bool { self.re.is_nan() || self.im.is_nan() }
  #[inline] fn is_infinite(self) -> bool { self.re.is_infinite() || self.im.is_infinite()}
  #[inline] fn is_finite(self) -> bool { self.re.is_finite() && self.im.is_finite() }
  #[inline] fn is_zero(self) -> bool { self.re.is_zero() && self.im.is_zero() }
  #[inline] fn is_negzero(self) -> bool { self.im.is_zero() && self.re.is_negzero() }
  #[inline] fn is_real(self) -> bool { self.im.is_zero() }
  #[inline] fn is_imag(self) -> bool { self.re.is_zero() }
  #[inline] fn is_negreal(self) -> bool { self.is_real() && self.re.is_negreal() }
  #[inline] fn is_posreal(self) -> bool { self.is_real() && self.re.is_posreal() }
  #[inline] fn is_nonnegreal(self) -> bool { self.is_real() && self.re.is_nonnegreal() }
  #[inline] fn is_nonposreal(self) -> bool { self.is_real() && self.re.is_nonposreal() }
  #[inline] fn is_int(self) -> bool { self.re.is_int() && self.is_real() }
  #[inline] fn is_posint(self) -> bool { self.is_int() && self.is_posreal() }
  #[inline] fn is_negint(self) -> bool { self.is_int() && self.is_negreal() }
  #[inline] fn is_nonposint(self) -> bool { self.is_int() && self.is_nonposreal() }
  #[inline] fn is_nonnegint(self) -> bool { self.is_int() && self.is_nonnegreal() }
  #[inline] fn is_evenint(self) -> bool { self.is_real() && self.re.is_evenint() }
  #[inline] fn is_oddint(self) -> bool { self.is_real() && self.re.is_oddint() }
  #[inline] fn is_halfint(self) -> bool { self.is_real() && self.re.is_halfint() }
}

impl Base for c64 {}
impl Zero for c64 {
  const zero: c64 = c64 { re: r64(0.0), im: r64(0.0) };
}
impl Addition for c64 {}
impl Subtraction for c64 {}
impl Additive for c64 {}
impl One for c64 {
  const one: c64 = c64 { re: r64(1.0), im: r64(0.0) };
}
impl Multiplication for c64 {}
impl Division for c64 {}
impl Multiplicative for c64 {}
impl Embeds<isize> for c64 {}
impl Embeds<f64> for c64 {}
impl Embeds<r64> for c64 {}
impl Field for c64 {}

impl Roots for c64 {
  #[inline]
  fn sqrt(self) -> Self {
    let (r, a) = self.to_polar();
    c64::polar(r.sqrt(), a / 2)
  }
  #[inline]
  fn cbrt(self) -> Self {
    let (r, a) = self.to_polar();
    c64::polar(r.cbrt(), a / 3)
  }
  fn nth_root(self, n: isize) -> Self {
    let (r, a) = self.to_polar();
    c64::polar(r.nth_root(n), a / n)
  }
}

impl Normed for c64 {
  type NT = r64;
  const epsilon: Self::NT = r64(f64::EPSILON);
  #[inline]
  fn abs(self) -> Self::NT {
    // TODO: robustify
    (self.re * self.re + self.im * self.im).sqrt()
  }
  #[inline]
  fn vabs(self) -> Self { c64{re:self.abs(),im:r64(0.0)} }
  #[inline]
  fn fabs(self) -> f64 { self.abs().0 }
  #[inline]
  fn signum(self) -> Self { self / self.abs() }
  #[inline]
  fn mu(self) -> Self::NT { sf_max(self.re.mu(), self.im.mu()) }
}

impl ComplexType for c64 {
  type RT = r64;
  const I : c64 = c64{re:r64(0.0), im:r64(1.0)};
  #[inline]
  fn real(self) -> Self::RT { self.re }
  #[inline]
  fn imag(self) -> Self::RT { self.im }
  #[inline]
  fn arg(self) -> Self::RT { r64(self.im.0.atan2(self.re.0)) }
  #[inline]
  fn conj(self) -> Self { c64 { re: self.re, im: -self.im } }
  #[inline]
  fn rect(re: Self::RT, im: Self::RT) -> Self { c64 { re, im } }
  #[inline]
  fn polar(r: Self::RT, arg: Self::RT) -> Self {
    let (cos, sin) = arg.cos_sin();
    let re = r * cos;
    let im = r * sin;
    c64 { re, im }
  }
  fn root_of_unity(n:isize) -> Self {
    c64::polar(r64(1.0), r64::PI*2/n)
  }
  fn root_of_unity_pow(n:isize, k:isize) -> Self {
    c64::polar(r64(1.0), r64::PI*2/n*(k%n))
  }
}

impl Value for c64 {}

impl Power<r64> for c64 {
  fn pow(self, p: r64) -> c64 {
    // TODO: temporary quick implementation
    let (r, th) = self.to_polar();
    c64::polar(r.pow(p), th * p)
  }
}

impl Power for c64 {
  fn pow(self, p: c64) -> c64 {
    // TODO: temporary quick implementation
    let (r, th) = self.to_polar();
    if p.im == 0 {
      c64::polar(r.pow(p.im), th * p.im)
    } else if p.re == 0 {
      c64::polar((-th * p.im).exp(), r.log() * p.im)
    } else {
      c64::polar(r.pow(p.re - th * p.im), r.log() * p.im + th * p.re)
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

use crate::log::Log;
impl Log for c64 {
  fn log(self) -> c64 {
    // TODO: temporary quick implementation
    let r = self.abs();
    let th = self.arg();
    c64::rect(r.log(), th)
  }
}
