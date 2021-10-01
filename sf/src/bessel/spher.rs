use crate::traits::*;
use crate::exp::*;
use crate::trig::*;
use super::*;

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl BesselSpherJ<isize> for r64 {
  fn bessel_spher_j(self, nu:isize) -> Self {
    if self.is_negreal() {
      j_repos(nu, -self).pari(nu)
    } else {
      j_repos(nu, self)
    }
  }
}

// assumes re(z)>=0
pub fn j_repos<V:Value+Normed+Trig>(n:isize, z:V) -> V {
  // TODO: check n>=0
  if z == 0 {
    if n==0 {V::one} else {V::zero}
  } else if n == 0 {
    j0(z)
  } else if n == 1 {
    j1(z)
  } else if abs(z) >= ι(n):V::NT {
    j_fore(n, z)
  } else {
    j_back(n, z)
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

pub fn j_fore<V:Value+Normed+Trig>(n:isize, z:V) -> V {
  let mut jm2 = j0(z);
  let mut jm1 = j1(z);
  for j in 2..=n {
    (jm2, jm1) = (jm1, jm1*(2*j-1)/z - jm2);
  }
  jm1
}

pub fn j_back<V:Value+Normed+Trig>(n:isize, z:V) -> V {
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
  // nnn = (0:(n+NN)).'; scale = sqrt(sum((2*nnn+1).*arr.^2));
  let scale = arr[0] / j0(z);
  arr[n as usize] / scale
}

////////////////////////////////////////////////////////////////////////////////
// fn bessel_spher_i1(self, nu: N) -> Self;
// fn bessel_spher_i2(self, nu: N) -> Self;

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl BesselSpherK<isize> for r64 {
  fn bessel_spher_k(self, nu:isize) -> Self {
    if self.is_negreal() {
      //-V::PI/2 * (spher_i1(n, -z) + spher_i2(n, -z))
      todo!("bessel_spher_k for negative z")
    } else {
      spher_k_real(nu, self)
    }
  }
}


// TODO: domain check n>=0
// assumes z>=0
pub fn spher_k_real<V:Value+Float+Exp>(n:isize, z:V) -> V {
  // sf_sqrt(V::PI/(z*2))*sf_bessel_k(n+0.5, z)
  if z == 0 {
    V::infinity
  } else if n == 0 {
    k0(z)
  } else if n == 1 {
    k1(z)
  } else {
    k_fore(n, z)
  }
}

pub fn k0<V:Value+Exp>(z:V) -> V {
  V::PI/2 * sf_exp(-z) / z
}

pub fn k1<V:Value+Exp>(z:V) -> V {
  let iz = z.recip();
  V::PI/2 * sf_exp(-z) * (iz + 1)*iz
}

// n>0
pub fn k_fore<V:Value+Exp>(n:isize, z:V) -> V {
  let mut am2 = k0(z);
  let mut am1 = -k1(z);
  for j in 2..=n {
    (am2, am1) = (am1, am2 - am1*(2*j-1)/z);
  }
  am1.pari(n)
}

// n>0
// TODO: don't actually need the array
pub fn k_back<V:Value+Exp>(n:isize, z:V) -> V {
  const EXTRA : usize = 10;
  let tot = (n as usize)+1+EXTRA;
  let mut arr = vec![V::zero; tot as usize];
  arr[tot-2] = V::one;
  for j in (0..(tot-2)).rev() {
    arr[j] = arr[j+1]*((2*j+3) as isize)/z + arr[j+2];
  }
  // nnn=(0:(n+NN)).';scale = sqrt(sum((2*nnn+1).*arr.^2));
  let scale = arr[0] / k0(z);
  (arr[n as usize] / scale).pari(n)
}

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl BesselSpherY<isize> for r64 {
  fn bessel_spher_y(self, nu:isize) -> Self {
    if self.is_negreal() {
      spher_y_real(nu, -self).pari(nu+1)
    } else {
      spher_y_real(nu, self)
    }
  }
}

// assume z>0, n>=0
pub fn spher_y_real<V:Value+Trig+Float>(n:isize, z:V) -> V {
  // sf_sqrt(V::PI/(z*2)) * sf_bessel_y(n+0.5, z)
  if z == 0 {
    -V::infinity
  } else if n == 0 {
    y0(z)
  } else if n == 1 {
    y1(z)
  } else {
    y_fore(n, z)
  }
}

pub fn y0<V:Value+Trig>(z:V) -> V {
  -sf_cos(z)/z
}

pub fn y1<V:Value+Trig>(z:V) -> V {
  -(sf_cos(z)/z + sf_sin(z))/z
}

pub fn y_fore<V:Value+Trig>(n:isize, z:V) -> V {
  let mut am2 = y0(z);
  let mut am1 = y1(z);
  for j in 2..=n {
    (am2, am1) = (am1, am1*(2*j-1)/z - am2);
  }
  am1
}

// TODO: remove use of array
pub fn y_back<V:Value+Trig>(n:isize, z:V) -> V {
  const EXTRA : usize = 10;
  let tot = EXTRA + 1 + (n as usize);
  let mut arr = vec![V::zero; tot];
  arr[tot-2] = V::one;
  for j in (0..(tot-2)).rev() {
    arr[j] = arr[j+1]*((2*j+3) as isize)/z - arr[j+2];
  }
  //nnn = (0:(n+NN)) .'; #scale = sqrt(sum((2*nnn+1) .* arr.^2));
  let scale = arr[0] / y0(z);
  arr[n as usize] / scale
}
