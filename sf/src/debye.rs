
pub trait Debye {
  fn debye(self, n:usize) -> Self;
  fn debye_scaled(self, n:usize) -> Self;
}

pub mod impls {
  use crate::traits::*;
  use crate::numbers::{sf_bernoulli_number_scaled_approx};
  use crate::exp::{Exp,sf_exp};

  pub fn debye_real<V:RealValue>(n:isize, z:V) -> V {
    if z < V::zero {
      (-z).pow(n+1).pari(n)/(n+1) - debye_real(n, -z)
    } else {
      //if μ(z) < μ(ι(2):V) {
        debye_series_1(n, z)
      //} else if μ(z) < μ(ι(n):V) {
      //  debye_coterm_2(n, z)
      //} else {
      //  sf_factorial(n)*sf_zeta(n+1) - debye_coint(n, k)
      //}
    }
  }

  pub fn debye_series_1<V:Value>(n:isize, z:V) -> V {
    let mut res = V::one/n - z/(2*(n+1));
    let z2 = z * z;
    let mut zpow = V::one;
    for k in 1..300 {
      zpow *= z2;
      let old = res;
      res += zpow * sf_bernoulli_number_scaled_approx(2*k as usize) / (2*k + n);
      if res == old { break; }
    }
    res *= z.pow(n);
    res
  }

  pub fn debye_coint<V:Value+Exp>(n:isize, z:V) -> V {
    let mut res = V::zero;
    for k in 1..1000 {
      let old = res;
      res += sf_exp(-z*k) * coterm(n, k, z);
      if old == res { break; }
    }
    res
  }
  pub fn coterm<V:Value>(n:isize, k:isize, z:V) -> V {
    let mut res = z.pow(n) / k;
    let mut term = res;
    for j in (0..n).rev() {
      term *= ι(j+1):V / (z*k);
      res += term;
    }
    res
  }

  // alternative formulation
  /*
  pub fn debye_coterm_2<V:Value>(n:isize, z:V) -> V {
    let zeta = sf_zeta_m1(n+1);
    let eee = sf_exp(-z) * sf_exp_men(n+1, z);
    res = zeta + eee;
    for k in 2..1000 {
      let old = res;
      res -= sf_exp(-z*k) / (ι(k):V).pow(n+1) * sf_expn(n, z*k);
      if old == res { break; }
    }
    res * sf_factorial(n)
  }
  */
}
