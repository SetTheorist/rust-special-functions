use crate::traits::*;

pub trait Erf {
  fn erf(self) -> Self;
  fn erfc(self) -> Self;
}

pub mod impls {
  use crate::algorithm::{contfrac_modlentz, sum_series};
  use crate::exp::{sf_exp, Exp};
  use crate::traits::*;

  pub fn erf_series<V: Value + Exp>(x: V) -> V {
    let x2 = x.sqr() * 2;
    let terms = (1..).scan(x, |s, n| {
      let o = *s;
      *s *= x2 / (2 * n + 1);
      Some(o)
    });
    sf_exp(-x.sqr()) * (V::FRAC_1_SQRTPI * 2) * sum_series(terms, V::mu_epsilon)
  }

  pub fn erfc_contfrac<V: Value + Exp>(x: V) -> V {
    let x2 = x.sqr();
    let terms = (1..).map(|n| (ι(n): V / 2, if n % 2 == 1 { ι(1) } else { x2 }));
    sf_exp(-x2) * V::FRAC_1_SQRTPI * x / contfrac_modlentz(x2, terms, V::mu_epsilon)
  }

  pub fn erfc_contfrac2<V: Value + Exp>(x: V) -> V {
    let x2 = x.sqr() * 2;
    let terms = (1..).map(|n| (ι(-(2 * n - 1) * (2 * n)): V, x2 + (4 * n + 1)));
    sf_exp(-x.sqr()) * V::FRAC_1_SQRTPI * (x * 2) / contfrac_modlentz(x2 + 1, terms, V::mu_epsilon)
  }
}

/*
pub fn erf<V:Value>(x:V) -> V {
  if x.dabs() < 1.0 {
    erf_series(x)
  } else {
    ι(1) - erfc(x)
  }
}
*/
