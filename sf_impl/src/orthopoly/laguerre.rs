use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::numbers::{sf_factorial};
use crate::gamma::*;
use crate::exp::{Exp,sf_exp};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Laguerre<V:Value> {
  pub alpha: V,
}

impl<V:Value> Laguerre<V> {
  pub fn new(alpha:V) -> Self {
    Laguerre { alpha }
  }
}

impl<V:RealValue+Exp+Gamma+Float+Power<V>> OrthogonalPolynomial<V> for Laguerre<V> {
  fn domain(&self) -> (V, V) {
    (ι(0), V::infinity)
  }
  fn scale(&self, n: isize) -> V {
    let a = self.alpha;
    sf_sqrt(sf_factorial(n as usize):V / sf_gamma(a+n+1))
  }
  fn value(&self, n: isize, x: V) -> V {
    let a = self.alpha;
    match n {
      0 => ι(1),
      1 => a + 1 - x,
      _ => {
        let mut x1 = ι(1);
        let mut x0 = a + 1 - x;
        for k in 2..(n+1) {
          let x2 = x1;
          x1 = x0;
          x0 = (x1*(a-x+(2*k-1)) - x2*(a+(k-1)))/k;
        }
        x0
      }
    }
  }
  fn kernel(&self, x: V) -> V {
    sf_exp(-x)*x.pow(self.alpha)
  }
  fn coeff(&self, n: isize, k: isize) -> V {
    self.coeffs(n)[k as usize]
  }
  fn coeffs(&self, n: isize) -> Vec<V> {
    self.poly(n).0
  }
  fn weight(&self, n: isize, k: isize) -> V {
    self.weights(n)[k as usize]
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
  fn zero(&self, n: isize, k: isize) -> V {
    self.zeros(n)[k as usize]
  }
  fn zeros(&self, n: isize) -> Vec<V> {
    match n {
      0 => vec![],
      1 => vec![V::zero],
      _ => {
        let a = self.alpha;
        let mut d : Vec<_> = (0..n).map(|k| a + (2*k+1)).collect();
        let mut e : Vec<_> = (0..n).map(|k| sf_sqrt((a+k)*k)).collect();
        crate::matrix::eig_symtrid(&mut d, &mut e);
        //d.sort(); // TODO: sort out traits later
        let dself = Laguerre::new(a + 1);
        let pol = |z|{
          let fx = self.value(n, z);
          let dfx = -dself.value(n-1, z);
          z - fx/dfx
        };
        // polish zeros
        for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        //for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        d
      }
    }
  }
  fn poly(&self, n: isize) -> Poly<V> {
    let a = self.alpha;
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![a+1,ι(-1)]),
      _ => {
        let mut t1 = Poly(vec![ι(1)]);
        let mut t0 = Poly(vec![a+1,ι(-1)]);
        for k in 2..=n {
          // TODO: polynomials need ergonomics work!
          let mut t2: Poly<V> = t1.clone();
          t1 = t0;
          let mut t1x = t1.clone();
          t1x.shift(1);
          t0 = (&t1*(a+(2*k-1)) - &t1x - &(t2*(a+k-1)))/ι(k);
        }
        t0
      }
    }
  }
}
