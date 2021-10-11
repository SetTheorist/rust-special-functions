use crate::algorithm::sum_series;
use crate::gamma::{sf_gamma, sf_digamma, Gamma};
use crate::traits::*;
use crate::trig::*;
use crate::exp::{sf_exp,Exp};
use crate::log::{sf_log,Log};
use crate::numbers::{sf_factorial_approx};
use super::*;

////////////////////////////////////////////////////////////////////////////////

// TODO: make generic version of the forward / backward recurrence computations

////////////////////////////////////////////////////////////////////////////////

// TODO: clean up handling of nu type
pub fn bessel_j_series_int<V: Value + Gamma + Power>(nu:isize, z:V) -> V {
  let z2 = -(z / 2).sqr();
  let terms = (1..).scan(ι(1): V, |s, m| {
    *s *= z2 / m / (nu + m);
    Some(*s)
  });
  let terms = std::iter::once(ι(1)).chain(terms);
  sum_series(terms, V::epsilon) * (z / 2).pow(nu) / sf_factorial_approx(nu as usize)
}
pub fn bessel_j_series<V: Value + Gamma + Power>(nu:V, z:V) -> V {
  let z2 = -(z / 2).sqr();
  let terms = (1..).scan(ι(1):V, |s, m| {
    *s *= z2 / m / (nu + m);
    Some(*s)
  });
  let terms = std::iter::once(ι(1)).chain(terms);
  sum_series(terms, V::epsilon) * (z / 2).pow(nu) / sf_gamma(nu + 1)
}

// for |z|>>nu, |arg z|<pi
// z needs to be fairly large for this to to be accurate
// TODO: separate type for nu and z
pub fn bessel_j_asymp_z<V:Value+Trig>(nu:V, z:V) -> V {
  let chi = z - (nu / 2 + 0.25) * V::PI;
  let mu = nu.sqr() * 4;
  (ι(2): V / (V::PI * z)).sqrt() * (asymp_even(nu, z) * sf_cos(chi) - asymp_odd(nu, z) * sf_sin(chi))
}
fn asymp_even<V: Value>(nu: V, z: V) -> V {
  let mu = nu.sqr() * 4;
  let mut res: V = ι(1);
  let mut term: V = ι(1);
  let z8 = -(z * 8).sqr();
  for k in 1..1000 {
    let old_term = term;
    term *= (mu - (4 * k - 3).sqr()) * (mu - (4 * k - 1).sqr()) / (z8 * (2 * k - 1) * (2 * k));
    let old_res = res;
    res += term;
    if res == old_res || μ(term) > μ(old_term) {
      res = old_res;
      break;
    }
  }
  res
}
fn asymp_odd<V: Value>(nu: V, z: V) -> V {
  let mu = nu.sqr() * 4;
  let mut res = (mu - 1) / (z * 8);
  let mut term = res;
  let z8 = -(z * 8).sqr();
  for k in 2..1000 {
    let old_term = term;
    term *= (mu - (4 * k - 5).sqr()) * (mu - (4 * k - 3).sqr()) / (z8 * (2 * k - 2) * (2 * k - 1));
    let old_res = res;
    res += term;
    if res == old_res || μ(term) > μ(old_term) {
      res = old_res;
      break;
    }
  }
  res
}

/*
pub fn bessel_j_recur_back_in_order<V:Value>(nu:V, z:V) -> V {
  let n = nu.floor();
  let nuf = nu - n;
  let nx = n + 10;
  unimplemented!()
}
*/

// for integral order (assumed non-negative)
pub fn bessel_j_recur_back<V: Value>(maxm: isize, n: isize, z: V) -> V {
  let mut jjp2 = V::zero;
  let mut jjp1 = V::one;
  let mut scale: V = ι(2);
  let mut res = V::zero;
  for m in (1..=(maxm - 2)).rev() {
    let jjm = -jjp2 + (ι(2): V * m / z) * jjp1;
    jjp2 = jjp1;
    jjp1 = jjm;
    if m == n + 1 {
      // desired value, but keep going to get scale-factor
      res = jjm;
    }
    scale += jjm.sqr() * (if m==1 {1} else {2});
    if abs(scale) > ι(1e20) {
      jjp2 /= 1024;
      jjp1 /= 1024;
      res /= 1024;
      scale /= 1024 * 1024;
    }
  }
  res / scale.sqrt()
}

////////////////////////////////////////////////////////////////////////////////

pub fn bessel_y_series_int<V:Value+BesselJ<isize>+Gamma+Log>(n:isize, z:V) -> V {
  let z22 = (z/2).sqr();
  let mut sum = V::FRAC_1_PI * 2 * sf_log(z/2) * sf_bessel_j(n,z);
  let mut t = -(z/2).pow(-n) * V::FRAC_1_PI * sf_factorial_approx((n-1) as usize);
  for k in 0..n {
    sum += t;
    t *= z22 / (k+1) / (n-k-1);
  }
  let mut t = -(z/2).pow(n) * V::FRAC_1_PI;
  for k in 0..1000 {
    let old_sum = sum;
    sum += t * (sf_digamma(ι(k+1):V) + sf_digamma(ι(n+k+1):V));
    if sum != sum {break;}
    if old_sum == sum {break;}
    t *= -z22 / (k+1) / (n+k+1);
  }
  sum
}

