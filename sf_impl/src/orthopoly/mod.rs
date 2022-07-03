use crate::poly::Poly;
use crate::traits::*;
use std::marker::PhantomData;

macro_rules! empty_type {
  ($t:ident) => {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct $t<V: Value> {
      _phantom: PhantomData<*const V>,
    }
    impl<V: Value> $t<V> {
      pub fn new() -> Self { $t { _phantom: PhantomData } }
    }
  };
}

pub mod chebyshev_t;
pub mod chebyshev_u;
pub mod gegenbauer;
pub mod hermite_h;
pub mod hermite_he;
pub mod laguerre;
pub mod legendre;

//empty_type!(ChebyshevTx);
//empty_type!(ChebyshevUx);
//empty_type!(Legendrex);
//empty_type!(ChebyshevV);
//empty_type!(ChebyshevW);

struct Jacobi<V: Value> {
  alpha: V,
  beta: V,
}

// NB use nalgebra for eigenvalues ...

pub trait OrthogonalPolynomial<V: Value> {
  /// domain over which the polynomials are defined
  fn domain(&self) -> (V, V);

  fn scale(&self, n: isize) -> V; // and scale_squared?

  // TODO: vector of values [0..n]
  fn value(&self, n: isize, x: V) -> V;

  fn kernel(&self, x: V) -> V;

  fn coeffs(&self, n: isize) -> Vec<V> {
    self.poly(n).0
  }

  /// the k'th coefficient in the degree n polynomial
  fn coeff(&self, n: isize, k: isize) -> V {
    self.coeffs(n)[k as usize]
  }

  fn weights(&self, n: isize) -> Vec<V> {
    match n {
      0 => vec![],
      1 => vec![ι(1)],
      _ => {
        let zs = self.zeros(n);
        let nrm : Vec<_> = (0..n).map(|k|self.scale(k)).collect();
        let mut res = vec![V::zero; n as usize];
        for j in 0..n {
          for k in 0..n {
            res[k as usize] += (self.value(j, zs[k as usize]) * nrm[j as usize]).sqr();
          }
        }
        for j in 0..(n as usize) { res[j] = res[j].recip(); }
        res
      }
    }
  }

  fn weight(&self, n: isize, k: isize) -> V {
    self.weights(n)[k as usize]
  }

  fn zeros(&self, n: isize) -> Vec<V>;

  fn zero(&self, n: isize, k: isize) -> V {
    self.zeros(n)[k as usize]
  }

  // (also variants for j'th derivative)

  // TODO: vector for [0..n]
  fn poly(&self, n: isize) -> Poly<V>;

  // TODO: maybe return more information...
  //fn integrate<F:Fn(V)->V>(&self, n:isize, f:F) -> V;
  // TODO: maybe do this kind of thing instead?
  //fn integrator(&self, n:isize) -> impl Integrator<V>;
}

/*
// (then we can have other integration techniques with common interface...)
trait Integrator<V> {
  fn integrate<F:Fn(V)->V>(&self, n:isize, f:F) -> V;
}
*/

////////////////////////////////////////////////////////////////////////////////
