use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::trig::*;

empty_type!(Legendre);

impl<V:RealValue+Trig+Float> OrthogonalPolynomial<V> for Legendre<V> {

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

  fn zeros(&self, n: isize) -> Vec<V> {
    if n == 0 {
      return vec![];
    } else if n == 1{
      return vec![V::zero];
    }
    //TODO: this will require integrating nalgebra traits
    //let mut m = nalgebra::DMatrix::<V>::identity(n as usize, n as usize);
    let mut d = vec![V::zero; n as usize];
    let mut e : Vec<_> = (0..n).map(|k| ι(k):V / sf_sqrt(ι(4*k*k-1):V)).collect();
    crate::matrix::eig_symtrid(&mut d, &mut e);
    //d.sort(); // TODO: sort out traits later
    let pol = |z|{
      let fx = self.value(n, z);
      let dfx = (-z*n*fx + self.value(n-1, z)*n) / (-z.sqr()+1);
      z - fx/dfx
    };
    // polish zeros
    for i in 0..n { d[i as usize] = pol(d[i as usize]); }
    // polish zeros (?)
    //for i in 0..n { d[i as usize] = pol(d[i as usize]); }
    d
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