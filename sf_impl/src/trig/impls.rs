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
    let r = self.re;
    let i = self.im;
    c64::new(r.cos() * i.cosh(), -r.sin() * i.sinh())
  }
  fn sin(self) -> Self {
    let r = self.re;
    let i = self.im;
    c64::new(r.sin()*i.cosh(), r.cos()*i.sinh())
  }
  fn cos_sin(self) -> (Self,Self) {
    let r = self.re;
    let i = self.im;
    let (rc,rs) = r.cos_sin();
    let (ich,ish) = i.cosh_sinh();
    let cos = c64::new(rc*ich, -rs*ish);
    let sin = c64::new(rs*ich, rc*ish);
    (cos, sin)
  }
  fn acos(self) -> Self {
    sf_log(self + c64::I*sf_sqrt(-self*self + 1)) / c64::I
  }
  fn asin(self) -> Self {
    sf_log(self*c64::I + sf_sqrt(-self*self + 1)) / c64::I
  }
  fn atan(self) -> Self {
    sf_log((c64::I - self) / (c64::I + self)) / (c64::I*2)
  }
  fn cosh(self) -> Self {
    let pe = sf_exp(self);
    let me = sf_exp(-self); // pe.recip();
    (pe + me)/2
  }
  fn acosh(self) -> Self {
    sf_log(self + sf_sqrt(self*self - 1))
  }
  fn sinh(self) -> Self {
    let pe = sf_exp(self);
    let me = sf_exp(-self); // pe.recip();
    (pe - me)/2
  }
  fn cosh_sinh(self) -> (Self,Self) {
    let pe = sf_exp(self);
    let me = sf_exp(-self); // pe.recip();
    let cosh = (pe + me)/2;
    let sinh = (pe - me)/2;
    (cosh, sinh)
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