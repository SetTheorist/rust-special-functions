use crate::value::{Value};

trait Trig : Value {
  fn cos(self) -> Self;
  fn acos(self) -> Self;
  fn vcos(self) -> Self;
  fn hvcos(self) -> Self;

  fn sin(self) -> Self;
  fn asin(self) -> Self;
  fn vsin(self) -> Self;
  fn hvsin(self) -> Self;

  fn sec(self) -> Self;
  fn exsec(self) -> Self;
  fn csc(self) -> Self;

  fn tan(self) -> Self;
  fn atan(self) -> Self;
  fn cot(self) -> Self;


  fn cosh(self) -> Self;
  fn acosh(self) -> Self;

  fn sinh(self) -> Self;
  fn asinh(self) -> Self;

  fn tanh(self) -> Self;
  fn atanh(self) -> Self;
  
  fn sech(self) -> Self;
  fn asech(self) -> Self;

  fn gud(self) -> Self;
  fn agud(self) -> Self;


  fn cosh_m1(self) -> Self;
  fn sinh_mx(self) -> Self;
}

// cos(x)
// TODO: range reduce
//sf_cos x = ksum $ ixiter 0 1 $ \n t -> -t*x^2/((#)$2*n+1)/((#)$2*n+2)
pub fn sf_cos(x:f64) -> f64 { x.cos() }

// cos(pi*x)
// TODO: efficient version
//pub fn sf_cos_pi(x:f64) -> f64 { sf_cos(pi*x) }

pub fn sf_acos(x:f64) -> f64 { x.acos() }

// vercosine function
pub fn sf_vcos(x:f64) -> f64 { 2.0 * sf_cos(x/2.0).powi(2) }

// havercosine function
pub fn sf_hvcos(x:f64) -> f64 { sf_cos(x/2.0).powi(2) }

//--------------------------------------
// sin(x)
// TODO: range reduce
//sf_sin x = ksum $ ixiter 1 x $ \n t -> -t*x^2/((#)$2*n)/((#)$2*n+1)
pub fn sf_sin(x:f64) -> f64 { x.sin() }

// sin(pi*x)
// TODO: efficient version
//pub fn sf_sin_pi(x:f64) -> f64 { sf_sin(pi*x) }

pub fn sf_asin(x:f64) -> f64 { x.asin() }

// versine function
pub fn sf_vsin(x:f64) -> f64 { 2.0 * sf_sin(x/2.0).powi(2) }

// haversine function
pub fn sf_hvsin(x:f64) -> f64 { sf_sin(x/2.0).powi(2) }

//--------------------------------------
// secant function
pub fn sf_sec(x:f64) -> f64 { 1.0 / sf_cos(x) }

// exsecant function
pub fn sf_exsec(x:f64) -> f64 { 2.0 * sf_sin(x/2.0).powi(2) * sf_sec(x) }

// cosecant function
pub fn sf_csc(x:f64) -> f64 { 1.0 / sf_sin(x) }

//--------------------------------------
// tangent function
pub fn sf_tan(x:f64) -> f64 { x.tan() }

// arctangent function
pub fn sf_atan(x:f64) -> f64 { x.atan() }

// $$\tan z = z/(1- z^2/(3- z^2/(5- z^2/(7- ...))))$$
// NB terrible convergence
//tan_cf :: (Value v) => v -> v
//tan_cf z = sf_cf_steeds (z:(cycle [-z^2,z^2])) (map (#) (0:[1,3..]))

// cotangent
// = 1/sf_tan x
pub fn sf_cot(x:f64) -> f64 { sf_cos(x) / sf_sin(x) }

/*
//--------------------------------------
// hyperbolic cosine function
pub fn sf_cosh :: (Value v) => v -> v
pub fn sf_cosh = cosh

pub fn sf_acosh :: (Value v) => v -> v
pub fn sf_acosh = acosh

// hyperbolic sine function
pub fn sf_sinh :: (Value v) => v -> v
pub fn sf_sinh = sinh

pub fn sf_asinh :: (Value v) => v -> v
pub fn sf_asinh = asinh

// hyperbolic secant function
pub fn sf_sech :: (Value v) => v -> v
pub fn sf_sech z = 1/sf_cosh z

// hyperbolic tangent function
pub fn sf_tanh :: (Value v) => v -> v
pub fn sf_tanh = tanh

pub fn sf_atanh :: (Value v) => v -> v
pub fn sf_atanh = atanh

//--------------------------------------
// Gudermannian function
pub fn sf_gud :: (Value v) => v -> v
pub fn sf_gud x = sf_asin(sf_tanh x)
// = 2*sf_atan(sf_exp(x)) - pi/2
// = 2*sf_atan(sf_tanh(x/2))
// = sf_atan(sf_sinh(x))

// Compute the inverse Gudermannian function
pub fn sf_agud :: (Value v) => v -> v
pub fn sf_agud z = sf_asinh (sf_tan z)
// = sf_log(abs((1+sf_sin(z))/sf_cos(z)))
// = sf_log(abs((1+sf_sin(z))/(1-sf_sin(z))))/2
// = sf_log(abs(sf_tan(z) + sf_sec(z))
// = sf_log(abs(sf_tan(pi/4 + z/2)))
// = sf_atanh(sf_sin(z))
*/
