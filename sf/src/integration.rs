use crate::traits::*;

// TODO: lots of other possibilities,
// including returning a lot more information...
pub trait Integrator<V> {
  fn domain(&self) -> (V, V);
  fn integrate<F>(&self, f: F) -> V
  where F: Fn(V) -> V;
}

// use trapezoidal rule to integrate on interval [a,b]
// will actually evaluate f at n+1 points
// non-adaptive, simply computes the sum
pub struct Trapezoidal<V> {
  a: V,
  b: V,
  h: V,
  n: isize,
}

impl<V: Value> Trapezoidal<V> {
  pub fn new(a: V, b: V, n: usize) -> Self {
    let n = (n as isize).max(1);
    let h = (b - a) / n;
    Trapezoidal { a, b, h, n }
  }
}

impl<V: Value> Integrator<V> for Trapezoidal<V> {
  fn domain(&self) -> (V, V) { (self.a, self.b) }
  fn integrate<F>(&self, f: F) -> V
  where F: Fn(V) -> V {
    let mut sum: V = (f(self.a) + f(self.b)) / 2;
    for i in 1..self.n {
      sum += f(self.a + self.h * i);
    }
    sum * self.h
  }
}
