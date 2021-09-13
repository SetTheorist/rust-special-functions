use crate::traits::{*};

pub trait Erf {
  fn erf(self) -> Self;
  fn erfc(self) -> Self;
}

pub mod impls {
use crate::algorithm::{contfrac_modlentz, sum_series};
use crate::exp::{Exp,sf_exp};
use crate::traits::{*};

pub fn erf_series<V:Value+Exp>(x:V) -> V {
  // 2/sqrt(pi) // TODO: use Constants
  let tqp : V = ι(1.1283791670955125738961589031215451716881012586579977136881714434);
  let x2 = x.sqr()*2;
  let terms = (1..).scan(x,|s,n|{let o=*s; *s*=x2/(2*n+1); Some(o)});
  sf_exp(-x.sqr()) * tqp * sum_series(terms, V::epsilon)
}

pub fn erfc_contfrac<V:Value+Exp>(x:V) -> V {
  //sqrt(pi) // TODO: use Constants
  let sqpi : V = ι(1.7724538509055160272981674833411451827975494561223871282138077898);
  let x2 = x.sqr();
  let terms = (1..).map(|n|(ι(n):V/2, if n%2==1{ι(1)}else{x2}));
  sf_exp(-x2) / sqpi * x / contfrac_modlentz(x2, terms, V::epsilon)
}

pub fn erfc_contfrac2<V:Value+Exp>(x:V) -> V {
  //sqrt(pi) // TODO: use Constants
  let sqpi : V = ι(1.7724538509055160272981674833411451827975494561223871282138077898);
  let x2 = x.sqr()*2;
  let terms = (1..).map(|n|(
    ι(-(2*n-1)*(2*n)):V,
    x2+(4*n+1)));
  sf_exp(-x.sqr()) / sqpi * (x*2) / contfrac_modlentz(x2+1, terms, V::epsilon)
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

