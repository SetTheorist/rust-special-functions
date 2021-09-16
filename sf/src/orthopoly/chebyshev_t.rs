use crate::traits::{*};
use crate::orthopoly::{*};

// TODO: sort out how to import this macro
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

impl<V:Value> OrthogonalPolynomial<V> for ChebyshevT<V> {
  fn domain(&self) -> (V,V) {
    (ι(-1), ι(1))
  }
  fn coeff(&self, n:usize, k:usize) -> V {
    unimplemented!()
  }
  fn scale(&self, n:usize) -> V {
    let PI : V = ι(3.14159265358979); // TODO: use Constants
    if n==0 {
      PI.sqrt()
    } else {
      (PI/2).sqrt()
    }
  }
  fn value(&self, n:usize, x:V) -> V {
    if n == 0 {
      ι(1)
    } else if n == 1 {
      x
    } else {
      let mut vm1 : V = ι(1);
      let mut vm0 : V = x;
      for _ in 2..=n {
        let vm2 = vm1;
        vm1 = vm0;
        vm0 = x*vm1*2 - vm2;
      }
      vm0
    }
  }
  fn weight(&self, n:usize, k:usize) -> V {
    unimplemented!()
  }
  fn zero(&self, n:usize, k:usize) -> V {
    unimplemented!()
  }

  fn coeffs(&self, n:usize) -> Vec<V> {
    unimplemented!()
  }
  fn weights(&self, n:usize) -> Vec<V> {
    unimplemented!()
  }
  fn zeros(&self, n:usize) -> Vec<V> {
    unimplemented!()
  }
  // (also variants for j'th derivative)

  fn kernel(&self, x:V) -> V {
    (ι(1):V - x*x).sqrt().recip()
  }
}
