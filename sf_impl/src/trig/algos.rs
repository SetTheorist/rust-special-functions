use crate::algorithm::*;
use crate::traits::*;

pub fn tan_contfrac<V:Value>(z:V) -> V {
  let z2 = -z.sqr();
  let terms = (1..1000).map(|j|(z2,ι(2*j+1):V));
  z / contfrac_modlentz(V::one, terms, V::epsilon)
}

pub fn cos_series<V:Value>(x:V) -> V {
  let x2 = -x.sqr();
  let terms = (1..).scan(V::one, move |s, n| {
    let o = *s;
    *s *= x2 / ((2*n-1)*(2*n));
    Some(o)
  });
  sum_series(terms, V::epsilon)
}

pub fn sin_series<V:Value>(x:V) -> V {
  let x2 = -x.sqr();
  let terms = (1..).scan(x, move |s, n| {
    let o = *s;
    *s *= x2 / ((2*n)*(2*n+1));
    Some(o)
  });
  sum_series(terms, V::epsilon)
}

pub fn range_reduce_pi<V:RealValue+Ordered>(x:V) -> (V,isize) {
  // range-reduce
  let n: isize = (x.abs() / V::NT::PI).floor().rint();
  // TODO: use Kahan/compensated idea to return 2 floats to get exact diff
  let r: V = x - V::PI * n;
  (r, n)
}