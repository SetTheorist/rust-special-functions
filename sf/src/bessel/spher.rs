use crate::traits::*;
use crate::trig::*;
use super::*;

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl BesselSpherJ<isize> for r64 {
  fn bessel_spher_j(self, nu:isize) -> Self {
    if self.is_negreal() {
      j_repos(-self, nu).pari(nu)
    } else {
      j_repos(self, nu)
    }
  }
}

// assumes re(z)>=0
pub fn j_repos<V:Value+Normed+Trig>(z:V, n:isize) -> V {
  // TODO: check n>=0
  if z == 0 {
    if n==0 {V::one} else {V::zero}
  } else if n == 0 {
    j0(z)
  } else if n == 1 {
    j1(z)
  } else if abs(z) >= ι(n):V::NT {
    j_fore(z, n)
  } else {
    j_back(z, n)
  }
}

// j0(z) = sin(z)/z
pub fn j0<V:Value+Normed+Trig>(z:V) -> V {
  // TODO: this can be tightened significantly - use fewer terms
  // (maybe tighten interval - need to test timing vs sin())
  if abs(z) < ι(0.5):V::NT {
    let z2 = -z.sqr();
    let mut t = V::one;
    let mut sum = V::zero;
    for n in 1..=24 {
      sum += t;
      t *= z2/((2*n)*(2*n+1));
    }
    sum
  } else {
    sf_sin(z) / z
  }
}

// j1(z) = sin(z)/z^2 - cos(z)/z
pub fn j1<V:Value+Normed+Trig>(z:V) -> V {
  // TODO: this can be tightened significantly - use fewer terms
  // (maybe tighten interval - need to test timing vs sin())
  if abs(z) < ι(0.5):V::NT {
    let z2 = -z.sqr();
    let mut t = z;
    let mut sum = V::zero;
    for n in 1..=24 {
      sum += t/(2*n+1);
      t *= z2/((2*n)*(2*n+1));
    }
    sum
  } else {
    sf_sin(z)/z.sqr() - sf_cos(z)/z
  }
}

pub fn j_fore<V:Value+Normed+Trig>(z:V, n:isize) -> V {
  let mut jm2 = j0(z);
  let mut jm1 = j1(z);
  for j in 2..=n {
    (jm2, jm1) = (jm1, jm1*(2*j-1)/z - jm2);
  }
  jm1
}

pub fn j_back<V:Value+Normed+Trig>(z:V, n:isize) -> V {
  // TODO: domain check!
  const EXTRA : isize = 20;
  // TODO: no need to actually store this array!
  let tot = (n+1+EXTRA) as usize;
  let mut arr = vec![V::zero; tot];
  arr[tot-1] = V::zero;
  arr[tot-2] = V::one;
  for j in (0..(tot-2)).rev() {
    arr[j] = arr[j+1]*((2*j+3)as isize)/z - arr[j+2];
  }
  // nnn = (0:(n+NN)) .'; scale = sqrt(sum((2*nnn+1).*arr.^2));
  let scale = arr[0] / j0(z);
  arr[n as usize] / scale
}

////////////////////////////////////////////////////////////////////////////////

// fn bessel_spher_i1(self, nu: N) -> Self;
// fn bessel_spher_i2(self, nu: N) -> Self;

