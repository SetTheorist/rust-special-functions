use crate::traits::*;
use crate::bessel::*;
use crate::exp::*;
use crate::gamma::*;
use crate::log::*;
use crate::trig::*;
use crate::numbers::{sf_factorial};

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

pub trait CosInt {
  fn cosint(self) -> Self;
}
#[inline] pub fn sf_cosint<V:CosInt>(z:V) -> V { z.cosint() }

pub trait CoshInt {
  fn coshint(self) -> Self;
}
#[inline] pub fn sf_coshint<V:CoshInt>(z:V) -> V { z.coshint() }

pub trait SinhInt {
  fn sinhint(self) -> Self;
}
#[inline] pub fn sf_sinhint<V:SinhInt>(z:V) -> V { z.sinhint() }

pub trait SinInt {
  fn sinint(self) -> Self;
}
#[inline] pub fn sf_sinint<V:SinInt>(z:V) -> V { z.sinint() }

////////////////////////////////////////////////////////////////////////////////

use crate::real::*;
impl ExpInt for r64 {
  fn expint_ei(self) -> Self { expint_ei_real(self) }
  fn expint_en(self, n:isize) -> Self { expint_en_real(n, self) }
}
impl LogInt for r64 {
  fn logint(self) -> Self { sf_expint_ei(sf_log(self)) }
}
impl CosInt for r64 {
  fn cosint(self) -> Self { cosint_real(self) }
}
impl CoshInt for r64 {
  fn coshint(self) -> Self { (sf_expint_ei(self) - sf_expint_en(1, self))/2 }
}
impl SinInt for r64 {
  fn sinint(self) -> Self { sinint_real(self) }
}
impl SinhInt for r64 {
  fn sinhint(self) -> Self { (sf_expint_ei(self) + sf_expint_en(1, self))/2 }
}

