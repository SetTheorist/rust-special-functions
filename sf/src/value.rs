/*
use num::complex::{Complex};
use num::traits::{NumAssign};
use std::ops::{Neg};
use crate::embed::{*};
use crate::traits::{*};

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

pub trait Value :
  Copy+Default+Sized
  +NumAssign+Neg<Output=Self>
  +Embed<isize>+Embed<f64>
  +Field
  +std::fmt::Debug
{
  fn epsilon() -> f64;
  fn vabs(self) -> Self;
  fn dabs(self) -> f64;
  fn ldexp(self, n:i32) -> Self;
}

pub trait RealValue :
  Value + PartialOrd
{
  type CT : ComplexValue;
  fn floor(self) -> Self;
  fn ceil(self) -> Self;
  fn round(self) -> Self;
  fn rint(self) -> isize;
}

pub trait ComplexValue : Value
  + Embed<Self::RT>
{
  type RT : RealValue;
  fn rabs(self) -> Self::RT;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn make_complex(r:Self::RT, i:Self::RT) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

impl Value for f64 {
  #[inline]
  fn epsilon() -> f64 { f64::EPSILON }
  #[inline]
  fn vabs(self) -> Self { (self as f64).abs() }
  #[inline]
  fn dabs(self) -> f64 { (self as f64).abs() }
  #[inline]
  fn ldexp(self, n:i32) -> Self { libm::ldexp(self, n) }
}

impl RealValue for f64 {
  type CT = Complex<f64>;
  fn floor(self) -> Self { self.floor() }
  fn ceil(self) -> Self { self.ceil() }
  fn round(self) -> Self { self.round() }
  fn rint(self) -> isize { self.round() as isize }
}

////////////////////////////////////////////////////////////////////////////////

impl Value for Complex<f64> {
  #[inline]
  fn epsilon() -> f64 { f64::EPSILON }
  #[inline]
  fn vabs(self) -> Self { ι(self.rabs()) }
  #[inline]
  fn dabs(self) -> f64 { self.rabs() }
  #[inline]
  fn ldexp(self, n:i32) -> Self { Complex::new(libm::ldexp(self.re, n), libm::ldexp(self.im, n)) }
}

impl ComplexValue for Complex<f64> {
  type RT = f64;
  #[inline]
  fn rabs(self) -> Self::RT { self.norm() }
  #[inline]
  fn real(self) -> Self::RT { self.re }
  #[inline]
  fn imag(self) -> Self::RT { self.im }
  #[inline]
  fn make_complex(r:Self::RT, i:Self::RT) -> Self { Complex::new(r,i) }
}


*/
