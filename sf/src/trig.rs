use crate::traits::{ι, Value};

// TODO: split into Trig and TrigExtra (TrigObscure?)

// NB lots TODO here
pub trait Trig: Value {
  // cosine
  fn cos(self) -> Self;
  fn acos(self) -> Self;
  // vercosine
  //fn vcos(self) -> Self { (self/ι(2)).cos().sqr()*ι(2) }
  // havercosine
  //fn hvcos(self) -> Self{ (self/ι(2)).cos().sqr() }
  // cos(pi*x)
  //fn cos_pix(self) -> Self { (self*ι(3.1415926535897932384626433)).cos() }
  // cos(x) - 1
  //fn cos_m1(self) -> Self { self.cos() - ι(1) }

  // sine
  fn sin(self) -> Self;
  fn asin(self) -> Self;
  // versine
  //fn vsin(self) -> Self { (self/ι(2)).sin().sqr()*ι(2) }
  // haversine
  //fn hvsin(self) -> Self { (self/ι(2)).sin().sqr() }
  // sin(pi*x)
  //fn sin_pix(self) -> Self { (self*ι(3.1415926535897932384626433)).sin() }

  fn cos_sin(self) -> (Self, Self) { (self.cos(), self.sin()) }

  // tangent
  fn tan(self) -> Self;
  fn atan(self) -> Self;

  /*
  // secant
  fn sec(self) -> Self { self.cos().recip() }
  // exsecant
  fn exsec(self) -> Self { self.vsin() * self.sec() }
  // cosecant
  fn csc(self) -> Self { self.sin().recip() }

  fn atan2(self,x:Self) -> Self;
  // cotangent = 1/tan(x)
  fn cot(self) -> Self { self.cos() / self.sin() }

  // hyperbolic cosine
  fn cosh(self) -> Self;
  fn acosh(self) -> Self;
  // cosh(x) - 1
  fn cosh_m1(self) -> Self { self.cosh() - ι(1) }

  // hyperbolic sine
  fn sinh(self) -> Self;
  fn asinh(self) -> Self;
  // sinh(x)/x
  fn sinh_mx(self) -> Self { self.sinh()/self }

  fn tanh(self) -> Self;
  fn atanh(self) -> Self;

  // hyperbolic secant
  fn sech(self) -> Self { self.cosh().recip() }

  fn asech(self) -> Self;

  // Gudermannian
  // = asin(tanh(x)) = 2 atan(exp(x))-pi/2 = 2 atan(tanh(x/2)) = atan(sinh(x))
  fn gud(self) -> Self { self.tanh().asin() }
  // Inverse Gudermannian
  // = log(abs((1+sin(z))/cos(z))) = log(abs((1+sin(z))/(1-sin(z))))/2
  // = log(abs(tan(z) + sec(z)) = log(abs(tan(pi/4 + z/2)))
  // = atanh(sin(z)) = sinh(tan(x))
  fn agud(self) -> Self { self.tan().asinh() }
  */
}

pub fn sf_cos<V: Trig>(x: V) -> V { x.cos() }
pub fn sf_sin<V: Trig>(x: V) -> V { x.sin() }

// TODO: quick placeholder impl
use crate::real::r64;
impl Trig for r64 {
  fn cos(self) -> Self { r64(self.0.cos()) }
  fn acos(self) -> Self { r64(self.0.acos()) }
  fn sin(self) -> Self { r64(self.0.sin()) }
  fn asin(self) -> Self { r64(self.0.asin()) }
  fn tan(self) -> Self { r64(self.0.tan()) }
  fn atan(self) -> Self { r64(self.0.atan()) }
}

// TODO: quick placeholder impl
use crate::complex::c64;
impl Trig for c64 {
  fn cos(self) -> Self {
    c64 { re: ι(self.re.0.cos() * self.im.0.cosh()), im: ι(-self.re.0.sin() * self.im.0.sinh()) }
  }
  fn acos(self) -> Self { unimplemented!("c64::acos()") }
  fn sin(self) -> Self {
    c64 { re: ι(self.re.0.sin() * self.im.0.cosh()), im: ι(self.re.0.cos() * self.im.0.sinh()) }
  }
  fn asin(self) -> Self { unimplemented!("c64::asin()") }

  fn tan(self) -> Self { unimplemented!("c64::tan()") }
  fn atan(self) -> Self { unimplemented!("c64::atan()") }
}

/*

pub fn sf_acos<V:Trig>(x:V) -> V { x.acos() }
pub fn sf_vcos<V:Trig>(x:V) -> V { x.vcos() }
pub fn sf_hvcos<V:Trig>(x:V) -> V { x.hvcos() }
pub fn sf_cos_pix<V:Trig>(x:V) -> V { x.cos_pix() }
pub fn sf_cos_m1<V:Trig>(x:V) -> V { x.cos_m1() }
pub fn sf_asin<V:Trig>(x:V) -> V { x.asin() }
pub fn sf_vsin<V:Trig>(x:V) -> V { x.vsin() }
pub fn sf_hvsin<V:Trig>(x:V) -> V { x.hvsin() }
pub fn sf_sin_pix<V:Trig>(x:V) -> V { x.sin_pix() }
pub fn sf_sec<V:Trig>(x:V) -> V { x.sec() }
pub fn sf_exsec<V:Trig>(x:V) -> V { x.exsec() }
pub fn sf_csc<V:Trig>(x:V) -> V { x.csc() }
pub fn sf_tan<V:Trig>(x:V) -> V { x.tan() }
pub fn sf_atan<V:Trig>(x:V) -> V { x.atan() }
pub fn sf_cot<V:Trig>(x:V) -> V { x.cot() }
pub fn sf_cosh<V:Trig>(x:V) -> V { x.cosh() }
pub fn sf_acosh<V:Trig>(x:V) -> V { x.acosh() }
pub fn sf_cosh_m1<V:Trig>(x:V) -> V { x.cosh_m1() }
pub fn sf_sinh<V:Trig>(x:V) -> V { x.sinh() }
pub fn sf_asinh<V:Trig>(x:V) -> V { x.asinh() }
pub fn sf_sinh_mx<V:Trig>(x:V) -> V { x.sinh_mx() }
pub fn sf_tanh<V:Trig>(x:V) -> V { x.tanh() }
pub fn sf_atanh<V:Trig>(x:V) -> V { x.atanh() }
pub fn sf_sech<V:Trig>(x:V) -> V { x.sech() }
pub fn sf_asech<V:Trig>(x:V) -> V { x.asech() }
pub fn sf_gud<V:Trig>(x:V) -> V { x.gud() }
pub fn sf_agud<V:Trig>(x:V) -> V { x.agud() }

////////////////////////////////////////////////////////////////////////////////

impl Trig for f64 {
  fn cos(self) -> Self { self.cos() }
  fn acos(self) -> Self { self.acos() }
  fn sin(self) -> Self { self.sin() }
  fn asin(self) -> Self { self.asin() }
  fn tan(self) -> Self { self.acos() }
  fn atan(self) -> Self { self.asin() }
  fn atan2(self,x:Self) -> Self { self.atan2(x) }
  fn cosh(self) -> Self { self.cosh() }
  fn acosh(self) -> Self { self.acosh() }
  fn sinh(self) -> Self { self.sinh() }
  fn asinh(self) -> Self { self.asinh() }
  fn tanh(self) -> Self { self.tanh() }
  fn atanh(self) -> Self { self.atanh() }
  fn asech(self) -> Self { self.recip().acosh() }
}
*/
