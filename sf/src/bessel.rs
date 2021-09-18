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

pub mod impls {
use crate::traits::{*};

pub fn bessel_j_series<V:Value>(z:V, nu:V) -> V {
  let z2 = -(z/2).sqr();
  let terms = (1..).scan(ι(1):V,|s,m|{*s *= z2/m/(nu+m); Some(*s)});
  let mut terms = std::iter::once(ι(1)).chain(terms);
  //sumit(terms, 1e-16) * (z/2)^nu / gamma(nu+1)
  while true && let Some(x) = terms.next() {
  }
  unimplemented!()
}

// for |z|>>nu, |arg z|<pi
pub fn bessel_j_asymp_z<V:Value>(z:V, nu:V) -> V {
  //let chi = z - (nu/2 + 0.25)*PI;
  //let mu = 4 * (nu^2);
  // sqrt(2/(pi*z)) * (asymp_p(z,nu)*sf_cos(chi) - asymp_q(z,nu)*sin(chi))
  unimplemented!()
}

}
/*
fn asymp_p(z:r64, nu:r64) -> r64 {
  let mu = 4*(nu^2);
  let mut res = ι(1);
  let mut term = ι(1);
  let z8 = -(8*z)^2;
  for k in 1..1000 {
    let old_term = term;
    term *= (mu - (2*k-1).sqr()) * (mu - (2*k+1).sqr()^2) / ((2*k-1)*(2*k)*z8);
    let old_res = res;
    res += term;
    if res == old_res || term.abs() > old_term.abs() { break; }
  }
}
*/


