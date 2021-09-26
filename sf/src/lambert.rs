
pub trait Lambert {
  fn lambert_w0(self) -> Self;
  fn lambert_w1(self) -> Self;
  fn lambert_w(self, b:isize) -> Self;
}


pub mod impls {
use crate::traits::*;
use crate::exp::*;
use crate::log::*;

pub fn real_branches<V:RealValue+Exp+Float+Log>(x:V, n:isize) -> V {
  // only 2 real branches
  match n {
    0 => { real_branch_pos(x) }
    -1 => { real_branch_neg(x) }
    _ => { V::nan }
  }
}

// positive real branch
pub fn real_branch_pos<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if x < -V::FRAC_1_E {return V::nan;}
  if x == 0 {return V::zero;}
  let w =
    if x < ι(0) { //-V::one/10
      // empirical fit
      x*(x*(x*9.06992 + 1.90189) + 1.42099) + 0.0215133
    } else {
      sf_log(x / sf_log_1p(x))
    };
  halley_iter(x, w)
}
pub fn real_branch_pos_2<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if x < -V::FRAC_1_E {return V::nan;}
  if x == 0 {return V::zero;}
  let w =
    if x < ι(0) { //-V::one/10
      // empirical fit
      x*(x*(x*9.06992 + 1.90189) + 1.42099) + 0.0215133
    } else {
      sf_log(x / sf_log_1p(x))
    };
  fritsch_iter(x, w)
}

// negative real branch
pub fn real_branch_neg<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if !x.є(-V::FRAC_1_E, ι(0)) {return V::nan;}
  if x == 0 {return V::zero;}
  if x == -V::FRAC_1_E {return ι(-1);}
  let w;
  if x < ι(-0.183939) {
    // series expansion near -1/e
    let p = -sf_sqrt((V::E*x+1)*2);
    w = p - 1 - p*(p/3 - p*(p*11/72 - p*p*43/540));
  } else {
    // asymptotic near 0^-
    let l1 = sf_log(-x);
    let l2 = sf_log(-sf_log(-x));
    w = l1 - l2 + l2/l1
      + ((l2-2)*l2)/(l1.sqr()*2)
      + (l2.sqr()*2 - l2*9 + 6)*l2/(l1.pow(3)*6);
  }
  halley_iter(x, w)
}
pub fn real_branch_neg_2<V:RealValue+Exp+Float+Log>(x:V) -> V {
  if !x.є(-V::FRAC_1_E, ι(0)) {return V::nan;}
  if x == 0 {return V::zero;}
  if x == -V::FRAC_1_E {return ι(-1);}
  let w;
  if x < ι(-0.183939) {
    // series expansion near -1/e
    let p = -sf_sqrt((V::E*x+1)*2);
    w = p - 1 - p*(p/3 - p*(p*11/72 - p*p*43/540));
  } else {
    // asymptotic near 0^-
    let l1 = sf_log(-x);
    let l2 = sf_log(-sf_log(-x));
    w = l1 - l2 + l2/l1
      + ((l2-2)*l2)/(l1.sqr()*2)
      + (l2.sqr()*2 - l2*9 + 6)*l2/(l1.pow(3)*6);
  }
  fritsch_iter(x, w)
}
// Halley iteration
fn halley_iter<V:Value+Exp>(x:V, w0:V) -> V {
  let mut w = w0;
  let mut ow = w + 1;
  for n in 0..1000 {
    let ew = sf_exp(w);
    let oow = ow;
    ow = w;
    w -= (w*ew - x) / ((w+1)*ew - (w+2)*(w*ew-x)/(w*2+2));
    if w==ow || w==oow {print!("({})",n);break;}
  }
  return w;
}
// Fritsch iteration
// claimed to converge faster, but in practice, looks worse?!
fn fritsch_iter<V:Value+Log>(x:V, w0:V) -> V {
  let mut w = w0;
  let mut ow = w + 1;
  for n in 0..1000 {
    let zn = sf_log(x/w) - w;
    let qn = (w+1)*(w+zn+1)*2;
    let epsn = (zn/(w+1))*((qn-zn)/(qn-zn*2));
    let oow = ow;
    ow = w;
    w *= epsn + 1;
    if w==ow || w==oow {print!("[{}]",n);break;}
  }
  return w;
}

}

mod tests {
use super::*;
use crate::traits::*;
use crate::exp::{sf_exp};
use crate::real::*;
use crate::util::{Grid,relerr};

#[test]
fn pos_real_branch() {
  // in domain
  assert!(!impls::real_branch_pos(r64::FRAC_1_E.next_dn()).is_nan());
  // not in domain
  assert!(impls::real_branch_pos(-r64::FRAC_1_E.next_up()).is_nan());
  assert!(impls::real_branch_pos(r64(-1.0)).is_nan());
  // defining relation
  for x in Grid::new(-r64::FRAC_1_E,r64(100.0),1000) {
    let wx = impls::real_branch_pos(x);
    let x0 = wx * sf_exp(wx);
    assert!(relerr(x, x0) < r64(-15.0));
  }
}

#[test]
fn neg_real_branch() {
  // in domain
  assert!(!impls::real_branch_pos(r64(0.0)).is_nan());
  assert!(!impls::real_branch_pos(r64::FRAC_1_E.next_dn()).is_nan());
  // not in domain
  assert!(impls::real_branch_neg(-r64::FRAC_1_E.next_up()).is_nan());
  assert!(impls::real_branch_neg(r64(-1.0)).is_nan());
  assert!(impls::real_branch_neg(r64(1.0)).is_nan());
  // defining relation
  for x in Grid::new(-r64::FRAC_1_E,r64(0.0),1000) {
    let wx = impls::real_branch_neg(x);
    let x0 = wx * sf_exp(wx);
    assert!(relerr(x, x0) < r64(-15.0));
  }
}

}

