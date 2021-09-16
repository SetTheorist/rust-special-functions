use crate::traits::{*};
use std::marker::PhantomData;

pub mod chebyshev_t;

trait OrthogonalPolynomial<V:Value> {
  fn domain(&self) -> (V,V);
  fn coeff(&self, n:usize, k:usize) -> V;
  fn scale(&self, n:usize) -> V; // and scale_squared?
  fn value(&self, n:usize, x:V) -> V;
  fn weight(&self, n:usize, k:usize) -> V;
  fn zero(&self, n:usize, k:usize) -> V;
  fn kernel(&self, x:V) -> V;

  fn coeffs(&self, n:usize) -> Vec<V>;
  fn weights(&self, n:usize) -> Vec<V>;
  fn zeros(&self, n:usize) -> Vec<V>;
  // (also variants for j'th derivative)

  // TODO: maybe return more information...
  //fn integrate<F:Fn(V)->V>(&self, n:usize, f:F) -> V;
  // TODO: maybe do this kind of thing instead?
  //fn integrator(&self, n:usize) -> impl Integrator<V>;
}

/*
// (then we can have other integration techniques with common interface...)
trait Integrator<V> {
  fn integrate<F:Fn(V)->V>(&self, n:usize, f:F) -> V;
}
*/

macro_rules! empty_type {
  ($t:ident) => {
    #[derive(Clone,Copy,Debug,Eq,PartialEq)]
    struct $t<V:Value> { _phantom : PhantomData<*const V> }
    impl <V:Value> $t<V> {
      pub fn new() -> Self { $t{_phantom:PhantomData} }
    }
  }
}

empty_type!(ChebyshevU);
struct Gegenbauer<V:Value>{alpha:V}
empty_type!(HermiteH);
empty_type!(HermiteHe);
struct Jacobi<V:Value>{a:V,b:V}
struct Laguerre<V:Value>{alpha:V}
empty_type!(Legendre);

////////////////////////////////////////////////////////////////////////////////
