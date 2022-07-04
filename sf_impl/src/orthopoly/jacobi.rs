use crate::orthopoly::*;
use crate::poly::Poly;
use crate::traits::*;

use crate::gamma::{Gamma,sf_gamma};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Jacobi<V: Value> {
  pub alpha: V,
  pub beta: V,
}

impl<V:Value> Jacobi<V> {
  // \alpha,\beta > -1
  pub fn new(alpha:V, beta:V) -> Self {
    Jacobi { alpha, beta }
  }
}

impl<V:RealValue+Float+Power+Gamma> OrthogonalPolynomial<V> for Jacobi<V> {
  fn domain(&self) -> (V, V) {
    (ι(-1):V, ι(1):V)
  }
  fn scale(&self, n: isize) -> V {
    let α = self.alpha;
    let β = self.beta;
    sf_sqrt( (α+β+(2*n+1)) / (ι(2):V).pow(α+β+1)
      * sf_gamma(ι(n+1):V) / sf_gamma(α+(n+1))
      * sf_gamma(α+β+(n+1)) / sf_gamma(β+(n+1))
    )
  }
  fn value(&self, n: isize, x: V) -> V {
    let α = self.alpha;
    let β = self.beta;
    match n {
      0 => ι(1),
      1 => (α-β)/2 + x * (α+β+2)/2,
      _ => {
        let mut v1 = ι(1);
        let mut v0 = (α-β)/2 + x * (α+β+2)/2;
        for k in 2..(n+1) {
          let v2 = v1;
          v1 = v0;
          let ax = (α+β+(2*k-1))*(α.sqr() - β.sqr());
          let bx = (α+β+(2*k-2))*(α+β+(2*k-1))*(α+β+(2*k));
          let cx = (α+(k-1))*(β+(k-1))*(α+β+(2*k))*2;
          let dx = (α+β+k)*(2*k)*(α+β+(2*k-2));
          v0 = (v1*(ax+bx*x) - cx*v2)/dx;
        }
        v0
      }
    }
  }
  fn kernel(&self, x: V) -> V {
    let α = self.alpha;
    let β = self.beta;
    (-x+1).pow(α)*(x+1).pow(β)
  }
  fn zeros(&self, n: isize) -> Vec<V> {
    let α = self.alpha;
    let β = self.beta;
    match n {
      0 => vec![],
      1 => vec![(β-α)/(α+β+2)],
      _ => {
        let mut d : Vec<_> = (0..n).map(|k|
            if k==0 { (β-α)/(α+β+2) }
            else { (β.sqr()-α.sqr()) / ((α+β+(2*k))*(α+β+(2*k+2))) }
          ).collect();
        let mut e : Vec<_> = (0..n).map(|k|
            ι(2):V/(α+β+(2*k)) * sf_sqrt(((α+k)*(β+k)*(α+β+k)*k)/((α+β+(2*k+1))*(α+β+(2*k-1))))
          ).collect();
        crate::matrix::eig_symtrid(&mut d, &mut e);
        //d.sort(); // TODO: sort out traits later
        /*
        let dself = Jacobi::new(α+1, β-1);
        let pol = |z|{
          let fx = self.value(n, z);
          let dfx = (α+β+(n+1))/2 * dself.value(n-1, z);
          z - fx/dfx
        };
        // polish zeros
        for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        //for i in 0..n { d[i as usize] = pol(d[i as usize]); }
        */
        d
      }
    }
  }
  fn poly(&self, n: isize) -> Poly<V> {
    let α = self.alpha;
    let β = self.beta;
    match n {
      0 => Poly(vec![ι(1)]),
      1 => Poly(vec![(α-β)/2, (α+β+2)/2]),
      _ => {
        let mut v1 = Poly(vec![ι(1)]);
        let mut v0 = Poly(vec![(α-β)/2, (α+β+2)/2]);
        for k in 2..(n+1) {
          let v2 = v1;
          v1 = v0;
          let ax = (α+β+(2*k-1))*(α.sqr() - β.sqr());
          let bx = (α+β+(2*k-2))*(α+β+(2*k-1))*(α+β+(2*k));
          let cx = (α+(k-1))*(β+(k-1))*(α+β+(2*k))*2;
          let dx = (α+β+k)*(2*k)*(α+β+(2*k-2));
          v0 = (&v1*ax + &(v1.x(1)*bx) - &(v2*cx)) / dx;
        }
        v0
      }
    }
  }
}