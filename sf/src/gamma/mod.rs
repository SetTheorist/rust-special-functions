use crate::traits::*;

pub mod impls;

pub trait Gamma {
  fn lngamma(self) -> Self;
  fn gamma(self) -> Self;
  fn beta(self, b:Self) -> Self;
  fn digamma(self) -> Self;
}

#[inline]
pub fn sf_gamma<V: Gamma>(x:V) -> V { x.gamma() }
#[inline]
pub fn sf_lngamma<V: Gamma>(x:V) -> V { x.lngamma() }
#[inline]
pub fn sf_digamma<V: Gamma>(x:V) -> V { x.digamma() }
#[inline]
pub fn sf_beta<V: Gamma>(a:V, b:V) -> V { a.beta(b) }


////////////////////////////////////////////////////////////////////////////////

// TODO: quick and dirty for now
use crate::real::*;
//use crate::log::{Log};
use crate::exp::{sf_exp, Exp};
use crate::traits::Constants;
use crate::trig::{sf_cos, sf_sin};
impl Gamma for r64 {
  fn gamma(self) -> Self {
    if self < ι(0.5) {
      // gamma(z) = pi/(sin(pi*z) * gamma(1-z))
      return r64::PI / (sf_sin(self * r64::PI) * (1 - self).gamma());
    }
    //impls::gamma_spouge(11, self)
    sf_exp(impls::lngamma_lanczos_15(self))
  }
  fn lngamma(self) -> Self {
    impls::lngamma_lanczos_15(self)
    //impls::gamma_spouge(11, self).log() // TODO
  }
  fn digamma(self) -> Self {
    impls::digamma(self)
  }
  fn beta(self, b:Self) -> Self {
    sf_exp(self.lngamma() + b.lngamma() - (self+b).lngamma())
  }
}
// TODO: quick and dirty for now
use crate::complex::*;
impl Gamma for c64 {
  fn gamma(self) -> Self {
    if self.real() < ι(0.5) {
      // gamma(z) = pi/(sin(pi*z) * gamma(1-z))
      return c64::PI / (sf_sin(self * c64::PI) * (1 - self).gamma());
    }
    //impls::gamma_spouge(11, self)
    impls::lngamma_lanczos_15(self).exp()
  }
  fn lngamma(self) -> Self {
    impls::lngamma_lanczos_15(self)
    //impls::gamma_spouge(11, self).log() // TODO
  }
  fn digamma(self) -> Self {
    todo!()
    //impls::digamma(self)
  }
  fn beta(self, b:Self) -> Self {
    sf_exp(self.lngamma() + b.lngamma() - (self+b).lngamma())
  }
}

