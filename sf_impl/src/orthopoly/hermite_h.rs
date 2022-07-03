use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::exp::{Exp,sf_exp};
use crate::numbers::{sf_factorial};

empty_type!(HermiteH);

impl<V:RealValue+Float+Exp> OrthogonalPolynomial<V> for HermiteH<V> {
  fn domain(&self) -> (V, V) {
    (-V::infinity, V::infinity)
  }

  fn scale(&self, n: isize) -> V {
    sf_sqrt((V::SQRTPI<<n)*sf_factorial::<V>(n as usize)).recip()
  }

  fn value(&self, n: isize, x: V) -> V {
    match n {
      0 => ι(1),
      1 => x*2,
      _ => {
        let mut v1 = ι(1);
        let mut v0 = x*2;
        for k in 2..(n+1) {
          let v2 = v1;
          v1 = v0;
          v0 = v1*x*2 - v2*(2*(k-1));
        }
        v0
      }
    }
  }

  fn kernel(&self, x: V) -> V {
    sf_exp(-x*x)
  }

  fn zeros(&self, n: isize) -> Vec<V> {
    match n {
      0 => vec![],
      1 => vec![V::zero],
      _ => {
        let mut d = vec![V::zero; n as usize];
        let mut e : Vec<_> = (0..n).map(|k| sf_sqrt(ι(k):V/2)).collect();
        crate::matrix::eig_symtrid(&mut d, &mut e);
        //d.sort(); // TODO: sort out traits later
        let pol = |z|{
          let fx = self.value(n, z);
          let dfx = self.value(n-1, z)*(2*n);
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
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(0),ι(2)]),
      _ => {
        let mut v1 = Poly(vec![ι(1)]);
        let mut v0 = Poly(vec![ι(0),ι(2)]);
        for k in 2..(n+1) {
          let v2 = v1;
          v1 = v0;
          v0 = v1.x(1)*ι(2):V - &(v2*ι(2*(k-1)):V);
        }
        v0
      }
    }
  }
}