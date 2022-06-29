use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(Legendre);

impl<V: Value+Trig> OrthogonalPolynomial<V> for Legendre<V> {
  fn domain(&self) -> (V, V) {
    unimplemented!()
  }
  fn coeff(&self, n: usize, k: usize) -> V {
    unimplemented!()
  }
  fn scale(&self, n: usize) -> V {
    unimplemented!()
  }
  fn value(&self, n: usize, x: V) -> V {
    unimplemented!()
  }
  fn weight(&self, n: usize, k: usize) -> V {
    unimplemented!()
  }
  fn zero(&self, n: usize, k: usize) -> V {
    unimplemented!()
  }
  fn kernel(&self, x: V) -> V {
    unimplemented!()
  }
  fn coeffs(&self, n: usize) -> Vec<V> {
    unimplemented!()
  }
  fn weights(&self, n: usize) -> Vec<V> {
    unimplemented!()
  }
  fn zeros(&self, n: usize) -> Vec<V> {
    unimplemented!()
  }
  fn poly(&self, n: usize) -> Poly<V> {
    unimplemented!()
  }
}