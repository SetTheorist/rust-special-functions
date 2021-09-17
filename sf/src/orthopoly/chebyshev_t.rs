use crate::traits::{*};
use crate::orthopoly::{*};
use crate::poly::{Poly};

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

  fn poly(&self, n:usize) -> Poly<V> {
    if n == 0 {
      Poly(vec![ι(1)])
    } else if n == 1 {
      Poly(vec![ι(0), ι(1)])
    } else {
      let mut t0 : Poly<V> = Poly(vec![ι(1)]);
      let mut t1 : Poly<V> = Poly(vec![ι(0), ι(1)]);
      for _ in 2..=n {
        // let t2 = t1.shift(1)*2 - t0;
        let mut t2 : Poly<V> = t1.clone();
        t2.shift(1);
        t2 *= ι(2);
        t2 -= &t0;

        t0 = t1;
        t1 = t2;
      }
      t1
    }
  }

  fn coeffs(&self, n:usize) -> Vec<V> {
    self.poly(n).0
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
