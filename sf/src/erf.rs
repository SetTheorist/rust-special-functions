/*
use crate::value::{Value};
use crate::embed::{ι};
use crate::exp::{Exp,sf_exp};

pub trait Erf {
  fn erf(self) -> Self;
  fn erfc(self) -> Self;
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

pub fn erf_series<V:Value+Exp>(x:V) -> V {
  let tqp : V = ι(1.1283791670955125738961589031215451716881012586579977136881714434); // 2/sqrt(pi)
  let mut sum : V = ι(0);
  let mut term = x;
  for n in 1..1000 {
    let old = sum;
    sum += term;
    if old == sum { print!("<{}>",n);break; }
    term *= x*x*ι(2) / ι(2*n+1);
  }
  sum * sf_exp(-x*x) * tqp
}

pub fn erfc_series<V:Value+Exp>(x:V) -> V {
  //let tqp : V = ι(1.1283791670955125738961589031215451716881012586579977136881714434); // 2/sqrt(pi)
  unimplemented!("{:?}",x)
}

/*
pub fn erfc_asympt<V:Value+Exp>(x:V) -> V {
  let qpi : V = ι(); // sqrt(pi)

}
*/

*/

