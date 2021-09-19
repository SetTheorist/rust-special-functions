use crate::traits::*;

pub mod impls;
mod tests;

//
//
//
pub trait BesselJ<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_j(self, nu: N) -> Self;
  fn bessel_j_ddz(self, nu: N) -> Self { (self.bessel_j(nu - 1) - self.bessel_j(nu + 1)) / 2 }
}
pub fn sf_bessel_j<N, V>(nu: N, z: V) -> V
where
  V: BesselJ<N>,
  N: Additive + Embeds<isize>,
{
  z.bessel_j(nu)
}
pub fn sf_bessel_j_ddz<N, V>(nu: N, z: V) -> V
where
  V: BesselJ<N>,
  N: Additive + Embeds<isize>,
{
  z.bessel_j_ddz(nu)
}

// TODO: Hankel^1, Hankel^2 also

//
//
//
pub trait BesselI<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_i(self, nu: N) -> Self;
  fn bessel_i_ddz(self, nu: N) -> Self { (self.bessel_i(nu - 1) + self.bessel_i(nu + 1)) / 2 }
}

//
//
//
pub trait BesselK<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_k(self, nu: N) -> Self;
  fn bessel_k_ddz(self, nu: N) -> Self { (self.bessel_k(nu - 1) + self.bessel_k(nu + 1)) / 2 }
}

//
//
//
pub trait BesselY<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_y(self, nu: N) -> Self;
  fn bessel_y_ddz(self, nu: N) -> Self { (self.bessel_y(nu - 1) - self.bessel_y(nu + 1)) / 2 }
}

//
//
//
pub trait BesselSpherI<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_i1(self, nu: N) -> Self;
  fn bessel_spher_i2(self, nu: N) -> Self;
}

//
//
//
pub trait BesselSpherJ<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_j(self, nu: N) -> Self;
  fn bessel_spher_j_ddz(self, nu: N) -> Self {
    self.bessel_spher_j(nu - 1) * (ι(nu): Self / self) - self.bessel_spher_j(nu + 1)
  }
}

//
//
//
pub trait BesselSpherK<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_k(self, nu: N) -> Self;
}

//
//
//
pub trait BesselSpherY<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_y(self, nu: N) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

pub mod real_impls {
  use super::*;
  use crate::real::*;

  impl BesselJ<isize> for r64 {
    fn bessel_j(self, nu: isize) -> Self {
      // for n integral, J_n(-z) = (-)^n J(z)
      if self < r64::zero {
        return (-self).bessel_j(nu).pari(nu);
      }
      // J_{-n}(z) = (-)^n J_n(z)
      if nu < 0 {
        return self.bessel_j(-nu).pari(nu);
      }

      // TODO: clean this up (rough sketch for now)
      if self <= ι(2) {
        impls::bessel_j_series(ι(nu), self)
      } else if self >= ι(15) {
        impls::bessel_j_asymp_z(ι(nu), self)
      } else {
        impls::bessel_j_recur_back(20 + 2 * nu + (self.abs().rint()), nu, self)
      }
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
