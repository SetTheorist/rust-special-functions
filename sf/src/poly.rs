use std::ops::{Add,Sub,Mul,Div,Neg};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};

use crate::traits::{*};

#[derive(Clone,Debug)]
pub struct Poly<T>(pub Vec<T>);

impl<T:Default> Default for Poly<T> {
  fn default() -> Self {
    Poly(vec![T::default()])
  }
}

impl<T> std::fmt::Display for Poly<T>
  where T:std::fmt::Display+Zero+PartialEq
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut i = 0;
    for c in &self.0 {
      if i == 0 {
        write!(f, "{}", c)?;
      } else {
        if c != &T::zero {
          write!(f, "+{}*x^{}", c, i)?;
        }
      }
      i += 1;
    }
    write!(f, "")
  }
}

impl<T:Zero+PartialEq> Poly<T> {
  pub fn reduce(&mut self) {
    let mut n = self.0.len()-1;
    while (n > 0) && (self.0[n] == T::zero) {
      n -= 1;
    }
    self.0.resize(n+1, T::zero);
  }
}

impl<T:Addition> AddAssign<&Poly<T>> for Poly<T> {
  fn add_assign(&mut self, rhs:&Poly<T>) {
    let ll = self.0.len();
    let lr = rhs.0.len();
    self.0.resize(ll.max(lr), T::zero);
    for i in 0..ll.min(lr) {
      self.0[i] += rhs.0[i];
    }
    self.reduce();
  }
}

impl<T:Subtraction> SubAssign<&Poly<T>> for Poly<T> {
  fn sub_assign(&mut self, rhs:&Poly<T>) {
    let ll = self.0.len();
    let lr = rhs.0.len();
    self.0.resize(ll.max(lr), T::zero);
    for i in 0..ll.min(lr) {
      self.0[i] -= rhs.0[i];
    }
    self.reduce();
  }
}

impl<T:Addition+Multiplication> Mul<&Poly<T>> for &Poly<T> {
  type Output = Poly<T>;
  fn mul(self, rhs:&Poly<T>) -> Poly<T> {
    let ll = self.0.len();
    let lr = rhs.0.len();
    let mut res : Vec<T> = vec![T::zero; ll+lr];
    for i in 0..ll {
      for j in 0..lr {
        res[i+j] += self.0[i] * rhs.0[j];
      }
    }
    let mut res = Poly(res);
    res.reduce();
    res
  }
}

// division requires remainder...

////////////////////////////////////////////////////////////////////////////////
// scalars

impl<T:Addition> AddAssign<T> for Poly<T> {
  fn add_assign(&mut self, rhs:T) {
    self.0[0] += rhs;
  }
}

impl<T:Subtraction> SubAssign<T> for Poly<T> {
  fn sub_assign(&mut self, rhs:T) {
    self.0[0] -= rhs;
  }
}

impl<T:Multiplication> MulAssign<T> for Poly<T> {
  fn mul_assign(&mut self, rhs:T) {
    for c in self.0.iter_mut() {
      *c *= rhs;
    }
  }
}

impl<T:Division> DivAssign<T> for Poly<T> {
  fn div_assign(&mut self, rhs:T) {
    for c in self.0.iter_mut() {
      *c /= rhs;
    }
  }
}
