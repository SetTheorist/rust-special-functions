use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::gamma::*;
use crate::numbers::{sf_factorial, sf_pochhammer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gegenbauer<V:Value> {
  pub lambda: V,
}
impl<V:Value> Gegenbauer<V> {
  // \lambda > -1/2
  pub fn new(lambda:V) -> Self {
    Gegenbauer { lambda }
  }
}

impl<V:Value+Power+Gamma> OrthogonalPolynomial<V> for Gegenbauer<V> {
  fn domain(&self) -> (V, V) {
    (ι(-1),ι(1))
  }
  fn scale(&self, n: isize) -> V {
    let lam = self.lambda;
    sf_pochhammer(lam+0.5, n) * sf_sqrt((lam+n)*2*sf_factorial::<V>(n as usize)*sf_gamma(lam*2+n))
      / (sf_pochhammer(lam*2, n)*(ι(2):V).pow(lam)*sf_gamma(lam+n+0.5))
  }
  fn value(&self, n: isize, x: V) -> V {
    let lam = self.lambda;
    match n {
      0 => ι(1),
      1 => x*lam*2,
      _ => {
        let mut z1 = ι(1);
        let mut z0 = x*lam*2;
        for k in 2..(n+1) {
          let z2 = z1;
          z1 = z0;
          z0 = (z1*(x*2*(lam+(k-1))) - z2*(lam*2+(k-2))) / k;
        }
        z0
      }
    }
  }
  fn kernel(&self, x: V) -> V {
    (-x.sqr() + 1).pow(self.lambda - 0.5)
  }
  fn weights(&self, n: isize) -> Vec<V> {
    unimplemented!()
  }
  fn zeros(&self, n: isize) -> Vec<V> {
    unimplemented!()
  }
  fn poly(&self, n: isize) -> Poly<V> {
    unimplemented!()
  }
}