use crate::traits::Value;

pub trait Dawson: Value {
  fn dawson(self) -> Self;
}

pub mod impls {
  use crate::algorithm::{contfrac_modlentz, sum_series};
  use crate::exp::{sf_exp, Exp};
  use crate::traits::*;

  // maybe use for small |z|:
  // exp(-z^2)*erf(I*z)*sqrt(pi)/(2*I)

  pub fn dawson_contfrac<V: Value + Normed>(x: V) -> V {
    let terms = (1..).map(|j| (-(x * x * (2 * j)).pari(j), ι(2 * j + 1)));
    x / contfrac_modlentz(ι(1), terms, V::mu_epsilon)
  }

  pub fn dawson_contfrac2<V: Value + Normed>(x: V) -> V {
    let terms = (1..).map(|j| (x * x * (-4 * j), x * x * 2 + (2 * j + 1)));
    x / contfrac_modlentz(x * x * 2 + 1, terms, V::mu_epsilon)
  }

  pub fn dawson_series<V: Value + Exp>(x: V) -> V {
    let x2 = x.sqr();
    let terms = (1..).scan(x, |s, n| {
      let o = *s;
      *s *= x2 / n;
      Some(o / (2 * n - 1))
    });
    sf_exp(-x * x) * sum_series(terms, V::mu_epsilon)
  }

  // doesn't include suggested optimizations
  pub fn dawson_rybicki<V: Value + Exp>(x: V) -> V {
    let h: V = ι(0.1);
    let terms = (1..).step_by(2).map(|n| (sf_exp(-(x - h * n).sqr()) - sf_exp(-(x + h * n).sqr())) / n);
    sum_series(terms, V::mu_epsilon) * V::FRAC_1_SQRTPI
  }

  // bessel series (in terms of spherical bessel i1() functions
}
