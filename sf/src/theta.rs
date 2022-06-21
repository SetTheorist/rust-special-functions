

// Jacobi theta functions
pub trait Theta {
  fn theta_1(self, q:Self) -> Self;
  fn theta_2(self, q:Self) -> Self;
  fn theta_3(self, q:Self) -> Self;
  fn theta_4(self, q:Self) -> Self;
}

#[inline] pub fn sf_theta_1<V:Theta>(z:V, q:V) -> V { z.theta_1(q) }
#[inline] pub fn sf_theta_2<V:Theta>(z:V, q:V) -> V { z.theta_2(q) }
#[inline] pub fn sf_theta_3<V:Theta>(z:V, q:V) -> V { z.theta_3(q) }
#[inline] pub fn sf_theta_4<V:Theta>(z:V, q:V) -> V { z.theta_4(q) }

pub mod impls {
use crate::traits::*;
use crate::exp::*;
use crate::log::*;
use crate::trig::*;

use super::*;
use crate::real::*;
impl Theta for r64 {
  fn theta_1(self, q:r64) -> r64 {
    if !q.є(ι(0),ι(1)) {
      ::log::warn!("Domain error Theta::theta_1::<{}>({},{})", std::any::type_name::<Self>(), self, q);
      return r64::nan;
    }
    theta_1_series(self, q)
  }
  fn theta_2(self, q:r64) -> r64 {
    if !q.є(ι(0),ι(1)) {
      ::log::warn!("Domain error Theta::theta_2::<{}>({},{})", std::any::type_name::<Self>(), self, q);
      return r64::nan;
    }
    theta_2_series(self, q)
  }
  fn theta_3(self, q:r64) -> r64 {
    if !q.є(ι(0),ι(1)) {
      ::log::warn!("Domain error Theta::theta_3::<{}>({},{})", std::any::type_name::<Self>(), self, q);
      return r64::nan;
    }
    theta_3_series(self, q)
  }
  fn theta_4(self, q:r64) -> r64 {
    if !q.є(ι(0),ι(1)) {
      ::log::warn!("Domain error Theta::theta_4::<{}>({},{})", std::any::type_name::<Self>(), self, q);
      return r64::nan;
    }
    theta_4_series(self, q)
  }
}

pub fn theta_1_series<V:Value+Trig+Power>(z:V, q:V) -> V {
  let mut res = V::zero;
  let half : V = ι(0.5);
  for n in 0..1000 {
    let qpow = q.pow((half+n).sqr()).pari(n); // can do this more efficiently
    res += qpow * sf_sin(z*(2*n+1));
    if res.abs() + qpow.abs() == res.abs() {
      ::log::debug!("theta_1_series::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, q, n);
      break;
    }
  }
  return res * 2;
}

pub fn theta_2_series<V:Value+Trig+Power>(z:V, q:V) -> V {
  let mut res = V::zero;
  let half : V = ι(0.5);
  for n in 0..1000 {
    let qpow = q.pow((half+n).sqr()); // can do this more efficiently
    res += qpow * sf_cos(z*(2*n+1));
    if res.abs() + qpow.abs() == res.abs() {
      ::log::debug!("theta_2_series::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, q, n);
      break;
    }
  }
  return res * 2;
}

pub fn theta_3_series_xform<V:Value+Exp+Log>(z:V, q:V) -> V {
  // use transform ...
  let phi = -sf_log(q) / V::PI;
  let q_prime = sf_exp(-V::PI/phi);
  let mut res = V::zero;
  for n in 1..1000 {
    let qpow = q_prime.pow(n*n);
    let old = res;
    let term = sf_exp(-V::PI*n*n/phi + z*2*n/phi)*(ι(1):V + sf_exp(-z*4*n/phi))*ι(0.5):V;
    res += term;
    if old == res {
      ::log::debug!("theta_3_series_xform::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, q, n);
      break;
    }
  }
  sf_exp(-z.sqr()/(V::PI*phi) + sf_log_1p(res*2))/sf_sqrt(phi)
}
pub fn theta_3_series<V:Value+Trig>(z:V, q:V) -> V {
  let mut res = V::zero;
  for n in 1..1000 {
    let qpow = q.pow(n*n);
    res += qpow * sf_cos(z*(2*n));
    if res.abs() + qpow.abs() == res.abs() {
      ::log::debug!("theta_3_series::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, q, n);
      break;
    }
  }
  res*2 + 1
}

pub fn theta_4_series<V:Value+Trig>(z:V, q:V) -> V {
  let mut res = V::zero;
  for n in 1..1000 {
    let qpow = q.pow(n*n).pari(n);
    res += qpow * sf_cos(z*(2*n));
    if res.abs() + qpow.abs() == res.abs() {
      ::log::debug!("theta_4_series::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, q, n);
      break;
    }
  }
  res*2 + 1
}

}
