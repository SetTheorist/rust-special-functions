use num::complex::{Complex};
use num::traits::{NumAssign};

////////////////////////////////////////////////////////////////////////////////

trait Embed<T> {
  fn embed(t:T) -> Self;
}
impl<T> Embed<T> for T {
  fn embed(t:T) -> Self { t }
}

trait Value :
  Copy+Default+Sized+NumAssign+Embed<isize>+Embed<f64>
{
  type RT : Value;
  type CT : Value;
  fn is_real() -> bool;
  fn is_complex() -> bool { !Self::is_real() }
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT;
  fn rabs(self) -> Self::RT;
}

////////////////////////////////////////////////////////////////////////////////

impl Embed<isize> for f64 { fn embed(t:isize) -> Self { t as f64 } }

impl Value for f64 {
  type RT = f64;
  type CT = Complex<f64>;
  fn is_real() -> bool { true }
  fn real(self) -> Self::RT { self }
  fn imag(self) -> Self::RT { 0.0 }
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT { Complex::new(r,i) }
  fn rabs(self) -> Self::RT { self.abs() }
}

impl Embed<f64> for Complex<f64> { fn embed(t:f64) -> Self { Complex::new(t,0.0) } }
impl Embed<isize> for Complex<f64> { fn embed(t:isize) -> Self { Complex::new(Embed::embed(t),0.0) } }

impl Value for Complex<f64> {
  type RT = f64;
  type CT = Complex<f64>;
  fn is_real() -> bool { false }
  fn real(self) -> Self::RT { self.re }
  fn imag(self) -> Self::RT { self.im }
  fn to_complex(r:Self::RT,i:Self::RT) -> Self::CT { Complex::new(r,i) }
  fn rabs(self) -> Self::RT { (self.re*self.re+self.im*self.im).sqrt() }
}


