use crate::traits::{*};
use crate::log::{sf_log};
use crate::exp::{sf_exp};

pub mod algos;
pub mod impls;
pub use impls::*;

// TODO: split into Trig and TrigExtra (TrigObscure?)

// verF(z) = 2 F(z/2)^2
// coF(z) = F(\pi/2 - z)
// haF(z) = F(z)/2
// exF(z) = F(z)-1

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
  fn cos_pix(self) -> Self { ((self % ι(2):Self)*Self::PI).cos() }
  // cos(x) - 1 (aka excos(x))
  fn cos_m1(self) -> Self { self.cos() - 1 }

  // sine
  fn sin(self) -> Self; // MUST IMPLEMENT
  fn asin(self) -> Self; // MUST IMPLEMENT
  // versine
  fn vsin(self) -> Self { (self/2).sin().sqr()*2 }
  // haversine
  fn hvsin(self) -> Self { (self/2).sin().sqr() }
  // sin(pi*x)
  fn sin_pix(self) -> Self { ((self % ι(2):Self)*Self::PI).sin() }

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

  fn cosh_sinh(self) -> (Self, Self) { (self.cosh(), self.sinh()) }

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
