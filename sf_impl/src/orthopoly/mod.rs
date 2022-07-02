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
pub mod laguerre;
pub mod legendre;

empty_type!(ChebyshevTx);
empty_type!(ChebyshevUx);
empty_type!(Legendrex);
empty_type!(ChebyshevV);
empty_type!(ChebyshevW);

empty_type!(HermiteH);
empty_type!(HermiteHe);
struct Jacobi<V: Value> {
  alpha: V,
  beta: V,
}

// NB use nalgebra for eigenvalues ...

pub trait OrthogonalPolynomial<V: Value> {
  /// domain over which the polynomials are defined
  fn domain(&self) -> (V, V);
  /// the k'th coefficient in the degree n polynomial
  fn scale(&self, n: isize) -> V; // and scale_squared?
  fn value(&self, n: isize, x: V) -> V;
  fn kernel(&self, x: V) -> V;

  fn coeffs(&self, n: isize) -> Vec<V> {
    self.poly(n).0
  }
  fn coeff(&self, n: isize, k: isize) -> V {
    self.coeffs(n)[k as usize]
  }

  fn weights(&self, n: isize) -> Vec<V>;
  fn weight(&self, n: isize, k: isize) -> V {
    self.weights(n)[k as usize]
  }

  fn zeros(&self, n: isize) -> Vec<V>;
  fn zero(&self, n: isize, k: isize) -> V {
    self.zeros(n)[k as usize]
  }

  // (also variants for j'th derivative)

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