pub fn bessel_y_asymp_z<V:Value+Trig>(nu:V, z:V) -> V {
  let chi = z - (nu / 2 + 0.25) * V::PI;
  let mu = nu.sqr() * 4;
  (ι(2): V / (V::PI * z)).sqrt() * (asymp_even(nu, z) * sf_sin(chi) + asymp_odd(nu, z) * sf_cos(chi))
}

// for large nu>0
pub fn bessel_y_asymp_nu_1<V:Value+Exp+Power>(nu:V, z:V) -> V {
  -sf_sqrt(V::FRAC_1_PI*2/nu)*(V::E*z/(nu*2)).pow(-nu)
}

////////////////////////////////////////////////////////////////////////////////

pub fn bessel_i_series_int<V:Value+Gamma+Power>(nu:isize, z:V) -> V {
  let z2 = (z / 2).sqr();
  let terms = (1..).scan(ι(1): V, |s, m| {
    *s *= z2 / m / (nu + m);
    Some(*s)
  });
  let terms = std::iter::once(ι(1)).chain(terms);
  sum_series(terms, V::epsilon) * (z / 2).pow(nu) / sf_factorial_approx(nu as usize)
}
pub fn bessel_i_series<V:Value+Gamma+Power>(nu:V, z:V) -> V {
  let z2 = (z / 2).sqr();
  let terms = (1..).scan(ι(1): V, |s, m| {
    *s *= z2 / m / (nu + m);
    Some(*s)
  });
  let terms = std::iter::once(ι(1)).chain(terms);
  sum_series(terms, V::epsilon) * (z / 2).pow(nu) / sf_gamma(nu + 1)
}

// assumes nu>=0
// adaptation of Miller's method
// TODO: not clear that when this is the method of choice...
pub fn bessel_i_order_recur<V:Value+Exp>(nu:isize, z:V, j:bool, EXTRA:isize) -> V {
  //const EXTRA : isize = 100; // TODO: need to compute appropriate bounds here...
  let tot = (nu+EXTRA+1) as usize;
  let mut rs = vec![V::zero; tot];
  let iz = z.recip();
  //if j { rs[tot-1] = z/2*(ι(tot as isize-1):V/((tot as isize).pow(2)));}
  if j { rs[tot-1] = z/(2*(tot) as isize); }
  else { rs[tot-1] = iz*(2*(tot) as isize); }
  for j in (0..(tot-1)).rev() {
    rs[j] = (rs[j+1] + iz*(2*(j+1) as isize)).recip();
  }
  let mut numer = sf_exp(z);
  for j in 0..(nu as usize) {
    numer *= rs[j];
  }
  let mut denom = V::one;
  for j in (1..tot).rev() {
    denom = denom * rs[j] + 1;
  }
  denom = denom * 2 * rs[0] + 1;
  numer / denom
}

pub fn bessel_i_asymp_z<V:Value+Exp>(nu:V, z:V) -> V {
  sf_exp(z) / sf_sqrt(V::PI*2*z) * asymp_all(nu, -z)
}


// useful for recurrence normalization:
// \sum_{i=-\infty}^\infty I_{2n+1}(x) = sinh(x)
// \sum_{i=0}^\infty I_{2n+1}(x) = sinh(x)/2
//
// \sum_{i=-\infty}^\infty I_{2n}(x) = cosh(x)
// I_0(x)/2 + \sum_{i=1}^\infty I_{2n}(x) = cosh(x)/2

////////////////////////////////////////////////////////////////////////////////


pub fn bessel_k_series_int<V:Value+BesselI<isize>+Gamma+Log>(n:isize, z:V) -> V {
  let z22 = (z/2).sqr();
  let mut sum = sf_log(z/2) * sf_bessel_i(n,z).pari(n+1);
  let mut t = if n==0 {V::one/2} else {(z/2).pow(-n)/2 * sf_factorial_approx((n-1) as usize)};
  for k in 0..n {
    sum += t;
    t *= -z22 / (k+1) / (n-k-1);
  }
  let mut t = (z/2).pow(n).pari(n)/2;
  for k in 0..1000 {
    let old_sum = sum;
    sum += t * (sf_digamma(ι(k+1):V) + sf_digamma(ι(n+k+1):V));
    if sum != sum {break;}
    if old_sum == sum {break;}
    t *= z22 / (k+1) / (n+k+1);
  }
  sum
}

// requires that nu is NOT an integer
pub fn bessel_k_connection<N:Value+Trig,V:Value+BesselI<N>+Embeds<N>>(nu:N, z:V) -> V {
  V::FRAC_PI_2 * (sf_bessel_i(-nu, z) - sf_bessel_i(nu, z)) / sf_sin_pix(nu)
}

pub fn bessel_k_asymp_z<V:Value+Exp>(nu:V, z:V) -> V {
  sf_sqrt(V::FRAC_PI_2/z) * sf_exp(-z) * asymp_all(nu, z)
}

fn asymp_all<V: Value>(nu: V, z: V) -> V {
  let mu = nu.sqr() * 4;
  let mut res: V = ι(1);
  let mut term: V = ι(1);
  let z8 = (z * 8);
  for k in 1..1000 {
    let old_term = term;
    term *= (mu - (2 * k - 1).sqr()) / (z8 * k);
    let old_res = res;
    res += term;
    if res == old_res || μ(term) > μ(old_term) && ι(k):V::NT > abs(nu) {
      res = old_res;
      break;
    }
  }
  res
}

