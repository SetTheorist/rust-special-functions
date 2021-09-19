use crate::traits::*;

pub trait BesselJ<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_j(self, nu:N) -> Self;
  fn bessel_j_ddz(self, nu:N) -> Self {
    (self.bessel_j(nu-1) - self.bessel_j(nu+1))/2
  }
}

// TODO: Hankel^1, Hankel^2 also

pub trait BesselI<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_i(self, nu:N) -> Self;
  fn bessel_i_ddz(self, nu:N) -> Self {
    (self.bessel_i(nu-1) + self.bessel_i(nu+1))/2
  }
}

pub trait BesselK<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_k(self, nu:N) -> Self;
  fn bessel_k_ddz(self, nu:N) -> Self {
    (self.bessel_k(nu-1) + self.bessel_k(nu+1))/2
  }
}

pub trait BesselY<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_y(self, nu:N) -> Self;
  fn bessel_y_ddz(self, nu:N) -> Self {
    (self.bessel_y(nu-1) - self.bessel_y(nu+1))/2
  }
}

pub trait BesselSpherI<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_spher_i1(self, nu:N) -> Self;
  fn bessel_spher_i2(self, nu:N) -> Self;
}

pub trait BesselSpherJ<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_spher_j(self, nu:N) -> Self;
  fn bessel_spher_j_ddz(self, nu:N) -> Self {
    self.bessel_spher_j(nu-1)*(ι(nu):Self/self) - self.bessel_spher_j(nu+1)
  }
}

pub trait BesselSpherK<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_spher_k(self, nu:N) -> Self;
}

pub trait BesselSpherY<N:Additive+Embeds<isize>> : Value+Embeds<N> {
  fn bessel_spher_y(self, nu:N) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

use crate::real::{*};
impl BesselJ<isize> for r64 {
  fn bessel_j(self, nu:isize) -> Self {
    // for n integral, J_n(-z) = (-)^n J(z)
    if self < r64::zero { return (-self).bessel_j(nu).pari(nu); }
    // J_{-n}(z) = (-)^n J_n(z)
    if nu < 0 { return self.bessel_j(-nu).pari(nu); }

    // TODO: clean this up (rough sketch for now)
    if self <= ι(2) {
      impls::bessel_j_series(ι(nu), self)
    } else if self >= ι(15) {
      impls::bessel_j_asymp_z(ι(nu), self)
    } else {
      impls::bessel_j_recur_back(20+2*nu+(self.abs().rint()), nu, self)
    }
  }
}

mod tests{
use super::*;
use crate::real::*;
#[test]
  fn real_symmetry() {
    assert_eq!(r64(1.0).bessel_j(0), r64(-1.0).bessel_j(0));

    assert_eq!(r64(1.0).bessel_j(1), -r64(1.0).bessel_j(-1));
    assert_eq!(r64(1.0).bessel_j(1), -r64(-1.0).bessel_j(1));
    assert_eq!(r64(1.0).bessel_j(1), r64(-1.0).bessel_j(-1));

    assert_eq!(r64(1.0).bessel_j(2), r64(1.0).bessel_j(-2));
    assert_eq!(r64(1.0).bessel_j(2), r64(-1.0).bessel_j(2));
    assert_eq!(r64(1.0).bessel_j(2), r64(-1.0).bessel_j(-2));
  }
}

////////////////////////////////////////////////////////////////////////////////

pub mod impls {
use crate::traits::{*};
use crate::algorithm::{sum_series};
use crate::gamma::{Gamma,sf_gamma};
use crate::trig::{*};

// TODO: separate type for nu and z
// (sf_gamma may be implemented more efficiently, e.g. for integral types)
pub fn bessel_j_series<V:Value+Gamma+Power>(nu:V, z:V) -> V {
  let z2 = -(z/2).sqr();
  let terms = (1..).scan(ι(1):V,|s,m|{*s *= z2/m/(nu+m); Some(*s)});
  let terms = std::iter::once(ι(1)).chain(terms);
  sum_series(terms, V::mu_epsilon) * (z/2).pow(nu) / sf_gamma(nu+1)
}

// for |z|>>nu, |arg z|<pi
// z needs to be fairly large for this to to be accurate
// TODO: separate type for nu and z
pub fn bessel_j_asymp_z<V:Value+Trig>(nu:V, z:V) -> V {
  let chi = z - (nu/2 + 0.25)*V::PI;
  let mu = nu.sqr() * 4;
  (ι(2):V/(V::PI*z)).sqrt() * (asymp_even(nu,z)*sf_cos(chi) - asymp_odd(nu,z)*sf_sin(chi))
}
pub fn bessel_y_asymp_z<V:Value+Trig>(nu:V, z:V) -> V {
  let chi = z - (nu/2 + 0.25)*V::PI;
  let mu = nu.sqr() * 4;
  (ι(2):V/(V::PI*z)).sqrt() * (asymp_even(nu,z)*sf_sin(chi) + asymp_odd(nu,z)*sf_cos(chi))
}
fn asymp_even<V:Value>(nu:V, z:V) -> V {
  let mu = nu.sqr()*4;
  let mut res : V = ι(1);
  let mut term : V = ι(1);
  let z8 = -(z*8).sqr();
  for k in 1..1000 {
    let old_term = term;
    term *= (mu - (4*k-3).sqr()) * (mu - (4*k-1).sqr()) / (z8*(2*k-1)*(2*k));
    let old_res = res;
    res += term;
    if res == old_res || μ(term) > μ(old_term) { res = old_res; break; }
  }
  res
}
fn asymp_odd<V:Value>(nu:V, z:V) -> V {
  let mu = nu.sqr()*4;
  let mut res = (mu - 1) / (z*8);
  let mut term = res;
  let z8 = -(z*8).sqr();
  for k in 2..1000 {
    let old_term = term;
    term *= (mu - (4*k-5).sqr()) * (mu - (4*k-3).sqr()) / (z8*(2*k-2)*(2*k-1));
    let old_res = res;
    res += term;
    if res == old_res || μ(term) > μ(old_term) { res = old_res; break; }
  }
  res
}

/*
pub fn bessel_j_recur_back_in_order<V:Value>(nu:V, z:V) -> V [
  let n = nu.floor();
  let nuf = nu - n;
  let nx = n + 10;
  unimplemented!()
}
*/

// for integral order (assumed non-negative)
pub fn bessel_j_recur_back<V:Value>(maxm:isize, n:isize, z:V) -> V {
  let mut jjp2 = V::zero;
  let mut jjp1 = V::one;
  let mut scale : V = ι(2);
  let mut res = V::zero;
  for m in (1..=(maxm-2)).rev() {
    let jjm = -jjp2 + (ι(2):V * m/z)*jjp1;
    jjp2 = jjp1;
    jjp1 = jjm;
    if m == n+1 {
      // desired value, but keep going to get scale-factor
      res = jjm;
    }
    scale += if m != 1 {jjm.sqr() * 2} else {jjm.sqr() * 1};
    if abs(scale) > ι(1e20) {
      jjp2 /= 1024;
      jjp1 /= 1024;
      res /= 1024;
      scale /= 1024*1024;
    }
  }
  res / scale.sqrt()
}

}


