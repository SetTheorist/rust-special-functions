use crate::traits::{*};
use crate::log::{sf_log};
use crate::exp::{sf_exp};

// TODO: split into Trig and TrigExtra (TrigObscure?)

// NB lots TODO here
pub trait Trig: Value+Constants {
  // cosine
  fn cos(self) -> Self; // MUST IMPLEMENT
  fn acos(self) -> Self; // MUST IMPLEMENT
  // vercosine
  fn vcos(self) -> Self { (self/2).cos().sqr()*2 }
  // havercosine
  fn hvcos(self) -> Self{ (self/2).cos().sqr() }
  // cos(pi*x)
  fn cos_pix(self) -> Self { (self*Self::PI).cos() }
  // cos(x) - 1
  fn cos_m1(self) -> Self { self.cos() - 1 }

  // sine
  fn sin(self) -> Self; // MUST IMPLEMENT
  fn asin(self) -> Self; // MUST IMPLEMENT
  // versine
  fn vsin(self) -> Self { (self/2).sin().sqr()*2 }
  // haversine
  fn hvsin(self) -> Self { (self/2).sin().sqr() }
  // sin(pi*x)
  fn sin_pix(self) -> Self { (self*Self::PI).sin() }

  fn cos_sin(self) -> (Self, Self) { (self.cos(), self.sin()) }

  // tangent
  fn tan(self) -> Self { self.sin() / self.cos() }
  fn atan(self) -> Self; // MUST IMPLEMENT
  fn cot(self) -> Self { self.cos() / self.sin() }

  // secant
  fn sec(self) -> Self { self.cos().recip() }
  // exsecant
  fn exsec(self) -> Self { self.vsin() * self.sec() }
  // cosecant
  fn csc(self) -> Self { self.sin().recip() }

  //fn atan2(self,x:Self) -> Self;

  // hyperbolic cosine
  fn cosh(self) -> Self; // MUST IMPLEMENT
  fn acosh(self) -> Self; // MUST IMPLEMENT
  // cosh(x) - 1
  fn cosh_m1(self) -> Self { self.cosh() - 1 }

  // hyperbolic sine
  fn sinh(self) -> Self; // MUST IMPLEMENT
  fn asinh(self) -> Self; // MUST IMPLEMENT
  // sinh(x)/x
  fn sinh_mx(self) -> Self { self.sinh()/self }

  fn tanh(self) -> Self; // MUST IMPLEMENT
  fn atanh(self) -> Self; // MUST IMPLEMENT

  // hyperbolic secant
  fn sech(self) -> Self { self.cosh().recip() }
  fn asech(self) -> Self { self.recip().acosh() }

  // Gudermannian
  // = asin(tanh(x)) = 2 atan(exp(x))-pi/2 = 2 atan(tanh(x/2)) = atan(sinh(x))
  fn gud(self) -> Self { self.tanh().asin() }

  // Inverse Gudermannian
  // = log(abs((1+sin(z))/cos(z))) = log(abs((1+sin(z))/(1-sin(z))))/2
  // = log(abs(tan(z) + sec(z)) = log(abs(tan(pi/4 + z/2)))
  // = atanh(sin(z)) = sinh(tan(x))
  fn agud(self) -> Self { self.tan().asinh() }
}

////////////////////////////////////////////////////////////////////////////////

#[inline] pub fn sf_acos<V: Trig>(x: V) -> V { x.acos() }
#[inline] pub fn sf_acosh<V:Trig>(x:V) -> V { x.acosh() }
#[inline] pub fn sf_agud<V:Trig>(x:V) -> V { x.agud() }
#[inline] pub fn sf_asech<V:Trig>(x:V) -> V { x.asech() }
#[inline] pub fn sf_asin<V: Trig>(x: V) -> V { x.asin() }
#[inline] pub fn sf_asinh<V:Trig>(x:V) -> V { x.asinh() }
#[inline] pub fn sf_atan<V:Trig>(x:V) -> V { x.atan() }
#[inline] pub fn sf_atanh<V:Trig>(x:V) -> V { x.atanh() }
#[inline] pub fn sf_cos_m1<V:Trig>(x:V) -> V { x.cos_m1() }
#[inline] pub fn sf_cos_pix<V:Trig>(x:V) -> V { x.cos_pix() }
#[inline] pub fn sf_cos<V: Trig>(x: V) -> V { x.cos() }
#[inline] pub fn sf_cosh_m1<V:Trig>(x:V) -> V { x.cosh_m1() }
#[inline] pub fn sf_cosh<V:Trig>(x:V) -> V { x.cosh() }
#[inline] pub fn sf_cot<V:Trig>(x:V) -> V { x.cot() }
#[inline] pub fn sf_csc<V:Trig>(x:V) -> V { x.csc() }
#[inline] pub fn sf_exsec<V:Trig>(x:V) -> V { x.exsec() }
#[inline] pub fn sf_gud<V:Trig>(x:V) -> V { x.gud() }
#[inline] pub fn sf_hvcos<V:Trig>(x:V) -> V { x.hvcos() }
#[inline] pub fn sf_hvsin<V:Trig>(x:V) -> V { x.hvsin() }
#[inline] pub fn sf_sec<V:Trig>(x:V) -> V { x.sec() }
#[inline] pub fn sf_sech<V:Trig>(x:V) -> V { x.sech() }
#[inline] pub fn sf_sin_pix<V:Trig>(x:V) -> V { x.sin_pix() }
#[inline] pub fn sf_sin<V: Trig>(x: V) -> V { x.sin() }
#[inline] pub fn sf_sinh_mx<V:Trig>(x:V) -> V { x.sinh_mx() }
#[inline] pub fn sf_sinh<V:Trig>(x:V) -> V { x.sinh() }
#[inline] pub fn sf_tan<V:Trig>(x:V) -> V { x.tan() }
#[inline] pub fn sf_tanh<V:Trig>(x:V) -> V { x.tanh() }
#[inline] pub fn sf_vcos<V:Trig>(x:V) -> V { x.vcos() }
#[inline] pub fn sf_vsin<V:Trig>(x:V) -> V { x.vsin() }

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
