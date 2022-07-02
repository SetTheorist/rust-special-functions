use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gegenbauer<V:Value> {
  pub lambda: V,
}
impl<V:Value> Gegenbauer<V> {
  pub fn new(lambda:V) -> Self {
    Gegenbauer { lambda }
  }
}

impl<V:Value> OrthogonalPolynomial<V> for Gegenbauer<V> {
  fn domain(&self) -> (V, V) {
    (ι(-1),ι(1))
  }
  fn coeff(&self, n: isize, k: isize) -> V {
    unimplemented!()
  }
  fn scale(&self, n: isize) -> V {
    unimplemented!()
  }
  fn value(&self, n: isize, x: V) -> V {
    unimplemented!()
  }
  fn weight(&self, n: isize, k: isize) -> V {
    unimplemented!()
  }
  fn zero(&self, n: isize, k: isize) -> V {
    unimplemented!()
  }
  fn kernel(&self, x: V) -> V {
    unimplemented!()
  }
  fn coeffs(&self, n: isize) -> Vec<V> {
    unimplemented!()
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