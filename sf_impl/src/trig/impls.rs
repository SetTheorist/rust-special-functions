use super::*;

////////////////////////////////////////////////////////////////////////////////
// TODO: quick placeholder impl
use crate::real::r64;
impl Trig for r64 {
  fn cos(self) -> Self { r64(self.0.cos()) }
  fn acos(self) -> Self { r64(self.0.acos()) }
  fn sin(self) -> Self { r64(self.0.sin()) }
  fn asin(self) -> Self { r64(self.0.asin()) }
  fn tan(self) -> Self { r64(self.0.tan()) }
  fn atan(self) -> Self { r64(self.0.atan()) }

  fn cosh(self) -> Self { r64(self.0.cosh()) }
  fn acosh(self) -> Self { r64(self.0.acosh()) }
  fn sinh(self) -> Self { r64(self.0.sinh()) }
  fn asinh(self) -> Self { r64(self.0.asinh()) }
  fn tanh(self) -> Self { r64(self.0.tanh()) }
  fn atanh(self) -> Self { r64(self.0.atanh()) }
}

////////////////////////////////////////////////////////////////////////////////
// TODO: quick placeholder impl
// NB These are _really_ hacky right now!!  Caveat emptor!
use crate::complex::c64;
impl Trig for c64 {
  fn cos(self) -> Self {
    c64 { re: ι(self.re.0.cos() * self.im.0.cosh()), im: ι(-self.re.0.sin() * self.im.0.sinh()) }
  }
  fn acos(self) -> Self {
    sf_log(self + c64::I*sf_sqrt(-self*self + 1)) / c64::I
  }
  fn asin(self) -> Self {
    sf_log(self*c64::I + sf_sqrt(-self*self + 1)) / c64::I
  }
  fn sin(self) -> Self {
    let r = self.re;
    let i = self.im;
    c64 { re: r.sin()*i.cosh(), im:r.cos()*i.sinh() }
  }
  fn atan(self) -> Self {
    sf_log((c64::I - self) / (c64::I + self)) / (c64::I*2)
  }
  fn cosh(self) -> Self {
    (sf_exp(self) + sf_exp(-self))/2
  }
  fn acosh(self) -> Self {
    sf_log(self + sf_sqrt(self*self - 1))
  }
  fn sinh(self) -> Self {
    (sf_exp(self) - sf_exp(-self))/2
  }
  fn asinh(self) -> Self {
    sf_log(self + sf_sqrt(self*self + 1))
  }
  fn tanh(self) -> Self {
    self.sinh() / self.cosh()
  }
  fn atanh(self) -> Self {
    sf_log((self + 1) / (-self + 1)) / 2
  }
}

////////////////////////////////////////////////////////////////////////////////