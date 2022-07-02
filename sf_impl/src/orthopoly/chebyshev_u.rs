use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(ChebyshevU);

impl<V: Value+Trig> OrthogonalPolynomial<V> for ChebyshevU<V> {
  fn domain(&self) -> (V, V) { (ι(-1), ι(1)) }

  fn scale(&self, n: isize) -> V {
    // \sqrt{\frac{2}{\pi}}
    sf_sqrt(V::FRAC_1_SQRT2PI * 2)
  }

  fn value(&self, n: isize, x: V) -> V {
    match n {
      0 => ι(1),
      1 => x*2,
      _ => {
        let mut vm1: V = ι(1);
        let mut vm0: V = x*2;
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
      1 => Poly(vec![ι(0), ι(1)]),
      _ => {
        let mut t0: Poly<V> = Poly(vec![ι(1)]);
        let mut t1: Poly<V> = Poly(vec![ι(0), ι(2)]);
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

  fn weights(&self, n: isize) -> Vec<V> {
    (1..(n+1)).map(|k|
        sf_sin(V::PI*k/(n+1)).sqr() * V::PI / (n+1)
    ).collect()
  }

  fn zero(&self, n: isize, k: isize) -> V { 
    let k = (n-1-k);
    if n%2 == 1 && k==(n-1)/2 {
      V::zero
    } else {
      sf_cos(V::PI * (k+1) / (n+1))
    }
  }

  fn zeros(&self, n: isize) -> Vec<V> { 
    let mut res = vec![V::zero; n as usize];
    for k in (0..(n/2)).rev() {
      let c = sf_cos(V::PI * (k+1) / (n+1));
      res[k as usize] = -c;
      res[(n-1-k) as usize] = c;
    }
    res
  }

  fn kernel(&self, x: V) -> V { (ι(1): V - x * x).sqrt() }
}
