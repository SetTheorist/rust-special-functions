use crate::traits::*;

pub trait Zeta : Value {
  fn zeta(self) -> Self;
  fn zeta_m1(self) -> Self;
}

pub mod impls {
use crate::traits::*;

pub fn zeta_series_em9<V:Value+Power>(s:V, μeps:V::NT) -> V {
  let terms = (1..).map(|n|(ι(n):V).pow(-s));
  let mut sum = V::zero;
  let mut n = 1;
  let mut old = V::zero;
  for t in terms {
    sum += t;
    let vn:V = ι(n);
    let res = sum
      + vn.pow(-s+1)/(s-1)
      - vn.pow(-s)/2
      + vn.pow(-s-1)*(s/12)
      - vn.pow(-s-3)*(s*(s+1)*(s+2)/720)
      + vn.pow(-s-5)*(s*(s+1)*(s+2)*(s+3)*(s+4)/30240)
      - vn.pow(-s-7)*(s*(s+1)*(s+2)*(s+3)*(s+4)*(s+5)*(s+6)/1209600)
      + vn.pow(-s-9)*(s*(s+1)*(s+2)*(s+3)*(s+4)*(s+5)*(s+6)*(s+7)*(s+8)/239500800)
      ;
    if μ(res-old) <= μ(res)*μeps && n>2 { print!("${}$",n);break; }
    old = res;
    n += 1;
  }
  old
}

}


/*
pub fn zeta_directseries<T:Value>(s:r64) -> r64 {
  let terms = (1..).map(|n|r64((n as f64).powf(-s.0)));
  sumit(terms, 1e-16)
}
// ~half as many terms needed...
pub fn zeta_directseries2(s:r64) -> r64 {
  let terms = (0..).map(|n|r64(((2*n+1) as f64).powf(-s.0)));
  sumit(terms, 1e-16)/(1.0 - 2.0_f64.powf(-s.0))
}
pub fn zeta_directseries_em1(s:r64) -> r64 {
  let terms = (1..).map(|n|r64((n as f64).powf(-s.0)));
  let mut sum = r64(0.0);
  let mut n = 1;
  let mut old_res = ι(0);
  for t in terms {
    sum += t;
    let res = sum + (n as f64).powf(1.0 - s.0)/(s.0 - 1.0);
    if abs(res - old_res) <= abs(res)*1e-16 { print!("${}$",n);break; }
    old_res = res;
    n += 1;
  }
  old_res
}
pub fn zeta_m1_directseries(s:r64) -> r64 {
  let terms = (2..).map(|n|r64((n as f64).powf(-s.0)));
  sumit(terms, 1e-16)
}
*/

