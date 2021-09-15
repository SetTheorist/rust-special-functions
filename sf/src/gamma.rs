use crate::traits::*;

pub trait Gamma : Value {
  fn lngamma(self) -> Self;
  fn gamma(self) -> Self;
}


pub mod impls {
use crate::algorithm::{contfrac_modlentz, sum_series};
use crate::exp::{Exp,sf_exp};
use crate::log::{Log,sf_log};
use crate::numbers::{sf_factorial_approx};
use crate::traits::{*};

#[inline]
fn spouge_c<V:Value+Exp+Log>(k:isize, a:V) -> V {
  let x1 : V = (ι(1):V/sf_factorial_approx(k-1)).pari(k+1);
  let x2 : V = sf_exp(sf_log(a-k) * (ι(k):V - 0.5));
  let x3 : V = sf_exp(a-k);
  x1 * x2 * x3
}
pub fn gamma_spouge<V:Value+Exp+Log>(a:isize, z:V) -> V {
  let z = z - 1;
  let res : V = sf_exp((z+0.5)*sf_log(z+a))*sf_exp(-(z+a));
  let mut sm : V = 
    ι(2.5066282746310005024157652848110452530069867406099383166299235763); // sqrt(2*\pi)
  for k in 1..=(a-1) {
    sm += spouge_c(k,ι(a)):V/(z+k);
  }
  res * sm
}

////////////////////////////////////////////////////////////////////////////////

/*
 g=7, n=9
0 	0.99999999999980993227684700473478
1 	676.520368121885098567009190444019
2 	-1259.13921672240287047156078755283
3 	771.3234287776530788486528258894
4 	-176.61502916214059906584551354
5 	12.507343278686904814458936853
6 	-0.13857109526572011689554707
7 	9.984369578019570859563e-6
8 	1.50563273514931155834e-7
*/

/*
g=9, n=10
0 	1.000000000000000174663
1 	5716.400188274341379136
2 	-14815.30426768413909044
3 	14291.49277657478554025
4 	-6348.160217641458813289
5 	1301.608286058321874105
6 	-108.1767053514369634679
7 	2.605696505611755827729
8 	-0.7423452510201416151527e-2
9 	0.5384136432509564062961e-7
10 	-0.4023533141268236372067e-8
*/

const LANCZOS_15 : [(isize,f64); 15] = [
  (0, 0.99999999999999709182),
  (1, 57.156235665862923517),
  (2, -59.597960355475491248),
  (3, 14.136097974741747174),
  (4, -0.49191381609762019978),
  (5, 0.33994649984811888699e-4),
  (6, 0.46523628927048575665e-4),
  (7, -0.98374475304879564677e-4),
  (8, 0.15808870322491248884e-3),
  (9, -0.21026444172410488319e-3),
  (10, 0.21743961811521264320e-3),
  (11, -0.16431810653676389022e-3),
  (12, 0.84418223983852743293e-4),
  (13, -0.26190838401581408670e-4),
  (14, 0.36899182659531622704e-5)];
const LANCZOS_G_15 : f64 = 4.7421875;
// use for z>1/2 (otherwise use standard reflection)
pub fn lngamma_lanczos_15<V:Value+Exp+Log>(z:V) -> V {
  // log(sqrt(2*Pi))
  const LQ2P : f64 = 0.9189385332046727417803297364056176398613974736377834128171515404;
  const PI : f64 = 3.1415926535897932384626433832795028841971693993751058209749445923;
  let z = z - 1;
  let base = z + LANCZOS_G_15 + 0.5;
  let mut sum : V = ι(0);
  for &(i,c) in LANCZOS_15[1..15].iter().rev() {
    sum += ι(c):V / (z + i);
  }
  sum += LANCZOS_15[0].1;
  ((sf_log(sum) + LQ2P) - base) + sf_log(base)*(z + 0.5)
}

const LANCZOS_7 : [(isize,f64); 7] = [
  (0, 1.000000000190015),
  (1, 76.18009172947146),
  (2, -86.50532032941677),
  (3, 24.01409824083091),
  (4, -1.231739572450155),
  (5, 0.1208650973866179e-2),
  (6, -0.5395239384953e-5)];
const LANCZOS_G_7 : f64 = 5.0;
// use for z>1/2 (otherwise use standard reflection)
pub fn lngamma_lanczos_7<V:Value+Exp+Log>(z:V) -> V {
  // log(sqrt(2*Pi))
  const LQ2P : f64 = 0.9189385332046727417803297364056176398613974736377834128171515404;
  const PI : f64 = 3.1415926535897932384626433832795028841971693993751058209749445923;
  let z = z - 1;
  let base = z + LANCZOS_G_7 + 0.5;
  let mut sum : V = ι(0);
  for &(i,c) in LANCZOS_7[1..7].iter().rev() {
    sum += ι(c):V / (z + i);
  }
  sum += LANCZOS_7[0].1;
  ((sf_log(sum) + LQ2P) - base) + sf_log(base)*(z + 0.5)
}

}

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
*/

