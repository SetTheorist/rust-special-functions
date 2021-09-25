
pub trait Lambert {
  fn lambert_w0(self) -> Self;
  fn lambert_w1(self) -> Self;
}

pub mod impls {
use crate::traits::*;
use crate::exp::*;
use crate::log::*;

// positive real branch
pub fn pos_real_0<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if x < V::FRAC_1_E {return V::nan;}
  if x == 0 {return V::zero;}
  let w = if x.is_negreal() {-V::one/10}
          else {sf_log(x / sf_log_1p(x))};
  halley_iter(x, w)
}

// negative real branch
pub fn pos_real_1<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if x < -V::FRAC_1_E || x.is_posreal() {return V::nan;}
  if x == 0 {return V::zero;}
  if x == -V::FRAC_1_E {return ι(-1);}
  // initial approximation
  if x < ι(-0.183939) {
    // series expansion near -1/e
    let p = -sf_sqrt((V::E*x+1)*2);
    let w = p - 1 - p*p/3 + p*p*p*11/72 - p*p*p*p*43/540;
    halley_iter(x, w)
  } else {
    // asymptotic near 0^-
    let l1 = sf_log(-x);
    let l2 = sf_log(-sf_log(-x));
    let w = l1 - l2 + l2/l1
      + ((l2-2)*l2)/(l1.sqr()*2)
      + (l2.sqr()*2 - l2*9 + 6)*l2/(l1.pow(3)*6);
    halley_iter(x, w)
  }
}

pub fn halley_iter<V:RealValue+Exp>(x:V, w0:V) -> V {
  let mut w = w0;
  // Halley iteration
  let mut ow = w + 1;
  for _ in 0..1000 {
    let ew = sf_exp(w);
    let oow = ow;
    ow = w;
    w -= (w*ew - x) / ((w+1)*ew - (w+2)*(w*ew-x)/(w*2+2));
    if w==ow || w==oow {break;}
  }
  return w;
}

}
