use num::complex::{Complex};
use num::traits::{NumAssign};
use num::traits::real::{Real};
use std::ops::{Neg};

////////////////////////////////////////////////////////////////////////////////

pub trait Embed<T> {
  fn embed(t:T) -> Self;
}
impl<T> Embed<T> for T {
  fn embed(t:T) -> Self { t }
}

pub trait Value :
  Copy+Default+Sized+NumAssign+Embed<isize>+Embed<f64>+Neg<Output=Self>
{
  type RT : Value+PartialOrd+Real;
  type CT : Value;
  fn epsilon() -> f64;
  fn is_real() -> bool;
  fn is_complex() -> bool { !Self::is_real() }
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn rabs(self) -> Self::RT;
  fn abs(self) -> Self;
  fn dabs(self) -> f64;
  fn from_real(r:Self::RT) -> Self;
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT;
  fn ldexp(self, n:i32) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

impl Embed<isize> for f64 { fn embed(t:isize) -> Self { t as f64 } }

impl Value for f64 {
  type RT = Self;
  type CT = Complex<f64>;
  fn epsilon() -> f64 { 1e-17 }
  fn is_real() -> bool { true }
  fn real(self) -> Self::RT { self }
  fn imag(self) -> Self::RT { 0.0 }
  fn rabs(self) -> Self::RT { self.abs() }
  fn abs(self) -> Self { (self as f64).abs() }
  fn dabs(self) -> f64 { (self as f64).abs() }
  fn from_real(r:Self::RT) -> Self { r }
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT { Complex::new(r,i) }
  fn ldexp(self, n:i32) -> Self { libm::ldexp(self, n) }
}

impl Embed<f64> for Complex<f64> { fn embed(t:f64) -> Self { Complex::new(t,0.0) } }
impl Embed<isize> for Complex<f64> { fn embed(t:isize) -> Self { Complex::new(Embed::embed(t),0.0) } }

impl Value for Complex<f64> {
  type RT = f64;
  type CT = Self;
  fn epsilon() -> f64 { 1e-17 }
  fn is_real() -> bool { false }
  fn real(self) -> Self::RT { self.re }
  fn imag(self) -> Self::RT { self.im }
  fn rabs(self) -> Self::RT { (self.re*self.re+self.im*self.im).sqrt() }
  fn abs(self) -> Self { Self::to_complex(self.rabs(),0.0) }//{ (self as Complex<f64>).abs() }
  fn dabs(self) -> f64 { (self as Complex<f64>).abs().re }
  fn from_real(r:Self::RT) -> Self { Complex::new(r,0.0) }
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT { Complex::new(r,i) }
  fn ldexp(self, n:i32) -> Self { Complex::new(libm::ldexp(self.re, n), libm::ldexp(self.im, n)) }
}


