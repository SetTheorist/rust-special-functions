use crate::algorithm::{contfrac_modlentz, sum_series};
use crate::exp::{sf_exp, Exp};
use crate::log::{sf_log, Log};
use crate::numbers::{sf_bernoulli_number_approx, sf_factorial_approx};
use crate::traits::*;

////////////////////////////////////////////////////////////////////////////////
//
// Spouge approximation
//
#[inline]
fn spouge_c<V:Value+Exp+Power>(k: isize, a: V) -> V {
  (ι(1):V / sf_factorial_approx((k-1) as usize)).pari(k+1) * (a-k).pow(ι(k):V - 0.5) * sf_exp(a-k)
}
pub fn gamma_spouge<V:Value+Exp+Power>(a: isize, z: V) -> V {
  let z = z - 1;
  let res: V = (z + a).pow(z + 0.5) * sf_exp(-(z + a));
  let mut sm: V = V::SQRT2PI;
  for k in 1..=(a - 1) {
    sm += spouge_c(k, ι(a)):V / (z + k);
  }
  res * sm
}

////////////////////////////////////////////////////////////////////////////////
//
// Asymptotic expansion
//

// assumes re>0
pub fn gamma_asympt<V: Value+Log+Exp>(x: V) -> V {
  let mut div = V::one;
  let mut z = x;
  // shift z
  while z.fabs() < 50.0 {
    //res += -sf_log(z*(z+1));
    div *= (z * (z + 1));
    z += 2;
  }
  let z = z;

  let mut res = V::zero;
  let mut term: V = (z - 0.5) * sf_log(z) - z + V::FRAC_LOG2PI_2;
  res += term;
  for m in (2..250).step_by(2) {
    let old_term = term;
    term = (ι(sf_bernoulli_number_approx(m as usize)):V) / (z.pow(m-1) * (m*(m-1)));
    if μ(term) > μ(old_term) { break; }
    let old_res = res;
    res += term;
    if res == old_res { break; }
  }
  sf_exp(res) / div
}

////////////////////////////////////////////////////////////////////////////////
//
// Lanczos approximation(s)
//
use paste::paste;
macro_rules! make_lanczos {
  ($fty:ty, $n:expr, $g:expr; $coeffs:expr) => {
    paste! {
      // use for z>1/2 (otherwise use standard reflection)
      pub fn [<lngamma_lanczos_ $n>]<V:Value+Exp+Log>(z: V) -> V {
        const G : $fty = $g;
        const COEFFS : [(isize, $fty); $n] = $coeffs;
        let z = z - 1;
        let base = z + G + 0.5;
        let mut sum: V = ι(0);
        for &(i, c) in COEFFS[1..7].iter().rev() {
          sum += ι(c):V / (z + i);
        }
        sum += COEFFS[0].1;
        ((sf_log(sum) + V::FRAC_LOG2PI_2) - base) + sf_log(base) * (z + 0.5)
      }
    }
  }
}

make_lanczos!{f64, 7, 5.0; [
  (0, 1.000000000190015),
  (1, 76.18009172947146),
  (2, -86.50532032941677),
  (3, 24.01409824083091),
  (4, -1.231739572450155),
  (5, 0.1208650973866179e-2),
  (6, -0.5395239384953e-5),
]}

make_lanczos!{f64, 15, 4.7421875; [
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
  (14, 0.36899182659531622704e-5),
]}

make_lanczos!{f64, 9, 7.0; [
  (0, 0.99999999999980993227684700473478),
  (1, 676.520368121885098567009190444019),
  (2, -1259.13921672240287047156078755283),
  (3, 771.3234287776530788486528258894),
  (4, -176.61502916214059906584551354),
  (5, 12.507343278686904814458936853),
  (6, -0.13857109526572011689554707),
  (7, 9.984369578019570859563e-6),
  (8, 1.50563273514931155834e-7),
]}

make_lanczos!{f64, 11, 9.0; [
  (0, 1.000000000000000174663),
  (1, 5716.400188274341379136),
  (2, -14815.30426768413909044),
  (3, 14291.49277657478554025),
  (4, -6348.160217641458813289),
  (5, 1301.608286058321874105),
  (6, -108.1767053514369634679),
  (7, 2.605696505611755827729),
  (8, -0.7423452510201416151527e-2),
  (9, 0.5384136432509564062961e-7),
  (10, -0.4023533141268236372067e-8),
]}
