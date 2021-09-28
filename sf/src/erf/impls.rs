use crate::algorithm::{contfrac_modlentz, sum_series};
use crate::exp::{sf_exp, Exp};
use crate::traits::*;
use super::*;

pub fn erf_series<V: Value + Exp>(x: V) -> V {
  let x2 = x.sqr() * 2;
  let terms = (1..).scan(x, |s, n| {
    let o = *s;
    *s *= x2 / (2 * n + 1);
    Some(o)
  });
  sf_exp(-x.sqr()) * (V::FRAC_1_SQRTPI * 2) * sum_series(terms, V::epsilon)
}

pub fn erfc_contfrac<V: Value + Exp>(x: V) -> V {
  let x2 = x.sqr();
  let terms = (1..).map(|n| (ι(n): V / 2, if n % 2 == 1 { ι(1) } else { x2 }));
  sf_exp(-x2) * V::FRAC_1_SQRTPI * x / contfrac_modlentz(x2, terms, V::epsilon)
}

pub fn erfc_contfrac2<V: Value + Exp>(x: V) -> V {
  let x2 = x.sqr() * 2;
  let terms = (1..).map(|n| (ι(-(2 * n - 1) * (2 * n)): V, x2 + (4 * n + 1)));
  sf_exp(-x.sqr()) * V::FRAC_1_SQRTPI * (x * 2) / contfrac_modlentz(x2 + 1, terms, V::epsilon)
}

use crate::real::{*};
// TODO: quick-crude for now; replace with better approach
impl Erf for r64 {
  fn erf(self) -> r64 {
    if self.abs() < r64::one {
      impls::erf_series(self)
    } else {
      r64::one - impls::erfc_contfrac2(self)
    }
  }
  fn erfc(self) -> r64 {
    if self.abs() < r64::one {
      r64::one - impls::erf_series(self)
    } else {
      impls::erfc_contfrac2(self)
    }
  }
}

use crate::complex::{*};
// TODO: quick-crude for now; replace with better approach
impl Erf for c64 {
  fn erf(self) -> c64 {
    if μ(self) < r64::one {
      impls::erf_series(self)
    } else {
      c64::one - impls::erfc_contfrac2(self)
    }
  }
  fn erfc(self) -> c64 {
    if μ(self) < r64::one {
      c64::one - impls::erf_series(self)
    } else {
      impls::erfc_contfrac2(self)
    }
  }
}

/*
pub fn erf_ss(x:r64) -> r64 {
  let tqp = r64(1.1283791670955125738961589031215451716881012586579977136881714434); // 2/sqrt(pi)
  let terms = (1..1000).scan(x,|s,n|{*s*=2*x*x/(2*n+1);Some(*s)});
  (x+sumit(terms,1e-16)) * eps2(-x*x) * tqp
}
*/

