pub trait Airy : Sized {
  fn airy_ai(self) -> Self;
  fn airy_bi(self) -> Self;

  fn airy_aibi(self) -> (Self,Self);
}
#[inline] pub fn sf_airy_ai<V:Airy>(z:V) -> V { z.airy_ai() }
#[inline] pub fn sf_airy_bi<V:Airy>(z:V) -> V { z.airy_bi() }
#[inline] pub fn sf_airy_aibi<V:Airy>(z:V) -> (V,V) { z.airy_aibi() }

pub trait AiryConstants {
  const AI_0  : Self;  // =  sf_exp(-sf_log(3)*2/3 - sf_lngamma(2/3))
  const DAI_0 : Self;  // = -sf_exp(-sf_log(3)*1/3 - sf_lngamma(1/3))
  const BI_0  : Self;  // =  sf_exp(-sf_log(3)*1/6 - sf_lngamma(2/3))
  const DBI_0 : Self;  // =  sf_exp( sf_log(3)*1/6 - sf_lngamma(1/3))
}

pub mod impls {
use super::*;
use crate::traits::*;
use crate::exp::*;
use crate::gamma::*;
use crate::log::*;
use crate::trig::*;

use crate::real::*;
use crate::complex::*;
use crate::wide::*;

impl AiryConstants for r64 {
  const AI_0  : Self = r64( 0.35502805388781723926);
  const DAI_0 : Self = r64(-0.25881940379280679841);
  const BI_0  : Self = r64( 0.61492662744600073515);
  const DBI_0 : Self = r64( 0.44828835735382635791);
}

impl AiryConstants for c64 {
  const AI_0  : Self = c64{re:r64::AI_0,  im:r64::zero};
  const DAI_0 : Self = c64{re:r64::DAI_0, im:r64::zero};
  const BI_0  : Self = c64{re:r64::BI_0,  im:r64::zero};
  const DBI_0 : Self = c64{re:r64::DBI_0, im:r64::zero};
}

impl AiryConstants for Wide {
  // TODO: procmacro to do all this at compile time!
  //const AI_0  : Wide =  "0.3550280538878172392600631860041831763980".parse().unwrap();
  //const DAI_0 : Wide = "-0.2588194037928067984051835601892039634791".parse().unwrap();
  //const BI_0  : Wide =  "0.6149266274460007351509223690936135535947".parse().unwrap();
  //const DBI_0 : Wide =  "0.4482883573538263579148237103988283908662".parse().unwrap();
  const AI_0  : Wide = Wide( 3.550280538878172e-1, 2.0523363243621203e-17);
  const DAI_0 : Wide = Wide(-2.588194037928068e-1, 2.522243111610832e-17);
  const BI_0  : Wide = Wide( 6.149266274460007e-1, 5.089920779489141e-17);
  const DBI_0 : Wide = Wide( 4.482883573538264e-1,-2.5363237774417318e-17);
}

// TODO: need methods for intermediate cases (and generally more accurate approach)
impl Airy for r64 {
  fn airy_ai(self) -> Self {
    if self >= ι(1.8) {
      ai_integ_pos(self)
    } else if self >= ι(10) {
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
pub fn airy_series<V:Value+AiryConstants>(z:V) -> (V,V) {
  let s1 = aibi_1(z);
  let s2 = aibi_2(z);
  let ai = V::AI_0*s1 + V::DAI_0*s2;
  let bi = V::BI_0*s1 + V::DBI_0*s2;
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

pub fn airy_series__combined<V:Value+AiryConstants>(z:V) -> (V,V) {
  let z3 = z*z*z;
  let mut term_1 = V::one;
  let mut term_2 = z;
  let mut ai = V::AI_0 + V::DAI_0*z;
  let mut bi = V::BI_0 + V::DBI_0*z;
  for n in 1..1000 {
    let old_ai = ai;
    let old_bi = bi;
    term_1 *= z3 * (n*3-2) / ((n*3)*(n*3-1)*(n*3-2));
    term_2 *= z3 * (n*3-1) / ((n*3+1)*(n*3)*(n*3-1));
    ai += V::AI_0*term_1 + V::DAI_0*term_2;
    bi += V::BI_0*term_1 + V::DBI_0*term_2;
    if ai == old_ai && bi == old_bi {break;}
  }
  (ai,bi)
}

// for |ph(z)|<=π-δ
pub fn ai_asympt_pos<V:Value+Exp+Normed>(z:V) -> V {
  let ζ = z*z.sqrt()*2/3;
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
  sf_exp(-ζ)/(V::SQRTPI*2*z.nth_root(4))*sum
}

// for |ph(z)|<=π-δ
pub fn bi_asympt_pos<V:Value+Exp+Normed>(z:V) -> V {
  let ζ = z*z.sqrt()*2/3;
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
  sf_exp(ζ)/(V::SQRTPI*z.nth_root(4))*sum
}

// for |ph(-z)|<=π-δ
pub fn ai_asympt_neg<V:Value+Normed+Trig>(z:V) -> V {
  let z = -z;
  let ζ = z*z.sqrt()*2/3;
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
  V::FRAC_1_SQRTPI*z.nth_root(-4)*(cos*sum_even + sin*sum_odd)
}

// for |ph(-z)|<=π-δ
pub fn bi_asympt_neg<V:Value+Normed+Trig>(z:V) -> V {
  let z = -z;
  let ζ = z*z.sqrt()*2/3;
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
  V::FRAC_1_SQRTPI*z.nth_root(-4)*(-sin*sum_even + cos*sum_odd)
}

pub fn ai_integ_pos<V:Value+Exp>(z:V) -> V {
  // 1/(\sqrt{\pi} 48^{1/6} \Gamma(5/6))
  let ig56 : V = ι(0.26218399708832294968);
  let ζ = z*z.sqrt()*2/3;
  let mut sum = V::zero;
  for (x,w) in crate::integration::GAUSS_LAGUERRE_23__MINUS16_XW.iter().map(|(x,w)|(ι(*x):V,ι(*w):V)) {
    sum += w*(x/ζ + 2).nth_root(-6);
  }
  sum * sf_exp(-ζ) * ζ.nth_root(-6) * ig56
}


pub fn ai_integ_pos__wide(z:Wide) -> Wide {
  // 1/(\sqrt{\pi} 48^{1/6} \Gamma(5/6))
  let ig56 : Wide = "0.2621839970883229496786247788550868016857".parse().unwrap();
  let ζ = z*z.sqrt()*2/3;
  let mut sum = Wide::zero;
  for (x,w) in
    crate::integration::GAUSS_LAGUERRE_41__MINUS16_XW__STRING
    .iter()
    .map(|(x,w)|(x.parse().unwrap():Wide, w.parse().unwrap():Wide))
  {
    sum += w*(x/ζ + 2).nth_root(-6);
  }
  sum * (-ζ).exp() * ζ.nth_root(-6) * ig56
}

}
