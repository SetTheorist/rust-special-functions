
pub trait EllipticIntegral : Value {
  // complete elliptic integral of the first kind
  fn ellint_k(self) -> Self;
  // complete complementary elliptic integral of the first kind
  fn ellint_kc(self) -> Self { sf_kc(self).ellint_k() }
  // incomplete elliptic integral of the first kind
  fn ellint_f(self, phi:Self) -> Self;

  // complete elliptic integral of the second kind
  fn ellint_e(self) -> Self;
  // complete complementary elliptic integral of the second kind
  fn ellint_ec(self) -> Self { sf_kc(self).ellint_e() }
  // incomplete elliptic integral of the second kind
  fn ellint_e_inc(self, phi:Self) -> Self;
}

#[inline]
pub fn sf_ellint_k<V:EllipticIntegral>(k:V) -> V { k.ellint_k() }
#[inline]
pub fn sf_ellint_kc<V:EllipticIntegral>(k:V) -> V { k.ellint_kc() }
#[inline]
pub fn sf_ellint_f<V:EllipticIntegral>(phi:V, k:V) -> V { k.ellint_f(phi) }
#[inline]
pub fn sf_ellint_e<V:EllipticIntegral>(k:V) -> V { k.ellint_e() }
#[inline]
pub fn sf_ellint_ec<V:EllipticIntegral>(k:V) -> V { k.ellint_ec() }
#[inline]
pub fn sf_ellint_e_inc<V:EllipticIntegral>(phi:V, k:V) -> V { k.ellint_e_inc(phi) }

#[inline]
pub fn sf_kc<V:Value>(k:V) -> V {
  sf_sqrt(V::one - k.sqr())
}

use crate::traits::*;
use crate::agm::*;
use crate::real::*;
use crate::trig::*;
impl EllipticIntegral for r64 {
  fn ellint_k(self) -> Self {
    if (ι(1):r64 - self*self).is_negreal()  {
      r64::nan
    } else {
      impls::ell_k(self)
    }
  }
  fn ellint_f(self, phi:Self) -> Self {
    if !phi.є(-r64::FRAC_PI_2, r64::FRAC_PI_2) {
      r64::nan
    } else if (ι(1):r64 - (self*sf_sin(phi)).sqr()).is_negreal()  {
      r64::nan
    } else {
      impls::ell_f(phi,self)
    }
  }
  fn ellint_e(self) -> Self {
    impls::ell_e(self)
  }
  fn ellint_e_inc(self, phi:Self) -> Self {
    // TODO: domain checking
    impls::ell_e_incomplete(phi, self)
  }
}

pub mod impls {
use super::*;
use crate::agm::*;
use crate::log::*;
use crate::trig::*;
use crate::traits::*;


pub fn ell_k<V:Value+AGM>(k:V) -> V {
  let a = sf_agm(ι(1), sf_kc(k));
  V::PI / (a*2)
}

////////////////////////////////////////

pub fn ell_f<V:Value+Log+Trig>(phi:V, k:V) -> V {
  if k == 1 {
    sf_log((sf_sin(phi)+1)/(sf_sin(-phi)+1))/2
  } else if k == 0 {
    phi
  } else if phi == 0 {
    ι(0)
  } else {
    f_ascending_landen(phi, k)
  }
}

pub fn f_agm_method<V:Value+AGM>(phi:V, k:V) -> V {
  let (an,bn,cn,phin) = sf_agm_vec_extra(V::one, sf_kc(k), phi, k);
  let n = phin.len();
  phin[n-1] / (an[n-1]<<((n-1) as isize))
}

pub fn f_ascending_landen<V:Value+Log+Trig>(phi:V, k:V) -> V {
  let mut res = V::one;
  let mut k = k;
  let mut phi = phi;
  for n in 0..1000 {
    let k2 = sf_sqrt(k) * 2 / (k + 1);
    let phi2 = (phi + sf_asin(k * sf_sin(phi))) / 2;
    res *= ι(2):V / (k + 1);
    k = k2;
    phi = phi2;
    if k == ι(1):V {break;}
  }
  res *= sf_log((sf_sin(phi)+1) / (sf_sin(-phi)+1)) / 2;
  return res;
}

////////////////////////////////////////

pub fn ell_e<V:Value+AGM>(k:V) -> V {
  // TODO: domain
  let (va,_,vc) = sf_agm_vec(V::one, sf_kc(k), k);
  let n = vc.len();
  let mut res : V = -k.sqr() + 2;
  for i in 1..n {
    res -= vc[i].sqr() << (i as isize);
  }
  res * V::PI / (va[n-1]*4)
}

pub fn ell_e_incomplete<V:Value+Log+Trig>(phi:V, k:V) -> V {
  // TODO: domain
  if k == 1 {
    sf_sin(phi)
  } else if k == 0 {
    phi
  } else {
    e_ascending_landen(phi, k)
  }
}

// TODO: transform recursion into iteration
pub fn e_ascending_landen<V:Value+Log+Trig>(phi:V, k:V) -> V {
  if k == 1 {
    sf_sin(phi)
  } else {
    let k2 = sf_sqrt(k)*2/(k+1);
    let phi2 = (phi + sf_asin(k * sf_sin(phi)))/2;
    (k + 1)*e_ascending_landen(phi2, k2) + (-k + 1)*ell_f(phi2, k2) - k*sf_sin(phi)
  }
}

// TODO: buggy!
pub fn e_agm<V:Value+AGM+Trig>(phi:V, k:V) -> V {
  let (va,vb,vc,vphi) = sf_agm_vec_extra(V::one, sf_kc(k), k, phi);
  let n = vphi.len();
  let a = va[n-1];
  let ph = vphi[n-1];
  let mut cphi = V::zero;
  for i in 1..n { cphi += vc[i] * sf_sin(vphi[i]) };
  let mut c2 = V::zero;
  for i in 0..n { c2 += vc[i] << (i as isize); }
  ph / (a << ((n-1) as isize)) + cphi - (ph/(a<<((n+1) as isize))) * c2
}

////////////////////////////////////////

}
