use crate::traits::*;

pub trait Zeta: Value {
  fn zeta(self) -> Self;
  fn zeta_m1(self) -> Self;
}

pub trait HurwitzZeta: Value {
  fn hurwitz_zeta(self, a:Self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////

use crate::real::r64;
pub fn sf_zeta_approx(n: usize) -> f64 { impls::zeta_series_em9(ι(n as isize): r64, r64::epsilon).0 }

pub mod impls {
  use crate::traits::*;

  pub fn zeta_series_em9<V: Value + Power>(s: V, eps: V::NT) -> V {
    let terms = (1..).map(|n| (ι(n): V).pow(-s));
    let mut sum = V::zero;
    let mut n = 1;
    let mut old = V::zero;
    for t in terms {
      sum += t;
      let vn: V = ι(n);
      let res = sum + vn.pow(-s + 1) / (s - 1) - vn.pow(-s) / 2 + vn.pow(-s - 1) * (s / 12)
        - vn.pow(-s - 3) * (s * (s + 1) * (s + 2) / 720)
        + vn.pow(-s - 5) * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) / 30240)
        - vn.pow(-s - 7) * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) * (s + 5) * (s + 6) / 1209600)
        + vn.pow(-s - 9)
          * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) * (s + 5) * (s + 6) * (s + 7) * (s + 8) / 239500800);
      if μ(res - old) <= μ(res) * eps && n > 2 {
        //::log::debug!("zeta_series_em9::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), s, eps, n);
        break;
      }
      old = res;
      n += 1;
    }
    old
  }

  pub fn zeta_m1_series_em9<V: Value + Power>(s: V, eps: V::NT) -> V {
    let terms = (2..).map(|n| (ι(n): V).pow(-s));
    let mut sum = V::zero;
    let mut n = 2;
    let mut old = V::zero;
    for t in terms {
      sum += t;
      let vn: V = ι(n);
      let res = sum + vn.pow(-s + 1) / (s - 1) - vn.pow(-s) / 2 + vn.pow(-s - 1) * (s / 12)
        - vn.pow(-s - 3) * (s * (s + 1) * (s + 2) / 720)
        + vn.pow(-s - 5) * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) / 30240)
        - vn.pow(-s - 7) * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) * (s + 5) * (s + 6) / 1209600)
        + vn.pow(-s - 9)
          * (s * (s + 1) * (s + 2) * (s + 3) * (s + 4) * (s + 5) * (s + 6) * (s + 7) * (s + 8) / 239500800);
      if μ(res - old) <= μ(res) * eps && n > 2 {
        //::log::debug!("zeta_m1_series_em9::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), s, eps, n);
        break;
      }
      old = res;
      n += 1;
    }
    old
  }
}

/*
pub fn zeta_directseries<T:Value>(s:r64) -> r64 {
  let terms = (1..).map(|n|r64((n as f64).powf(-s.0)));
  sumit(terms, 1e-16)
}
// ~half as many terms needed...
pub fn zeta_directseries2(s:r64) -> r64 {
  let terms = (0..).map(|n|r64(((2*n+1) as f64).powf(-s.0)));
  sumit(terms, 1e-16)/(1.0 - 2.0_f64.powf(-s.0))
}
pub fn zeta_directseries_em1(s:r64) -> r64 {
  let terms = (1..).map(|n|r64((n as f64).powf(-s.0)));
  let mut sum = r64(0.0);
  let mut n = 1;
  let mut old_res = ι(0);
  for t in terms {
    sum += t;
    let res = sum + (n as f64).powf(1.0 - s.0)/(s.0 - 1.0);
    if abs(res - old_res) <= abs(res)*1e-16 { print!("${}$",n);break; }
    old_res = res;
    n += 1;
  }
  old_res
}
pub fn zeta_m1_directseries(s:r64) -> r64 {
  let terms = (2..).map(|n|r64((n as f64).powf(-s.0)));
  sumit(terms, 1e-16)
}
*/

////////////////////////////////////////////////////////////////////////////////

pub mod impls_hurwitz {
use super::*;
use crate::traits::*;

// TODO: generally looks incorrect for complex values
pub fn hurwitz_series_em<V:Value+Power>(z:V, a:V) -> V {
  if z == 1 { return ι(f64::INFINITY); } // TODO
  // TODO: validate reflection
  /*
  if z.re() < 1 {
    return 2 * (2*pi).pow(z-1)*sf_sin(V::PI*z/2)*sf_gamma(1-z)*sf_hurwitz_zeta(1-z);
  }
  */
  let mut oores;
  let mut ores = V::zero;
  let mut res = V::zero;
  let mut sum = a.pow(-z) + (a+1).pow(-z);
  let em1 = z/12;
  let em2 = z*(z+1)*(z+2)/720;
  let em3 = z*(z+1)*(z+2)*(z+3)*(z+4)/30240;
  let em4 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)/1209600;
  let em5 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)/239500800;
  for n in 2..1000 {
    sum += (a+n).pow(-z);
    oores = ores;
    ores = res;
    res = sum + (a+n).pow(-z+1)/(z-1) - (a+n).pow(-z)/2
      + (a+n).pow(-z-1)*em1 - (a+n).pow(-z-3)*em2
      + (a+n).pow(-z-5)*em3 - (a+n).pow(-z-7)*em4
      + (a+n).pow(-z-9)*em5;
    if res==ores && res==oores {
      //::log::debug!("hurwitz_zeta_em::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), z, a, n);
      break;
    }
  }
  res
}

}

