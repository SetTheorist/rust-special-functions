pub trait Airy : Sized {
  fn airy_ai(self) -> Self;
  fn airy_bi(self) -> Self;

  fn airy_aibi(self) -> (Self,Self);
}
#[inline] pub fn sf_airy_ai<V:Airy>(z:V) -> V { z.airy_ai() }
#[inline] pub fn sf_airy_bi<V:Airy>(z:V) -> V { z.airy_bi() }
#[inline] pub fn sf_airy_aibi<V:Airy>(z:V) -> (V,V) { z.airy_aibi() }

pub mod impls {
use super::*;
use crate::traits::*;
use crate::exp::*;
use crate::gamma::*;
use crate::log::*;
use crate::trig::*;

use crate::real::*;
// TODO: need methods for intermediate cases (and generally more accurate approach)
impl Airy for r64 {
  fn airy_ai(self) -> Self {
    if self >= ι(6) {
      ai_asympt_pos(self)
    } else if self <= ι(-7) {
      ai_asympt_neg(self)
    } else {
      airy_series(self).0
    }
  }
  fn airy_bi(self) -> Self {
    if self >= ι(30) {
      bi_asympt_pos(self) // TODO: (?) when is this preferred to series?
      //airy_series(self).1
    } else if self <= ι(-7) {
      bi_asympt_neg(self)
    } else {
      airy_series(self).1
    }
  }
  fn airy_aibi(self) -> (Self,Self) {
    // TODO: conditions for this
    let (ai,bi) = airy_series(self);
    (ai,bi)
  }
}

// TODO: not great precision from this
// may be from lngamma, perhaps
pub fn airy_series<V:Value+Exp+Gamma+Log>(z:V) -> (V,V) {
  // TODO: precompute these fixed constants
  let ai_0  =  sf_exp(-sf_log(ι(3):V)*2/3 - sf_lngamma(ι(2):V/3));
  let dai_0 = -sf_exp(-sf_log(ι(3):V)*1/3 - sf_lngamma(ι(1):V/3));
  let bi_0  =  sf_exp(-sf_log(ι(3):V)*1/6 - sf_lngamma(ι(2):V/3));
  let dbi_0 =  sf_exp( sf_log(ι(3):V)*1/6 - sf_lngamma(ι(1):V/3));
  let s1 = aibi_1(z);
  let s2 = aibi_2(z);
  let ai = ai_0*s1 + dai_0*s2;
  let bi = bi_0*s1 + dbi_0*s2;
  (ai,bi)
}

// basic series
pub fn aibi_1<V:Value>(z:V) -> V {
  let mut res = V::one;
  let mut term = V::one;
  let z3 = z*z*z;
  for n in 1..1000 {
    term *= z3 * (n*3-2) / ((n*3)*(n*3-1)*(n*3-2));
    let old_res = res;
    res += term;
    if res == old_res { break; }
  }
  res
}

// basic series
pub fn aibi_2<V:Value>(z:V) -> V {
  let mut res = z;
  let mut term = z;
  let z3 = z*z*z;
  for n in 1..1000 {
    term *= z3 * (n*3-1) / ((n*3+1)*(n*3)*(n*3-1));
    let old_res = res;
    res += term;
    if res == old_res { break; }
  }
  res
}

// for |ph(z)|<=π-δ
pub fn ai_asympt_pos<V:Value+Exp+Normed+Power>(z:V) -> V {
  let ζ = z.pow(ι(1.5):V)*2/3;
  let ζi = ζ.recip();
  let mut ζk = V::one;
  let mut sum = V::zero;
  let mut uk = V::one;
  let mut old_t = ι(V::epsilon.recip()):V;
  for k in 0..1000 {
    let t = uk * ζk.pari(k);
    if μ(t) > μ(old_t) {break;}
    old_t = t;
    let old_sum = sum;
    sum += t;
    if sum == old_sum {break;}
    ζk *= ζi;
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
  }
  sf_exp(-ζ)/(V::SQRTPI*2*z.pow(ι(0.25):V))*sum
}

// for |ph(z)|<=π-δ
pub fn bi_asympt_pos<V:Value+Exp+Normed+Power>(z:V) -> V {
  let ζ = z.pow(ι(1.5):V)*2/3;
  let ζi = ζ.recip();
  let mut ζk = V::one;
  let mut sum = V::zero;
  let mut uk = V::one;
  let mut old_t = ι(V::epsilon.recip()):V;
  for k in 0..1000 {
    let t = uk * ζk;
    if μ(t) > μ(old_t) {break;}
    old_t = t;
    let old_sum = sum;
    sum += t;
    if sum == old_sum {break;}
    ζk *= ζi;
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
  }
  sf_exp(ζ)/(V::SQRTPI*z.pow(ι(0.25):V))*sum
}

// for |ph(-z)|<=π-δ
pub fn ai_asympt_neg<V:Value+Normed+Power+Trig>(z:V) -> V {
  let z = -z;
  let ζ = (z.pow(ι(1.5):V)*2)/3;
  let ζi = ζ.recip();
  let mut ζk = V::one;
  let mut sum_even = V::zero;
  let mut sum_odd = V::zero;
  let mut uk = V::one;
  let mut do_even = true;
  let mut do_odd = true;
  let mut old_t_even = ι(V::epsilon.recip()):V;
  let mut old_t_odd = ι(V::epsilon.recip()):V;
  for k in (0..1000).step_by(2) {
    let t_even = uk * ζk.pari(k/2);
    if μ(t_even) > μ(old_t_even) { do_even = false; }
    old_t_even = t_even;
    let old_sum_even = sum_even;
    if do_even { sum_even += t_even; }
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
    ζk *= ζi;

    let t_odd = uk * ζk.pari((k-1)/2);
    if μ(t_odd) > μ(old_t_odd) { do_odd = false; }
    old_t_odd = t_odd;
    let old_sum_odd = sum_odd;
    if do_odd { sum_odd += t_odd; }
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
    ζk *= ζi;

    if sum_odd == old_sum_odd && sum_even == old_sum_even { break; }
  }
  let cos = sf_cos(ζ - V::PI*0.25);
  let sin = sf_sin(ζ - V::PI*0.25);
  V::FRAC_1_SQRTPI*z.pow(ι(-0.25):V)*(cos*sum_even + sin*sum_odd)
}

// for |ph(-z)|<=π-δ
pub fn bi_asympt_neg<V:Value+Normed+Power+Trig>(z:V) -> V {
  let z = -z;
  let ζ = (z.pow(ι(1.5):V)*2)/3;
  let ζi = ζ.recip();
  let mut ζk = V::one;
  let mut sum_even = V::zero;
  let mut sum_odd = V::zero;
  let mut uk = V::one;
  let mut do_even = true;
  let mut do_odd = true;
  let mut old_t_even = ι(V::epsilon.recip()):V;
  let mut old_t_odd = ι(V::epsilon.recip()):V;
  for k in (0..1000).step_by(2) {
    let t_even = uk * ζk.pari(k/2);
    if μ(t_even) > μ(old_t_even) { do_even = false; }
    old_t_even = t_even;
    let old_sum_even = sum_even;
    if do_even { sum_even += t_even; }
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
    ζk *= ζi;

    let t_odd = uk * ζk.pari((k-1)/2);
    if μ(t_odd) > μ(old_t_odd) { do_odd = false; }
    old_t_odd = t_odd;
    let old_sum_odd = sum_odd;
    if do_odd { sum_odd += t_odd; }
    let k = k+1;
    uk = uk * (6*k-5)*(6*k-3)*(6*k-1) / ((2*k-1)*216*k);
    ζk *= ζi;

    if sum_odd == old_sum_odd && sum_even == old_sum_even { break; }
  }
  let cos = sf_cos(ζ - V::PI*0.25);
  let sin = sf_sin(ζ - V::PI*0.25);
  V::FRAC_1_SQRTPI*z.pow(ι(-0.25):V)*(-sin*sum_even + cos*sum_odd)
}

}
