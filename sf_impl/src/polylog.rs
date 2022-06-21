use crate::traits::*;

pub trait Dilog {
  fn dilog(self) -> Self;
}
#[inline] pub fn sf_dilog<V:Dilog>(z:V) -> V { z.dilog() }

pub mod impls {
use crate::traits::*;
use crate::numbers::{sf_bernoulli_number_approx};
use crate::log::*;
use crate::expint::*;
use super::*;

use crate::real::*;
impl Dilog for r64 {
  fn dilog(self) -> Self {
    // TODO: incorporate dilog_zeta_series()!
    let z = self;
    if z > ι(1) {
      r64::nan
    } else if z == 1 {
      r64::PI.sqr()/6
    } else if z > ι(0.5) {
      // L(z) + L(1-z) = pi^2/6 - log(z)log(1-z)
      r64::PI.sqr()/6 - sf_log(z)*sf_log_1p(-z) - dilog_series(r64::one - z)
    } else if abs(z) <= r64(2.0)/3 {
      // radius of convergence = 1
      dilog_series(z)
    } else if abs(z/(z-1)) < r64(2.0)/3 {
      // L(z) + L(z/(z-1)) = -log(1-z)^2/2
      -sf_log_1p(-z).sqr()/2 - dilog_series(z/(z-1))
    } else if abs(z) >= r64(2.0)/3 {
      // L(z) + L(1/z) = -pi^2/6 - log(-z)^2/2
      -r64::PI.sqr()/6 - sf_log(-z).sqr()/2 - dilog_series(z.recip())
    } else {
      // L(z/(z-1)) + L((z-1)/z) = -pi^2/6 - log(-z(z-1))^2/2
      -sf_log_1p(-z).sqr()/2 - (-dilog_series((z-1)/z) - r64::PI.sqr()/6 - sf_log(-z/(z-1)).sqr()/2)
    }
  }
}

pub fn dilog_series<V:Value>(z:V) -> V {
  let mut t = z;
  let mut sum = t;
  for n in 2..1000 {
    t *= z;
    let old = sum;
    sum += t/(n*n);
    if sum == old {break;}
  }
  sum
}

// for |log(z)|<2π
// converges faster for z near 1 !
// TODO: incorporate into r64 implementation above!
pub fn dilog_zeta_series<V:Value+Log>(z:V) -> V {
  let lnz = sf_log(z);
  let ln2z = lnz.sqr();
  let mut t = ln2z/2;
  let mut sum = lnz*(V::one - sf_log(-lnz)) + V::PI.sqr()/6 - t/2;
  t *= lnz/3;
  for n in (3..1000).step_by(2) {
    let old = sum;
    sum += t * ι(-sf_bernoulli_number_approx((n-1) as usize)):V / (n-1);
    if sum == old {break;}
    t *= ln2z/((n+1)*(n+2));
  }
  sum
}

// TODO: Doesn't seem to actually improve convergence, need to recalculate
// (or maybe sf_expint_en() accuracy is too low to help)
pub fn dilog_series_em3<V:Value+ExpInt+Log>(z:V) -> V {
  let mut t = z;
  let mut sum = t;
  let mut res = t;
  let lnz = sf_log(z);
  let bn2 : V = ι(sf_bernoulli_number_approx(2));
  let bn4 : V = ι(sf_bernoulli_number_approx(4));
  let bn6 : V = ι(sf_bernoulli_number_approx(6));
  for n in 2..1000 {
    let n2 = n*n;
    t *= z;
    sum += t/n2;
    let old = res;
    res = sum + sf_expint_en(2, -lnz*n) - t/(2*n2)
      - bn2*(lnz - ι(2):V/n)*t/n2
      - bn4*(lnz.pow(3) - lnz.pow(2)*6/n + lnz*18/n2 - ι(24):V/n.pow(3))*t/n2
      - bn6*(lnz.pow(5) - lnz.pow(4)*10/n + lnz.pow(3)*60/n2 - lnz.pow(2)*240/n.pow(3)
              + lnz*600/n.pow(4) - ι(720):V/n.pow(5))*t/n2;
    //if μ(old-res)<V::epsilon*μ(res)*1024 {print!("*{}*",n);break;}
    if old==res {break;}
  }
  res
}

}


