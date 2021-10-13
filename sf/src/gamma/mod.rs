use crate::traits::*;

pub mod impls;

pub trait Gamma {
  fn lngamma(self) -> Self;
  fn gamma(self) -> Self;
  fn beta(self, b:Self) -> Self;
  fn digamma(self) -> Self;
}

#[inline] pub fn sf_gamma<V:Gamma>(x:V) -> V { x.gamma() }
#[inline] pub fn sf_lngamma<V:Gamma>(x:V) -> V { x.lngamma() }
#[inline] pub fn sf_digamma<V:Gamma>(x:V) -> V { x.digamma() }
#[inline] pub fn sf_beta<V:Gamma>(a:V, b:V) -> V { a.beta(b) }

pub trait IncompleteGamma {
  // $\gamma(a,x) = \int_0^x t^{a-1}e^{-t}\,dx $
  fn gamma_inc(self, x:Self) -> Self;
  // $\Gamma(a,x) = \int_x^\infty t^{a-1}e^{-t}\,dx $
  fn gamma_inc_co(self, x:Self) -> Self;

  // TODO: maybe use better names? e.g. gamma_incomplete_scaled_lower???
  // $ = \frac{\gamma(a,x)}{\Gamma(a)} $
  fn gamma_inc_p(self, x:Self) -> Self;
  // $ = \frac{\Gamma(a,x)}{\Gamma(a)} $
  fn gamma_inc_q(self, x:Self) -> Self;

  fn beta_inc(self, a:Self, b:Self) -> Self;
  fn beta_inc_i(self, a:Self, b:Self) -> Self;
}
#[inline] pub fn sf_gamma_inc<V:IncompleteGamma>(a:V, x:V) -> V { a.gamma_inc(x) }
#[inline] pub fn sf_gamma_inc_co<V:IncompleteGamma>(a:V, x:V) -> V { a.gamma_inc_co(x) }
#[inline] pub fn sf_gamma_inc_p<V:IncompleteGamma>(a:V, x:V) -> V { a.gamma_inc_p(x) }
#[inline] pub fn sf_gamma_inc_q<V:IncompleteGamma>(a:V, x:V) -> V { a.gamma_inc_q(x) }
#[inline] pub fn sf_beta_inc<V:IncompleteGamma>(x:V, a:V, b:V) -> V { x.beta_inc(a, b) }
#[inline] pub fn sf_beta_inc_i<V:IncompleteGamma>(x:V, a:V, b:V) -> V { x.beta_inc_i(a, b) }


////////////////////////////////////////////////////////////////////////////////

// TODO: quick and dirty for now
// TODO: need to get good implementation!
use crate::real::*;
use crate::log::{sf_log, Log};
use crate::exp::{sf_exp, Exp};
use crate::traits::Constants;
use crate::trig::{sf_cos, sf_sin};
impl Gamma for r64 {
  fn gamma(self) -> Self {
    if self < ι(0.5) {
      // gamma(z) = pi/(sin(pi*z) * gamma(1-z))
      return r64::PI / (sf_sin(self * r64::PI) * (1 - self).gamma());
    }
    //sf_exp(impls::lngamma_lanczos_15(self)) // TODO: this has terrible accuracy?!
    //sf_exp(impls::lngamma_lanczos_11(self)) // TODO: this has terrible accuracy?!
    let mut mult = r64(1.0);
    let mut x = self;
    //while x > r64(8.0)
    while x > r64(3.0)
    {
      x = x - 1.0;
      mult *= x;
    }
    //mult * sf_exp(impls::lngamma_lanczos_7(x)) // TODO
    mult * impls::gamma_spouge(11, x)
  }
  fn lngamma(self) -> Self {
    //impls::lngamma_lanczos_15(self) // TODO
    sf_log(impls::gamma_spouge(11, self)) // TODO
  }
  fn digamma(self) -> Self {
    impls::digamma(self)
  }
  fn beta(self, b:Self) -> Self {
    sf_exp(self.lngamma() + b.lngamma() - (self+b).lngamma())
  }
}

// TODO: negative, etc.!
// TODO: these are _VERY_ primitive implementations for now!
impl IncompleteGamma for r64 {
  fn gamma_inc(self, x:Self) -> Self {
    sf_gamma(self) - self.gamma_inc_co(x) // TODO
  }
  fn gamma_inc_co(self, x:Self) -> Self {
    impls::gamma_inc_co_contfrac(self, x) // TODO
  }
  fn gamma_inc_p(self, x:Self) -> Self {
    Self::one - self.gamma_inc_co(x)/sf_gamma(self)  // TODO
  }
  fn gamma_inc_q(self, x:Self) -> Self {
    self.gamma_inc_co(x)/sf_gamma(self)  // TODO
  }

  fn beta_inc(self, a:Self, b:Self) -> Self {
    impls::beta_inc_contfrac(self, a, b) // TODO
  }
  fn beta_inc_i(self, a:Self, b:Self) -> Self {
    self.beta_inc(a, b) / sf_beta(a, b) // TODO
  }
}


// TODO: quick and dirty for now
// TODO: need to get good implementation!
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

// TODO: these are _VERY_ primitive implementations for now!
impl IncompleteGamma for c64 {
  fn gamma_inc(self, x:Self) -> Self {
    sf_gamma(self) - self.gamma_inc_co(x) // TODO
  }
  fn gamma_inc_co(self, x:Self) -> Self {
    impls::gamma_inc_co_contfrac(self, x) // TODO
  }
  fn gamma_inc_p(self, x:Self) -> Self {
    Self::one - self.gamma_inc_co(x)/sf_gamma(self)  // TODO
  }
  fn gamma_inc_q(self, x:Self) -> Self {
    self.gamma_inc_co(x)/sf_gamma(self)  // TODO
  }
  fn beta_inc(self, a:Self, b:Self) -> Self {
    todo!() // TODO: contfrac doesn't work for complex
  }
  fn beta_inc_i(self, a:Self, b:Self) -> Self {
    todo!() // TODO
  }
}
