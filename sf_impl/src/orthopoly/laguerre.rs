use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::numbers::{sf_factorial_approx};
use crate::gamma::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Laguerre<V:Value> {
  pub alpha: V,
}

impl<V:Value> Laguerre<V> {
  pub fn new(alpha:V) -> Self {
    Laguerre { alpha }
  }
}

impl<V:RealValue+Gamma+Float> OrthogonalPolynomial<V> for Laguerre<V> {
  fn domain(&self) -> (V, V) {
    (ι(0), V::infinity)
  }
  fn scale(&self, n: isize) -> V {
    // TODO: factorial
    let a = self.alpha;
    sf_sqrt(ι(sf_factorial_approx(n as usize)):V / sf_gamma(a+n+1))
  }
  fn value(&self, n: isize, x: V) -> V {
    let a = self.alpha;
    match n {
      0 => ι(1),
      1 => a + 1 - x,
      _ => {
        let mut x1 = ι(1);
        let mut x0 = a + 1 - x;
        for k in 2..(n+1) {
          let x2 = x1;
          x1 = x0;
          x0 = (x1*(a-x+(2*k-1)) - x2*(a+(k-1)))/k;
        }
        x0
      }
    }
  }
  fn kernel(&self, x: V) -> V {
    unimplemented!()
  }
  fn coeff(&self, n: isize, k: isize) -> V {
    self.coeffs(n)[k as usize]
  }
  fn coeffs(&self, n: isize) -> Vec<V> {
    self.poly(n).0
  }
  fn weight(&self, n: isize, k: isize) -> V {
    self.weights(n)[k as usize]
  }
  fn weights(&self, n: isize) -> Vec<V> {
    unimplemented!()
  }
  fn zero(&self, n: isize, k: isize) -> V {
    self.zeros(n)[k as usize]
  }
  fn zeros(&self, n: isize) -> Vec<V> {
    unimplemented!()
  }
  fn poly(&self, n: isize) -> Poly<V> {
    let a = self.alpha;
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![a+1,ι(-1)]),
      _ => {
        let mut t1 = Poly(vec![ι(1)]);
        let mut t0 = Poly(vec![a+1,ι(-1)]);
        for k in 2..=n {
          // TODO: polynomials need ergonomics work!
          let mut t2: Poly<V> = t1.clone();
          t1 = t0;
          let mut t1x = t1.clone();
          t1x.shift(1);
          t0 = (&t1*(a+(2*k-1)) - &t1x - &(t2*(a+k-1)))/ι(k);
        }
        t0
      }
    }
  }
}
