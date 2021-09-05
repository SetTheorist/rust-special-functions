use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use num::complex::{Complex};
use crate::util::{power_i};

pub trait Additive
  : Add<Self,Output=Self> + AddAssign<Self>
  + Sub<Self,Output=Self> + SubAssign<Self>
  + Neg<Output=Self>
  + Copy+Sized
{
  const ZERO : Self;
}

pub trait Multiplicative
  : Mul<Self,Output=Self> + MulAssign<Self>
  + Div<Self,Output=Self> + DivAssign<Self>
  + Rem<Self,Output=Self> + RemAssign<Self>
  + Copy+Sized
{
  const ONE : Self;
  fn recip(self) -> Self;
  fn sqr(self) -> Self { self*self }
  fn powi(self,n:isize) -> Self { power_i(self,n) }
}

pub trait Ordered : PartialOrd<Self> {
  const MIN : Self;
  const MAX : Self;
}

pub trait Float {
  const EPSILON : Self;
  const NAN : Self;
  const INFINITY : Self;
  fn is_nan(self) -> bool;
  fn is_infinite(self) -> bool;
  fn is_finite(self) -> bool;
}

/*
pub trait Constants {
  // $\pi$
  const PI : Self;
  // $1/\pi$
  const ONE_PI : Self;
  // $\ln(2)$
  const LN2 : Self;
  // $e^{1}$
  const E : Self;
}
*/

pub trait Field : Additive + Multiplicative + Float {}

////////////////////////////////////////////////////////////////////////////////

impl Additive for f64 {
  const ZERO : Self = 0.0;
}

impl Multiplicative for f64 {
  const ONE : Self = 1.0;
  fn recip(self) -> Self { 1.0/self }
}

impl Float for f64 {
  const EPSILON : Self = f64::EPSILON;
  const NAN : Self = f64::NAN;
  const INFINITY : Self = f64::INFINITY;
  fn is_nan(self) -> bool { self.is_nan() }
  fn is_infinite(self) -> bool { self.is_infinite() }
  fn is_finite(self) -> bool { self.is_finite() }
}

impl Ordered for f64 {
  const MIN : Self = f64::MIN_POSITIVE;
  const MAX : Self = f64::MAX;
}

impl Field for f64 {}

////////////////////////////////////////////////////////////////////////////////

impl Additive for Complex<f64> {
  const ZERO : Self = Complex{re:0.0,im:0.0};
}

impl Multiplicative for Complex<f64> {
  const ONE : Self = Complex{re:1.0,im:0.0};
  fn recip(self) -> Self { 1.0/self }
}

impl Float for Complex<f64> {
  const EPSILON : Self = Complex{re:f64::EPSILON,im:0.0};
  const NAN : Self = Complex{re:f64::NAN,im:f64::NAN};
  const INFINITY : Self = Complex{re:f64::INFINITY,im:f64::INFINITY};
  fn is_nan(self) -> bool { self.re.is_nan() || self.im.is_nan() }
  fn is_infinite(self) -> bool { self.re.is_infinite() || self.im.is_infinite() }
  fn is_finite(self) -> bool { self.re.is_finite() && self.im.is_finite() }
}

impl Field for Complex<f64> {}

////////////////////////////////////////////////////////////////////////////////



