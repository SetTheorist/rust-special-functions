pub trait Airy : Sized {
  fn airy_ai(self) -> Self;
  fn airy_bi(self) -> Self;

  fn airy_aibi(self) -> (Self,Self);
}

pub mod impls {
use crate::traits::*;
use crate::exp::*;
use crate::gamma::*;
use crate::log::*;

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


}
