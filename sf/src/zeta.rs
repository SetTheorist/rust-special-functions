use crate::traits::*;

pub trait Zeta : Value {
  pub fn zeta(self) -> Self;
  pub fn zeta_m1(self) -> Self;
}

pub mod impls {
use crate::traits::*;

/*
pub fn zeta_series_em9<T:Value>(s:T) -> T {
  let terms = (1..).map(|n|r64((n as f64).powf(-s.0)));
  let mut sum = r64(0.0);
  let mut n = 1;
  let mut old_res = ι(0);
  for t in terms {
    sum += t;
    let res = sum
      + (n as f64).powf(1.0 - s.0)/(s.0 - 1.0)
      - (n as f64).powf(-s.0)/2.0
      + (n as f64).powf(-s.0-1.0)*(s.0/12.0)
      - (n as f64).powf(-s.0-3.0)*(s.0*(s.0+1.0)*(s.0+2.0)/720.0)
      + (n as f64).powf(-s.0-5.0)*(s.0*(s.0+1.0)*(s.0+2.0)*(s.0+3.0)*(s.0+4.0)/30240.0)
      - (n as f64).powf(-s.0-7.0)*(s.0*(s.0+1.0)*(s.0+2.0)*(s.0+3.0)*(s.0+4.0)*(s.0+5.0)*(s.0+6.0)/1209600.0)
      + (n as f64).powf(-s.0-9.0)*(s.0*(s.0+1.0)*(s.0+2.0)*(s.0+3.0)*(s.0+4.0)*(s.0+5.0)*(s.0+6.0)*(s.0+7.0)*(s.0+8.0)/239500800.0)
      ;
    if abs(res - old_res) <= abs(res)*1e-16 && n>2 { print!("${}$",n);break; }
    old_res = res;
    n += 1;
  }
  old_res
}
*/

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

