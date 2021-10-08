use crate::traits::*;

pub mod impls;
mod tests;
pub mod spher;

//
//
//
pub trait BesselJ<N: Additive+Embeds<isize>>: Value + Embeds<N> {
  fn bessel_j(self, nu: N) -> Self;
  fn bessel_j_ddz(self, nu: N) -> Self {
    (self.bessel_j(nu - 1) - self.bessel_j(nu + 1)) / 2
  }
}

#[inline]
pub fn sf_bessel_j<N, V:BesselJ<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_j(nu) }
#[inline]
pub fn sf_bessel_j_ddz<N, V:BesselJ<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_j_ddz(nu) }

// TODO: Hankel^1, Hankel^2 also

//
//
//
pub trait BesselI<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_i(self, nu: N) -> Self;
  fn bessel_i_ddz(self, nu: N) -> Self {
    (self.bessel_i(nu - 1) + self.bessel_i(nu + 1)) / 2
  }
}
#[inline]
pub fn sf_bessel_i<N, V:BesselI<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_i(nu) }
#[inline]
pub fn sf_bessel_i_ddz<N, V:BesselI<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_i_ddz(nu) }

//
//
//
pub trait BesselK<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_k(self, nu: N) -> Self;
  fn bessel_k_ddz(self, nu: N) -> Self {
    (self.bessel_k(nu - 1) + self.bessel_k(nu + 1)) / 2
  }
}
#[inline]
pub fn sf_bessel_k<N, V:BesselK<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_k(nu) }
#[inline]
pub fn sf_bessel_k_ddz<N, V:BesselK<N>>(nu: N, z: V) -> V where N:Additive+Embeds<isize>,
{ z.bessel_k_ddz(nu) }

//
//
//
pub trait BesselY<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_y(self, nu: N) -> Self;
  fn bessel_y_ddz(self, nu: N) -> Self {
    (self.bessel_y(nu - 1) - self.bessel_y(nu + 1)) / 2
  }
}


// TODO: maybe remove genericity in N for spherical bessel functions
// (force them to be only isize?)

//
//
//
pub trait BesselSpherI<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_i1(self, nu: N) -> Self;
  fn bessel_spher_i2(self, nu: N) -> Self;
}
#[inline]
pub fn sf_bessel_spher_i1<N, V:BesselSpherI<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_i1(nu) }
#[inline]
pub fn sf_bessel_spher_i2<N, V:BesselSpherI<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_i2(nu) }

//
//
//
pub trait BesselSpherJ<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_j(self, nu: N) -> Self;
  fn bessel_spher_j_ddz(self, nu: N) -> Self {
    self.bessel_spher_j(nu - 1) * (ι(nu): Self / self) - self.bessel_spher_j(nu + 1)
  }
}
#[inline]
pub fn sf_bessel_spher_j<N, V:BesselSpherJ<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_j(nu) }
#[inline]
pub fn sf_bessel_spher_j_ddz<N, V:BesselSpherJ<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_j_ddz(nu) }

//
//
//
pub trait BesselSpherK<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_k(self, nu: N) -> Self;
}
#[inline]
pub fn sf_bessel_spher_k<N, V:BesselSpherK<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_k(nu) }

//
//
//
pub trait BesselSpherY<N: Additive + Embeds<isize>>: Value + Embeds<N> {
  fn bessel_spher_y(self, nu: N) -> Self;
}
pub fn sf_bessel_spher_y<N, V:BesselSpherY<N>>(nu:N, z:V) -> V
  where N:Additive+Embeds<isize> { z.bessel_spher_y(nu) }

////////////////////////////////////////////////////////////////////////////////

pub mod real_impls {
  use super::*;
  use crate::real::*;

  impl BesselJ<isize> for r64 {
    fn bessel_j(self, nu: isize) -> Self {
      // for n integral, J_n(-z) = (-)^n J_n(z)
      if self < r64::zero {
        return (-self).bessel_j(nu).pari(nu);
      }
      // J_{-n}(z) = (-)^n J_n(z)
      if nu < 0 {
        return self.bessel_j(-nu).pari(nu);
      }

      // TODO: clean this up (rough sketch for now)
      if self <= ι(2) {
        impls::bessel_j_series_int(nu, self)
      } else if self >= ι(15) {
        impls::bessel_j_asymp_z(ι(nu), self)
      } else {
        impls::bessel_j_recur_back(20 + 2 * nu + (self.abs().rint()), nu, self)
      }
    }
  }

  impl BesselI<isize> for r64 {
    fn bessel_i(self, nu: isize) -> Self {
      // for n integral, I_n(-z) = (-)^n I_n(z)
      if self < r64::zero {
        return (-self).bessel_i(nu).pari(nu);
      }
      // I_{-n}(z) = I_n(z)
      if nu < 0 {
        return self.bessel_i(-nu);
      }

      // TODO: need asymptotic, etc.
      impls::bessel_i_series_int(nu, self)
    }
  }
  impl BesselK<isize> for r64 {
    fn bessel_k(self, nu: isize) -> Self {
      if self == 0 {
        return r64::infinity;
      }
      if self.is_negreal() {
        return r64::nan;
      }
      // K_{-n}(z) = K_{n}(z)
      if nu < 0 {
        return self.bessel_k(-nu);
      }
      // TODO: this has unacceptably low accuracy
      impls::bessel_k_series_int(nu, self)
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
