use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use num::complex::{Complex};
use crate::util::{power_i};

/*
pub trait Base : Copy+Sized
{ }

pub trait Zero : Base {
  const zero : Self;
}

pub trait One : Base {
  const one : Self;
}

pub trait Addition
  : Base + Zero
  + Add<Self,Output=Self> + AddAssign<Self>
{
}

pub trait Subtraction
  : Base
  + Sub<Self,Output=Self> + SubAssign<Self>
  + Neg<Output=Self>
{
}

pub trait AdditiveGroup
  : Addition + Subtraction
{
}

// absorb isize & f64 into operations also...

pub trait Multiplicative
  : Base + One
  + Mul<Self,Output=Self> + MulAssign<Self>
{
  fn sqr(self) -> Self { self*self }
  fn powu(self,n:usize) -> Self { power_u(self,n) }
}


  fn ldexp(self,n:isize) -> Self; // (maybe also << for this?)

pub trait Division
  : Base
  + Div<Self,Output=Self> + DivAssign<Self>
  + Rem<Self,Output=Self> + RemAssign<Self>
{
  fn recip(self) -> Self;
  fn powi(self,n:isize) -> Self { power_i(self,n) }
}

pub trait MultiplicativeGroup
  : Multiplication +  Division
{
}

pub trait Signed
{
  fn signum(self) -> Self;
  fn (self) -> Self;
}

pub trait Bounded
{
  const MIN_VALUE : Self;
  const MAX_VALUE : Self;
}

pub trait Ordered
{
  fn min(self,b:Self) -> Self { if self<b {self} else {b} }
  fn max(self,b:Self) -> Self { if self>b {self} else {b} }

  fn floor(self) -> Self;
  fn ceil(self) -> Self;
  fn round(self) -> Self;
  fn trunc(self) -> Self;
  fn rint(self) -> isize;
}

pub trait Normed
{
  type RT : Ordered;
  fn abs(self) -> Self::RT;
  fn fabs(self) -> f64;
}

pub trait Complex
{
  type RT;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn make(re:Self::RT,im:Self::RT) -> Self;
  fn polar(r:Self::RT,arg:Self::RT) -> Self;
}

pub trait Value : AdditiveGroup + MultiplicativeGroup
{
}

pub trait ??? : ...
{
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
}

pub trait ExpLog : ...
{
  fn exp(self) -> Self;
  fn log(self) -> Self;
  fn powf(self,e:Self) -> Self { (e*self.log()).exp() }
}

pub trait Constants
{
  const EPSILON : Self;
  const TINY : Self; // ~ O(sqrt(min-positive normal)) for example

  // $e^1$
  const E : Self;

  // $\pi$
  const PI : Self;
  // $1/\pi$
  const FRAC_1_PI : Self;
  // $\pi/2$
  const FRAC_PI_2 : Self;
  // $\sqrt(2\pi)$
  const SQRT2PI : Self;
  // $1/\sqrt(2\pi)$
  const FRAC_1_SQRT2PI : Self;
  // $1/\sqrt(\pi)$
  const FRAC_1_SQRTPI : Self;

  // $\log(2)$
  const LOG2 : Self;
  // $1/\log(2)$
  const FRAC_1_LOG2 : Self;

  // $log(2\pi)/2$
  const FRAC_LOG2PI_2 : Self;
}


*/


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

pub trait Ordered : PartialOrd<Self>+Sized {
  const MIN : Self;
  const MAX : Self;
  fn min(self,b:Self) -> Self { if self<b {self} else {b} }
  fn max(self,b:Self) -> Self { if self>b {self} else {b} }
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



