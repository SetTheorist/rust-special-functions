
pub trait Debye {
  // $D_n(x) = \int_0^x \frac{t^n}{e^t - 1}\,dt$
  fn debye(self, n:usize) -> Self;
  // $\tilde{D}_n(x) = \frac{n}{x^n} D_n(x) = \frac{n}{x^n}\int_0^x \frac{t^n}{e^t - 1}\,dt$
  fn debye_scaled(self, n:usize) -> Self;
}

// asymptotics for fixed cases:
// n=1: exp(-x)*-1*(1) + log(1-cosh(x)+sinh(x))*x + pi^2/6
// n=2: exp(-x)*-2*(x+1) + log(1-cosh(x)+sinh(x))*x^2 + 2*zeta(3)
// n=3: exp(-x)*-3*(x^2+2x+2) + log(1-cosh(x)+sinh(x))*x^3 + pi^4/15 {{6*zeta(4)}}
// n=4: exp(-x)*-4*(x^3+3x^2+6x+6) + log(1-cosh(x)+sinh(x))*x^4 + 24*zeta(5)
// n=5: exp(-x)*-5*(x^4+4x^3+12x^2+24x+24) + log(1-cosh(x)+sinh(x))*x^5 + 8*pi^6/63 {{5!*zeta(6)}}
// n=6: exp(-x)*-6*(x^5+5x^4+20x^3+60x^2+120x+120) + log(1-cosh(x)+sinh(x))*x^6 + 720*zeta(7)
//
// conjectured general form:
// Let {n,k} = n(n-1)(...)(n-(k-1))
// exp(-x)*-n*(x^n+{n,1}x^(n-1)+{n,2}x^(n-2)+...+{n,n-1}x+{n,n}1) + log(1-cosh(x)+sinh(x))*x^n + n!*zeta(n+1)


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

  pub fn debye_scaled_series_1<V:Value>(n:isize, z:V) -> V {
    let mut res = V::one/n - z/(2*(n+1));
    let z2 = z * z;
    let mut zpow = V::one;
    for k in 1..300 {
      zpow *= z2;
      let old = res;
      res += zpow * sf_bernoulli_number_scaled_approx(2*k as usize) / (2*k + n);
      if res == old { break; }
    }
    res *= n;
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

  /*
  use crate::log::{*};
  use crate::trig::{*};
  // exp(-x)*-n*(x^n+{n,1}x^(n-1)+{n,2}x^(n-2)+...+{n,n-1}x+{n,n}1) + log(1-cosh(x)+sinh(x))*x^n + n!*zeta(n+1)
  pub fn debye_asympt<V:Value+Log+Trig>(n:isize, z:V) -> V {
    let mut poly = z.pow(n);
    let pp : V = ι(1);
    for j in 0..n {
      pp *= n-j;
      poly += z.pow(n-1-j) * pp;
    }
    sf_exp(-z) * -n * poly + sf_log(sf_cosh(z)+sf_sinh(z)-1)*z.pow(n) + sf_factorial(n)*sf_zeta(ι(n+1):V)
  }
  */
}



