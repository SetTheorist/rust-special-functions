/*
use crate::embed::*;
use crate::exp::{*};
use crate::kahan::{Kahan};
use crate::numbers::{sf_bernoulli_number_approx, sf_factorial_approx};
use crate::value::{*};
//use num::complex::{Complex};

// for values with real part < 1/2, should use reflection first:
// gamma(z) = pi/(sin(pi*z) * gamma(1-z))
pub fn gamma_asympt<V:Value+Ln+Exp>(x:V) -> V {
  let mut z = x;
  // shift z 
  let mut res : Kahan<V> = Kahan::default();
  while z.dabs() < 50.0 {
    res += -sf_ln(z*(z+ι(1)));
    z += ι(2);
  }
  let z = z;

  let ln2pi_2 = ι(0.9189385332046727417803297364056176398613974736377834128171515404); // log(2*pi)/2
  let mut term : V = (z - ι(0.5))*sf_ln(z) - z + ln2pi_2;
  res += term;
  for m in (2..250).step_by(2) {
    let old_term = term;
    term = (ι(sf_bernoulli_number_approx(m)):V) / ((ι(m*(m-1)):V)*z.powi(m-1));
    if term.dabs() > old_term.dabs() { break; }
    let old_res = res;
    res += term;
    if res == old_res { break; }
  }
  sf_exp(res.0) * sf_exp(res.1)
}

#[inline]
fn spouge_c<V:Value+Exp+Ln>(k:isize, a:isize) -> V {
  (ι(if k%2==0{-1}else{1}):V/ι(sf_factorial_approx(k-1)))
    * sf_exp(sf_ln::<V>(ι(-k+a))*(ι(k):V-ι(0.5)))
    * sf_exp(ι(-k+a):V)
}
/*
fn spouge_c(k:isize, a:r64) -> V {
  r64::mone(k-1)/sf_factorial_approx(k-1)
    * sf_exp(sf_ln(a-k)*(k-ι(0.5)))
    * sf_exp(a-k)
}
*/
pub fn gamma_spouge<V:Value+Exp+Ln>(a:isize, z:V) -> V {
  let z = z - ι(1);
  let res : V = sf_exp((z+ι(0.5))*sf_ln(z+ι(a)))*sf_exp(-(z+ι(a)));
  let mut sm = Kahan::<V>::new(
    ι(2.5066282746310005024157652848110452530069867406099383166299235763)); // sqrt(2*\pi)
  for k in 1..=(a-1) {
    sm += spouge_c::<V>(k,a)/(z+ι(k));
  }
  res * sm.0
}

*/
