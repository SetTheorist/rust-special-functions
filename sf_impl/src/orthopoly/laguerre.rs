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
          let t2: Poly<V> = t1.clone();
          t1 = t0;
          t0 = (&t1*(a+(2*k-1)) - &t1.x(1) - &(t2*(a+k-1)))/ι(k);
        }
        t0
      }
    }
  }
}
