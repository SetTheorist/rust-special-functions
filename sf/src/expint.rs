use crate::traits::*;
use crate::exp::*;
use crate::gamma::*;
use crate::log::*;
use crate::numbers::{sf_factorial_approx};

pub trait ExpInt {
  fn expint_ei(self) -> Self;
  fn expint_en(self, n:isize) -> Self;
}
#[inline] pub fn sf_expint_ei<V:ExpInt>(z:V) -> V { z.expint_ei() }
#[inline] pub fn sf_expint_en<V:ExpInt>(n:isize, z:V) -> V { z.expint_en(n) }

pub trait LogInt {
  fn logint(self) -> Self;
}
#[inline] pub fn sf_logint<V:LogInt>(z:V) -> V { z.logint() }

pub trait CoshInt {
  fn coshint(self) -> Self;
}
#[inline] pub fn sf_coshint<V:CoshInt>(z:V) -> V { z.coshint() }

pub trait SinhInt {
  fn sinhint(self) -> Self;
}
#[inline] pub fn sf_sinhint<V:SinhInt>(z:V) -> V { z.sinhint() }

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl ExpInt for r64 {
  fn expint_ei(self) -> Self { expint_ei_real(self) }
  fn expint_en(self, n:isize) -> Self { expint_en_real(n, self) }
}
impl LogInt for r64 {
  fn logint(self) -> Self { sf_expint_ei(sf_log(self)) }
}
impl CoshInt for r64 {
  fn coshint(self) -> Self { (sf_expint_ei(self) - sf_expint_en(1, self))/2 }
}
impl SinhInt for r64 {
  fn sinhint(self) -> Self { (sf_expint_ei(self) + sf_expint_en(1, self))/2 }
}

////////////////////////////////////////////////////////////////////////////////

pub fn expint_ei_real<V:RealValue+Float+Exp+Log>(z:V) -> V {
  if z.is_negreal() {
    V::nan
  } else if z == 0 {
    -V::infinity
  } else if z < ι(40):V {
    // series expansion
    let mut t = V::one;
    let mut sum = V::zero;
    for n in 1..1000 {
      t *= z/n;
      let old = sum;
      sum += t/n;
      if sum == old {break;}
    }
    if z < ι(0.5):V {
      sf_log(z * sf_exp(V::EULER_GAMMA + sum))
    } else {
      sum + sf_log(z) + V::EULER_GAMMA
    }
  } else {
    // asymptotic expansion
    let mut t = V::one;
    let mut sum = V::one;
    for n in 1..1000 {
      let old_t = t;
      t *= ι(n):V / z;
      if t > old_t {break;}
      let old_sum = sum;
      sum += t;
      if sum == old_sum {break;}
    }
    sum * sf_exp(z)/z
  }
}

////////////////////////////////////////////////////////////////////////////////

pub fn expint_en_real<V:RealValue+Exp+Log+Gamma>(n:isize, z:V) -> V {
  if n < 0 {
    V::nan
  } else if n == 0 {
    expint_en_0(z)
  } else if n == 1 {
    expint_en_1(z)
  } else {
    if z <= ι(1):V {
      expint_en_series(n, z)
    } else {
      expint_en_contfrac(n, z)
    }
  }
}

pub fn expint_en_0<V:Value+Exp>(z:V) -> V {
  sf_exp(-z)/z
}

pub fn expint_en_1<V:Value+Log>(z:V) -> V {
  let mut sum = -V::EULER_GAMMA - sf_log(z);
  let mut term = -V::one;
  for k in 1..1000 {
    term *= -z/k;
    let old_sum = sum;
    sum += term/k;
    if sum == old_sum {break;}
  }
  sum
}

// n>=2, x<=1
pub fn expint_en_series<V:Value+Log+Gamma>(n:isize, z:V) -> V {
  // TODO: shouldn't need to lift n to V type!
  let mut sum = (-sf_log(z) + sf_digamma(ι(n):V))
    * (-z).pow(n-1)/sf_factorial_approx((n-1) as usize) + V::one/(n-1);
  let mut t = V::one;
  for m in 1..1000 {
    t *= -z/m;
    if m == n-1 {continue;}
    let old_sum = sum;
    sum -= t/(m-(n-1));
    if sum == old_sum {break;}
  }
  sum
}

// n>=2, x>1
// TODO: use generic modlentz ...
pub fn expint_en_contfrac<V:Value+Exp>(n:isize, z:V) -> V {
  let ζ : V = ι(V::epsilon.sqr());
  let fix = |x| if x==0 {ζ} else {x};
  let mut fj = ζ;
  let mut cj = fj;
  let mut dj = V::zero;
  for j in 1..1000 {
    let aj : V = ι(if j==1 {1} else {-(j-1)*(n+j-2)});
    let bj : V = z + n + 2*(j-1);
    dj = fix(bj + aj*dj);
    cj = fix(bj + aj/cj);
    dj = dj.recip();
    fj *= cj*dj;
  }
  fj * sf_exp(-z)
}


