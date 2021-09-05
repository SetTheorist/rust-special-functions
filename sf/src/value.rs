use num::complex::{Complex};
use num::traits::{NumAssign};
use num::traits::real::{Real};
use std::ops::{Neg};
use crate::embed::{*};
use crate::traits::{*};

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

pub trait Value :
  Copy+Default+Sized
  +NumAssign+Neg<Output=Self>
  +Embed<isize>+Embed<f64>+Embed<Self::RT>
  +Field
{
  type RT : Value+PartialOrd+Real;
  type CT : Value + Embed<Self>;
  fn epsilon() -> f64;
  fn rabs(self) -> Self::RT;
  fn abs(self) -> Self;
  fn dabs(self) -> f64;
  fn ldexp(self, n:i32) -> Self;
  fn from_real(r:Self::RT) -> Self;
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT;
  fn complex_retract(r:Self::RT,i:Self::RT) -> Self;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn is_real_type() -> bool;
  fn is_complex_type() -> bool { !Self::is_real_type() }
}

////////////////////////////////////////////////////////////////////////////////


impl Value for f64 {
  type RT = Self;
  type CT = Complex<Self>;
  #[inline]
  fn epsilon() -> f64 { f64::EPSILON }
  #[inline]
  fn rabs(self) -> Self::RT { self.abs() }
  #[inline]
  fn abs(self) -> Self { (self as f64).abs() }
  #[inline]
  fn dabs(self) -> f64 { (self as f64).abs() }
  #[inline]
  fn ldexp(self, n:i32) -> Self { libm::ldexp(self, n) }
  #[inline]
  fn from_real(r:Self::RT) -> Self { r }
  #[inline]
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT { Complex::new(r,i) }
  #[inline]
  fn complex_retract(r:Self::RT,_i:Self::RT) -> Self { r }
  #[inline]
  fn real(self) -> Self::RT { self }
  #[inline]
  fn imag(self) -> Self::RT { 0.0 }
  #[inline]
  fn is_real_type() -> bool { true }
}

impl Value for Complex<f64> {
  type RT = f64;
  type CT = Self;
  #[inline]
  fn epsilon() -> f64 { f64::EPSILON }
  #[inline]
  fn rabs(self) -> Self::RT { (self.re*self.re+self.im*self.im).sqrt() }
  #[inline]
  fn abs(self) -> Self { Self::to_complex(self.rabs(),0.0) }//{ (self as Complex<f64>).abs() }
  #[inline]
  fn dabs(self) -> f64 { (self as Complex<f64>).abs().re }
  #[inline]
  fn ldexp(self, n:i32) -> Self { Complex::new(libm::ldexp(self.re, n), libm::ldexp(self.im, n)) }
  #[inline]
  fn real(self) -> Self::RT { self.re }
  #[inline]
  fn imag(self) -> Self::RT { self.im }
  #[inline]
  fn from_real(r:Self::RT) -> Self { Complex::new(r,0.0) }
  #[inline]
  fn to_complex(r:Self::RT,i:Self::RT) -> Self { Complex::new(r,i) }
  #[inline]
  fn complex_retract(r:Self::RT,i:Self::RT) -> Self { Self::to_complex(r,i) }
  #[inline]
  fn is_real_type() -> bool { false }
}



