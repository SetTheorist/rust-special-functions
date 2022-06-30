use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(Legendre);

impl<V: Value+Trig> OrthogonalPolynomial<V> for Legendre<V> {

  fn domain(&self) -> (V, V) {
    (ι(-1),ι(1))
  }

  fn scale(&self, n: isize) -> V {
    sf_sqrt(ι(2*n+1):V/2)
  }

  fn value(&self, n: isize, x: V) -> V {
    match n {
      0 => V::one,
      1 => x,
      _ => {
        let mut vm1: V = V::one;
        let mut vm0: V = x;
        for k in 2..=n {
          let vm2 = vm1;
          vm1 = vm0;
          vm0 = (x * vm1 * (2*k-1) - vm2*(k-1))/k;
        }
        vm0
      }
    }
  }

  fn weight(&self, n: isize, k: isize) -> V {
    self.weights(n)[k as usize]
  }

  fn zero(&self, n: isize, k: isize) -> V {
    self.zeros(n)[k as usize]
  }

  fn kernel(&self, _x: V) -> V {
    V::one
  }

  fn weights(&self, n: isize) -> Vec<V> {
    unimplemented!()
  }

  fn zeros(&self, n: isize) -> Vec<V> {
    //TODO: this will require integrating nalgebra traits
    //let mut m = nalgebra::DMatrix::<V>::identity(n as usize, n as usize);
    unimplemented!()
  }

  fn coeff(&self, n: isize, k: isize) -> V {
    self.coeffs(n)[k as usize]
  }

  fn coeffs(&self, n: isize) -> Vec<V> {
    self.poly(n).0
  }

  fn poly(&self, n: isize) -> Poly<V> {
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(0),ι(1)]),
      _ => {
        let mut t0 = Poly(vec![ι(1)]);
        let mut t1 = Poly(vec![ι(0),ι(1)]);
        for k in 2..=n {
          // TODO: polynomials need ergonomics work!
          let mut t2: Poly<V> = t1.clone();
          t2.shift(1);
          t2 *= ι(2*k-1);
          let mut t0x = t0.clone();
          t0x *= ι(k-1);
          t2 -= &t0x;
          t2 /= ι(k);

          t0 = t1;
          t1 = t2;
        }
        t1
      }
    }
  }
}