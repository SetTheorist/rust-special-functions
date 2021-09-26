
pub trait EllipticIntegral {
  // complete elliptic integral of the first kind
  fn ellint_k(self) -> Self;

  // incomplete elliptic integral of the first kind
  fn ellint_f(self, phi:Self) -> Self;
}
pub fn sf_ellint_k<V:EllipticIntegral>(k:V) -> V { k.ellint_k() }
pub fn sf_ellint_f<V:EllipticIntegral>(phi:V, k:V) -> V { k.ellint_f(phi) }

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
}

pub mod impls {
use crate::agm::*;
use crate::log::*;
use crate::trig::*;
use crate::traits::*;

#[inline]
pub fn kc<V:Value>(k:V) -> V {
  sf_sqrt(V::one - k.sqr())
}

pub fn ell_k<V:Value+AGM>(k:V) -> V {
  let a = sf_agm(ι(1), kc(k));
  V::PI / (a*2)
}

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
  let (an,bn,cn,phin) = sf_agm_vec_extra(V::one, kc(k), phi, k);
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


}
