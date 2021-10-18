use crate::traits::*;
//use crate::kahan::{Kahan};
use crate::algorithm::sum_series;

pub trait Log: Value {
  // $\log(x)$
  fn log(self) -> Self;

  // $\log(1+x)$
  fn log_1p(self) -> Self { (self + ι(1): Self).log() }

  // $\log_2(x)$
  fn log2(self) -> Self { unimplemented!() } // TODO
                                             // $\log_10(x)$
  fn log10(self) -> Self { unimplemented!() } // TODO
                                              // $\log_b(x)$
  fn logb(self, _b: Self) -> Self { unimplemented!() } // TODO
}
pub fn sf_log<V: Log>(x: V) -> V { x.log() }
pub fn sf_log_1p<V: Log>(x: V) -> V { x.log_1p() }

////////////////////////////////////////////////////////////////////////////////

use crate::wide::*;
impl Log for Wide {
  #[inline] fn log(self) -> Self { self.log() }
}

////////////////////////////////////////////////////////////////////////////////

pub mod impls {
  use crate::algorithm::{contfrac_modlentz, sum_series, sum_series_};
  use crate::traits::*;

  #[inline]
  pub fn ln1p_power_series_terms<V: Value>(x: V) -> impl Iterator<Item = V> {
    let xx = x / (x + 2);
    let x2 = xx.sqr();
    (0..).scan(xx * 2, move |s, n| {
      let o = *s;
      *s *= x2;
      Some(o / (2 * n + 1))
    })
  }

  #[inline]
  pub fn ln1p_power_series<V: Value>(x: V) -> V {
    let terms = ln1p_power_series_terms(x);
    sum_series(terms, V::epsilon)
  }

  // ln(1+x) for |arg(1+x)|<pi
  #[inline]
  pub fn ln1p_contfrac<V: Value>(x: V) -> V {
    let terms = (2..).map(|n: isize| (x * (n >> 1).pow(2), ι(n)));
    x / contfrac_modlentz(ι(1), terms, V::epsilon)
  }

  macro_rules! power_series {
    ($epsilon:expr; $term:ident; {$($prelude:stmt);*} $n:ident in ($r:expr) { $($body:stmt);*;}) => {{
    let eps : f64 = $epsilon;
    let mut sum = ι(0);
    $($prelude)*
    for $n in $r {
      let old_sum : f64 = sum;
      $($body)*
      sum += $term;
      if (sum - old_sum).abs() <= sum.abs()*eps { break; }
    }
    sum
  }};
  }

  pub fn sf_ln_1p_macroseries(x: f64) -> f64 {
    power_series!(f64::EPSILON; term;
      {let xx = x/(x+2.0);
      let x2 = xx.powi(2);
      let mut t = 2.0*xx}
      n in (0..1000) {
        let term = t/((2*n+1) as f64);
        t *=  x2;
      }
    )
  }
}

pub fn sf_ln_real(x: f64) -> f64 { x.ln() }

// this is 33% faster than the iterator-based implementation above!
// (for the pure power-series part)
// this is unexpected and unfortunate, as we'd prefer the iterator-based
// style for code conciseness
pub fn sf_ln_1p_real(x: f64) -> f64 {
  if x > 0.25 {
    sf_ln_real(1.0 + x)
  } else {
    let xx = x / (x + 2.0);
    let x2 = xx.powi(2);
    let mut s = 0.0;
    let mut t = 2.0 * xx;
    for n in 0..1000 {
      let oldv = s;
      s += t / ((2 * n + 1) as f64);
      if (oldv - s).abs() <= s.abs() * f64::EPSILON {
        break;
      }
      t *= x2;
    }
    s
  }
}
