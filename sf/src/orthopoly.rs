/*
use crate::value::{Value};
use std::marker::PhantomData;

trait OrthogonalPolynomial<V:Value> {
  fn domain(&self) -> (V,V);
  fn coeff(&self, n:usize, k:usize) -> V;
  fn scale(&self, n:usize) -> V;
  fn value(&self, n:usize, x:V) -> V;
  fn weight(&self, n:usize, k:usize) -> V;
  fn zero(&self, n:usize, k:usize) -> V;

  fn coeffs(&self, n:usize) -> Vec<V>;
  fn weights(&self, n:usize) -> Vec<V>;
  fn zeros(&self, n:usize) -> Vec<V>;
  // (also variants for j'th derivative)
}

macro_rules! empty_type {
  ($t:ident) => {
    #[derive(Clone,Copy,Debug,Eq,PartialEq)]
    struct $t<V:Value> { _phantom : PhantomData<*const V> }
    impl <V:Value> $t<V> {
      pub fn new() -> Self { $t{_phantom:PhantomData} }
    }
  }
}

empty_type!(ChebyshevT);
empty_type!(ChebyshevU);
struct Gegenbauer<V:Value>{alpha:V}
empty_type!(HermiteH);
empty_type!(HermiteHe);
struct Jacobi<V:Value>{a:V,b:V}
struct Laguerre<V:Value>{alpha:V}
empty_type!(Legendre);
*/
