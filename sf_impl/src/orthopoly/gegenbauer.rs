use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::gamma::*;
use crate::numbers::{sf_factorial, sf_pochhammer};

// TODO: note that accuracy here is affected by error in sf_gamma()

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gegenbauer<V:Value> {
  pub lambda: V,
}

impl<V:Value> Gegenbauer<V> {
  // \lambda > -1/2
  pub fn new(lambda:V) -> Self {
    Gegenbauer { lambda }
  }
}

impl<V:RealValue+Float+Power+Gamma> OrthogonalPolynomial<V> for Gegenbauer<V> {
  fn domain(&self) -> (V, V) {
    (ι(-1),ι(1))
  }

  fn scale(&self, n: isize) -> V {
    let λ = self.lambda;
    sf_pochhammer(λ+0.5, n) * sf_sqrt((λ+n)*2*sf_factorial::<V>(n as usize)*sf_gamma(λ*2+n))
      / (sf_pochhammer(λ*2, n)*(ι(2):V).pow(λ)*sf_gamma(λ+n+0.5))
  }

  fn value(&self, n: isize, x: V) -> V {
    let λ = self.lambda;
    match n {
      0 => ι(1),
      1 => x*λ*2,
      _ => {
        let mut z1 = ι(1);
        let mut z0 = x*λ*2;
        for k in 2..(n+1) {
          let z2 = z1;
          z1 = z0;
          z0 = (z1*(x*2*(λ+(k-1))) - z2*(λ*2+(k-2))) / k;
        }
        z0
      }
    }
  }

  fn kernel(&self, x: V) -> V {
    (-x.sqr() + 1).pow(self.lambda - 0.5)
  }

  fn zeros(&self, n: isize) -> Vec<V> {
    match n {
      0 => vec![],
      1 => vec![V::zero],
      _ => {
        let λ = self.lambda;
        let mut d = vec![V::zero; n as usize];
        let mut e : Vec<_> = (0..n).map(|k|
          match k {
            0 => V::zero,
            1 => sf_sqrt((λ*2+2).recip()),
            _ =>
              ι(2):V/(λ*2+(2*k-1))
                * sf_sqrt(((λ+k-0.5).sqr()*k*(λ*2+(k-1))) / ((λ*2+(2*k))*(λ*2+(2*k-2)))),
          }
        ).collect();
        crate::matrix::eig_symtrid(&mut d, &mut e);
        //d.sort(); // TODO: sort out traits later
        /*
        let dself = Gegenbauer::new(λ+1);
        let pol = |z|{
          let fx = self.value(n, z);
          let dfx = dself.value(n-1, z)*2;
          z - fx/dfx
        };
        log::warn!("{:.18?}", d.iter().map(|z|self.value(n,*z)).collect::<Vec<_>>());
        // polish zeros
        for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        log::warn!("{:.18?}", d.iter().map(|z|self.value(n,*z)).collect::<Vec<_>>());
        //for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        */
        d
      }
    }
  }

  fn poly(&self, n: isize) -> Poly<V> {
    let λ = self.lambda;
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![ι(0),ι(λ*2)]),
      _ => {
        let mut t1 = Poly(vec![ι(1)]);
        let mut t0 = Poly(vec![ι(0),ι(λ*2)]);
        for k in 2..(n+1) {
          let t2 = t1;
          t1 = t0;
          t0 = (t1.x(1)*((λ+(k-1))*2) - &(t2*(λ*2+(k-2)))) / ι(k):V;
        }
        t0
      }
    }
  }
}