use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::exp::{Exp,sf_exp};
use crate::numbers::{sf_factorial};

empty_type!(HermiteHe);

// TODO: weighting by 1/sqrt(2pi)?
impl<V:RealValue+Float+Exp> OrthogonalPolynomial<V> for HermiteHe<V> {
  fn domain(&self) -> (V, V) {
    (-V::infinity, V::infinity)
  }

  fn scale(&self, n: isize) -> V {
    sf_sqrt(V::SQRT2PI*sf_factorial::<V>(n as usize)).recip()
  }

  fn value(&self, n: isize, x: V) -> V {
    match n {
      0 => ι(1),
      1 => x,
      _ => {
        let mut z1 = ι(1);
        let mut z0 = x;
        for k in 2..(n+1) {
          let z2 = z1;
          z1 = z0;
          z0 = z1*x - z2*(k-1);
        }
        z0
      }
    }
  }

  fn kernel(&self, x: V) -> V {
    sf_exp(-x*x/2)*V::FRAC_1_SQRT2PI
  }

  fn zeros(&self, n: isize) -> Vec<V> {
    let sqrt2 = sf_sqrt(ι(2):V);
    super::hermite_h::HermiteH::new().zeros(n).into_iter().map(|z:V|z*sqrt2).collect()
  }

  fn poly(&self, n: isize) -> Poly<V> {
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(0),ι(2)]),
      _ => {
        let mut v1 = Poly(vec![ι(1)]);
        let mut v0 = Poly(vec![ι(0),ι(2)]);
        for k in 2..(n+1) {
          let v2 = v1;
          v1 = v0;
          v0 = v1.x(1) - &(v2*ι(k-1):V);
        }
        v0
      }
    }
  }
}