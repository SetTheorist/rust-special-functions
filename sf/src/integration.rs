use crate::traits::*;

// Trapezoidal
// Erf / Tanh/Sinh ("stretched trapezoidal")
// Simpson's
// Gaussian &c.
// adaptive? (may not need for purposes of spec.fun. implementations)

// TODO: lots of other possibilities,
// including returning a lot more information...
// allow separate domain & range types?
pub trait Integrator<X> {
  fn domain(&self) -> (X, X);
  fn integrate<F>(&self, f: F) -> X
    where F: Fn(X) -> X;
}

////////////////////////////////////////////////////////////////////////////////

// use trapezoidal rule to integrate on interval [a,b]
// will actually evaluate f at n+1 points
// non-adaptive, simply computes the sum
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Trapezoidal<X> {
  a: X,
  b: X,
  h: X,
  n: isize,
}

impl<X:Value> Trapezoidal<X> {
  pub fn new(a:X, b:X, n:usize) -> Self {
    let n = (n as isize).max(1);
    let h = (b - a) / n;
    Trapezoidal { a, b, h, n }
  }
}

impl<X:Value> Integrator<X> for Trapezoidal<X> {
  fn domain(&self) -> (X, X) { (self.a, self.b) }
  fn integrate<F:Fn(X)->X>(&self, f: F) -> X {
    let mut sum: X = (f(self.a) + f(self.b)) / 2;
    for i in 1..self.n {
      sum += f(self.a + self.h * i);
    }
    sum * self.h
  }
}

////////////////////////////////////////////////////////////////////////////////

use crate::trig::{*};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TanhRule<X> {
  a: X,
  b: X,
  h: X,
  n: isize,
}

impl<X:Value> TanhRule<X> {
  pub fn new(a:X, b:X, n:usize) -> Self {
    let n = ((n/2) as isize).max(1);
    let h = X::PI/sf_sqrt(ι(2*n):X);
    TanhRule { a, b, h, n }
  }
}

// TODO: cleanup to allow separate domain & range
impl<X:Value+Trig+std::fmt::LowerExp> Integrator<X> for TanhRule<X> {
  fn domain(&self) -> (X, X) { (self.a, self.b) }
  fn integrate<F:Fn(X)->X>(&self, f: F) -> X {
    let mut sum = X::zero;
    for k in -self.n..(self.n+1) {
      let t = self.h * k;
      let xk = (self.b+self.a)/2 + (self.b-self.a)/2 * sf_tanh(t);
      let wk = (self.b - xk)*(xk - self.a)*2/(self.b - self.a);
      let fx = f(xk);
      sum += self.h * wk * fx;
    }
    sum
  }
}


