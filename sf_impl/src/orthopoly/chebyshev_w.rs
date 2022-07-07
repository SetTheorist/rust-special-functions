use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(ChebyshevW);

impl<V: Value+Trig> OrthogonalPolynomial<V> for ChebyshevW<V> {
  fn domain(&self) -> (V, V) {
    (ι(-1), ι(1))
  }

  fn kernel(&self, x: V) -> V {
    sf_sqrt((-x+1)/(x+1))
  }

  fn scale(&self, n: isize) -> V {
    V::SQRTPI
  }

  fn value(&self, n: isize, x: V) -> V {
    match n {
      0 => ι(1),
      1 => x*2+1,
      _ => {
        let mut vm1: V = ι(1);
        let mut vm0: V = x*2+1;
        for _ in 2..=n {
          let vm2 = vm1;
          vm1 = vm0;
          vm0 = x * vm1 * 2 - vm2;
        }
        vm0
      }
    }
  }

  fn poly(&self, n: isize) -> Poly<V> {
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(1), ι(2)]),
      _ => {
        let mut t0: Poly<V> = Poly(vec![ι(1)]);
        let mut t1: Poly<V> = Poly(vec![ι(1), ι(2)]);
        for _ in 2..=n {
          let t2 = t1.x(1)*ι(2):V - &t0;
          t0 = t1;
          t1 = t2;
        }
        t1
      }
    }
  }

  // TODO: cleanup
  fn weight(&self, n: isize, k: isize) -> V {
    let xk = self.zero(n, k);
    let k = k+1;
    V::PI*2/(2*n+1)*(-xk+1)
  }

  fn weights(&self, n: isize) -> Vec<V> {
    (0..n).map(|k|self.weight(n,k)).collect()
  }

  fn zero(&self, n: isize, k: isize) -> V {
    let k = (n-1-k);
    sf_cos(V::PI * (2*k+2) / (2*n+1))
  }

  fn zeros(&self, n: isize) -> Vec<V> {
    (0..n).map(|k|self.zero(n,k)).collect::<Vec<_>>()
  }
}
