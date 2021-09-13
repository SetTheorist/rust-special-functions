use crate::traits::{Value};

pub trait Dawson : Value {
  fn dawson(self) -> Self;
}

pub mod impls {
use crate::algorithm::{contfrac_modlentz, sum_series};
use crate::traits::{*};
use crate::exp::{Exp,sf_exp};

// maybe use for small |z|:
// exp(-z^2)*erf(I*z)*sqrt(pi)/(2*I)

pub fn dawson_contfrac<V:Value+Normed>(x:V) -> V {
  let terms = (1..).map(|j|(-(x*x*(2*j)).pari(j), ι(2*j+1)));
  x / contfrac_modlentz(ι(1), terms, V::epsilon)
}

pub fn dawson_contfrac2<V:Value+Normed>(x:V) -> V {
  let terms = (1..).map(|j|(x*x*(-4*j), x*x*2+(2*j+1)));
  x / contfrac_modlentz(x*x*2 + 1, terms, V::epsilon)
}

pub fn dawson_series<V:Value+Exp>(x:V) -> V {
  let x2 = x.sqr();
  let terms = (1..).scan(x, |s,n|{let o=*s; *s*=x2/n; Some(o/(2*n-1))});
  sf_exp(-x*x) * sum_series(terms, V::epsilon)
}

// doesn't include suggested optimizations
pub fn dawson_rybicki<V:Value+Exp>(x:V) -> V {
  let h : V = ι(0.1);
  let terms = (1..).step_by(2)
    .map(|n|(sf_exp(-(x-h*n).sqr()) - sf_exp(-(x+h*n).sqr()))/n);
  // sqrt(pi) - TODO: use Constants
  let sqpi : V = ι(1.7724538509055160272981674833411451827975494561223871282138077898);
  sum_series(terms, V::epsilon) / sqpi
}

// bessel series (in terms of spherical bessel i1() functions
}
