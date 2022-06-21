use sf_impl::exp;
use sf_impl::real::*;

pub trait SFExp {
  fn sf_exp(self) -> Self;
}

/// compute sf_exp(x) $` {} = \exp(x) = e^x `$ 
/// ```math
/// {} = \sum_{n=0}^\infty \frac{x^n}{n!}
/// ```
#[inline] pub fn sf_exp<V:SFExp>(x:V) -> V { x.sf_exp() }

impl SFExp for f64 {
  #[inline] fn sf_exp(self) -> Self { exp::sf_exp(r64(self)).0 }
}
