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
  const AI_0 : Wide =
    Wide!("0.5ae31e589c56e17a96d7bb04e64f6da97ab1006b26f9eb6421233394220b8457047cb9557c9f3b43d25");
  const DAI_0 : Wide =
    Wide!("-0.4241fd0adc3e2a2eba6fca5021d9ee1e6a31f0935d3a8b0fa6c28c3a9c0b546160e3c53e9e70f05d7ec");
  const BI_0 : Wide =
    Wide!("0.9d6bd4da51f54baaecb6804cf7a2ebed9295e0411ef5e1837c243549c5f62ba6b264f543ab24817a527");
  const DBI_0 : Wide =
    Wide!("0.72c3069a0322822c217efc48899a487ad1b3abb9b4b8f6e55fe15334ba584999b84b702b5d6bdcaf4bd");
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
    if self >= ι(80) {
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
  // explicit series sum (iterator versions much slower)
  let mut res = V::one;
  let mut term = V::one;
  let z3 = z*z*z;
  for n in 1..1000 {
    term *= z3 / ((n*3)*(n*3-1));
    let old_res = res;
    res += term;
    if res == old_res {break;}
  }
  res
}

// basic series
pub fn aibi_2<V:Value>(z:V) -> V {
  let mut res = z;
  let mut term = z;
  let z3 = z*z*z;
  for n in 1..1000 {
    term *= z3 / ((n*3+1)*(n*3));
    let old_res = res;
    res += term;
    if res == old_res {break;}
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
    term_1 *= z3 / ((n*3)*(n*3-1));
    term_2 *= z3 / ((n*3+1)*(n*3));
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
  let (cos,sin) = sf_cos_sin(ζ - V::PI*0.25);
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
  for (x,w) in crate::algorithm::integration::GAUSS_LAGUERRE_23__MINUS16_XW.iter().map(|(x,w)|(ι(*x):V,ι(*w):V)) {
    sum += w*(x/ζ + 2).nth_root(-6);
  }
  sum * sf_exp(-ζ) * ζ.nth_root(-6) * ig56
}

pub fn ai_integ_pos__wide(z:Wide) -> Wide {
  // 1/(\sqrt{\pi} 48^{1/6} \Gamma(5/6))
  const ig56 : Wide = Wide!("0.431e7d8d0766671c5141a22d6491f3fd1dc7fc5358");
  let ζ = z*z.sqrt()*2/3;
  let mut sum = Wide::zero;
  for (x,w) in crate::algorithm::integration::GAUSS_LAGUERRE_41__MINUS16_XW__WIDE {
    sum += w*(x/ζ + 2).nth_root(-6);
  }
  sum * (-ζ).exp() * ζ.nth_root(-6) * ig56
}