use crate::complex::*;
impl CosInt for c64 {
  fn cosint(self) -> Self { cosint_complex(self) }
}
impl SinInt for c64 {
  fn sinint(self) -> Self { sinint_complex(self) }
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
  match n {
    n if n < 0 => V::nan,
    0 => expint_en_0(z),
    1 => expint_en_1(z),
    _ => 
      if z <= ι(1):V {
        expint_en_series(n, z)
      } else {
        expint_en_contfrac(n, z)
      },
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
    * (-z).pow(n-1)/sf_factorial::<V>((n-1) as usize) + V::one/(n-1);
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

////////////////////////////////////////////////////////////////////////////////

pub fn cosint_real<V:RealValue+Log>(z:V) -> V where V::CT:Exp {
  if abs(z) < ι(5):V::NT {
    cosint_series(z)
  } else if z.is_nonnegreal() {
    -e1_contfrac(V::CT::I * z).real()
  } else {
    V::nan
  }
}
pub fn cosint_complex<V:ComplexValue+Exp+Log>(z:V) -> V {
  if abs(z) < ι(5):V::NT {
    cosint_series(z)
  } else {
    if z.is_real() {
      ι(-e1_contfrac(V::I * z).real()):V + (if z.is_negreal() {V::PI*V::I} else {ι(0):V})
    } else {
      -(e1_contfrac(V::I*z) + e1_contfrac(-V::I*z))/2
    }
  }
}

// TODO: use Kahan (use Wide value of constant to start?)
pub fn cosint_series<V:Value+Log>(z:V) -> V {
  let mut sum  = V::EULER_GAMMA + sf_log(z);
  let mut t = V::one;
  let z2 = -z.sqr();
  for n in 1..1000 {
    t *= z2/((2*n-1)*(2*n));
    let old_sum = sum;
    sum += t/(2*n);
    if sum == old_sum {break;}
  }
  sum
}

use crate::algorithm::{contfrac_modlentz};
// continued fraction for E_1(z)
pub fn e1_contfrac<V:Value+Exp>(z:V) -> V {
  let terms = (1..).map(|n|(ι((n+1)/2):V, if n.is_evenint(){z}else{V::one}));
  sf_exp(-z)/contfrac_modlentz(z, terms, V::epsilon)
}

// doesn't appear to have much accuracy
// TODO: investigate if is useful for _very_ large values
pub fn cosint_asympt<V:Value+Normed+Trig>(z:V) -> V {
  // if z.real() < 0 { z = -z; }
  sf_sin(z)*cosint_asympt_f(z) - sf_cos(z)*cosint_asympt_g(z)
}

pub fn cosint_asympt_f<V:Value+Normed>(z:V) -> V {
  let mut t = V::one;
  let mut sum = V::one;
  let z2 = -z.sqr().recip();
  for n in 1..1000 {
    let old_t = t;
    t *= z2*(2*n)*(2*n-1);
    if abs(t) > abs(old_t) {break;}
    let old_sum = sum;
    sum += t;
    if old_sum == sum {break;}
  }
  sum / z
}

pub fn cosint_asympt_g<V:Value+Normed>(z:V) -> V {
  let mut t = V::one;
  let mut sum = V::one;
  let z2 = -z.sqr().recip();
  for n in 1..1000 {
    let old_t = t;
    t *= z2*(2*n)*(2*n+1);
    if abs(t) > abs(old_t) {break;}
    let old_sum = sum;
    sum += t;
    if old_sum == sum {break;}
  }
  sum / z.sqr()
}

////////////////////////////////////////////////////////////////////////////////

pub fn sinint_real<V:RealValue+BesselSpherJ<isize>+Normed+Trig>(z:V) -> V where V::CT:Exp {
  if abs(z) < ι(5):V::NT {
    sinint_series(z)
  } else if abs(z) < ι(10):V::NT {
    sinint_bessel_series(z)
  } else if abs(z) < ι(50):V::NT {
    V::PI/2 + e1_contfrac(V::CT::I*z).imag()
  } else {
    sinint_asympt(z)
  }
}

pub fn sinint_complex<V:ComplexValue+BesselSpherJ<isize>+Exp+Normed+Trig>(z:V) -> V {
  let mult = if z.is_imag() {V::I} else {V::one};
  if abs(z) < ι(5):V::NT {
    mult * sinint_series(z)
  } else if abs(z) < ι(10):V::NT {
    mult * sinint_bessel_series(z)
  } else if abs(z) < ι(50):V::NT {
    if z.is_real() {
      mult * (V::PI/2 + ι(e1_contfrac(V::I*z).imag()):V)
    } else {
      mult * (V::PI/2 + V::I/2 * (e1_contfrac(-V::I*z) - e1_contfrac(V::I*z)))
    }
  } else {
    mult * sinint_asympt(z)
  }
}

pub fn sinint_series<V:Value>(z:V) -> V {
  let mut sum = z;
  let mut t = z;
  let z2 = -z.sqr();
  for n in 1..1000 {
    t *= z2/((2*n)*(2*n+1));
    let old_sum = sum;
    sum += t/(2*n+1);
    if sum == old_sum {break;}
  }
  sum
}

pub fn sinint_bessel_series<V:Value+BesselSpherJ<isize>>(z:V) -> V {
  let mut sum = V::zero;
  let scale = sf_sqrt(z/V::PI);
  let j = |n| sf_bessel_spher_j(n, z/2)*scale;
  for n in 0..1000 {
    let old_sum = sum;
    sum += j(n).sqr();
    if sum == old_sum {break;}
  }
  sum * V::PI
}

// TODO: sort out complex/real issues
pub fn sinint_asympt<V:Value+Normed+Trig>(z:V) -> V {
  //if z.real() < V::RT::zero {
  //  -sinint_asympt(-z)
  //} else {
    V::PI/2 - sf_cos(z)*cosint_asympt_f(z) - sf_sin(z)*cosint_asympt_g(z)
  //}
}


