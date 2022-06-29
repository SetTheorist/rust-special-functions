use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(ChebyshevT);

impl<V: Value+Trig> OrthogonalPolynomial<V> for ChebyshevT<V> {
  fn domain(&self) -> (V, V) { (ι(-1), ι(1)) }

  fn scale(&self, n: usize) -> V {
    match n {
      0 => sf_sqrt(V::PI),
      _ => sf_sqrt(V::PI/2),
    }
  }

  fn value(&self, n: usize, x: V) -> V {
    match n {
      0 => ι(1),
      1 => x,
      _ => {
        let mut vm1: V = ι(1);
        let mut vm0: V = x;
        for _ in 2..=n {
          let vm2 = vm1;
          vm1 = vm0;
          vm0 = x * vm1 * 2 - vm2;
        }
        vm0
      }
    }
  }

  fn poly(&self, n: usize) -> Poly<V> {
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(0), ι(1)]),
      _ => {
        let mut t0: Poly<V> = Poly(vec![ι(1)]);
        let mut t1: Poly<V> = Poly(vec![ι(0), ι(1)]);
        for _ in 2..=n {
          // let t2 = t1.shift(1)*2 - t0;
          let mut t2: Poly<V> = t1.clone();
          t2.shift(1);
          t2 *= ι(2);
          t2 -= &t0;

          t0 = t1;
          t1 = t2;
        }
        t1
      }
    }
  }

  fn coeffs(&self, n: usize) -> Vec<V> { self.poly(n).0 }

  fn coeff(&self, n: usize, k: usize) -> V { self.coeffs(n)[k] }

  fn weight(&self, n: usize, _k: usize) -> V { V::PI / (n as isize) }

  fn weights(&self, n: usize) -> Vec<V> { vec![V::PI/(n as isize); n] }

  fn zero(&self, n: usize, k: usize) -> V { 
    let k = (n-1-k);
    if n%2 == 1 && k==(n-1)/2 {
      V::zero
    } else {
      let kk = (2*k+1) as isize;
      sf_cos(V::FRAC_PI_2 * kk / (n as isize))
    }
  }

  fn zeros(&self, n: usize) -> Vec<V> { 
    let mut res = vec![V::zero; n];
    for k in 0..(n/2) {
      let kk = (2*k+1) as isize;
      let c = sf_cos(V::FRAC_PI_2 * kk / (n as isize));
      res[k] = -c;
      res[n-1-k] = c;
    }
    res
  }

  fn kernel(&self, x: V) -> V { (ι(1): V - x * x).sqrt().recip() }
}
