use std::ops::{Add,Sub,Mul,Div,Rem,Neg};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use std::ops::{Shl,ShlAssign,Shr,ShrAssign};

use crate::traits::*;


#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct Dual<V> {
  pub x: V,
  pub dx: V,
}

// TODO: alternative idea, allowing nth-derivatives (?)
pub struct DualN<V,const N:usize> {
  pub x: V,
  pub d: [V;N],
}

impl<V:std::fmt::Display> std::fmt::Display for Dual<V> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.x, f)?;
    write!(f, "Δ(")?;
    std::fmt::Display::fmt(&self.dx, f)?;
    write!(f, ")")
  }
}


impl<V:Default> From<V> for Dual<V> {
  fn from(x:V) -> Self {
    let dx = V::default();
    Dual { x, dx }
  }
}

impl<V:Neg<Output=V>> Neg for Dual<V> {
  type Output = Self;
  fn neg(self) -> Self {
    Dual { x:-self.x, dx:-self.dx }
  }
}
impl<V:Additive> Add<Dual<V>> for Dual<V> {
  type Output = Self;
  #[inline]
  fn add(self, rhs:Self) -> Self {
    let x = self.x + rhs.x;
    let dx = self.dx + rhs.dx;
    Dual { x, dx }
  }
}
impl<V:Additive> AddAssign<Dual<V>> for Dual<V> {
  #[inline]
  fn add_assign(&mut self, rhs:Self) {
    *self = *self + rhs;
  }
}
impl<V:Additive> Sub<Dual<V>> for Dual<V> {
  type Output = Self;
  #[inline]
  fn sub(self, rhs:Self) -> Self {
    let x = self.x - rhs.x;
    let dx = self.dx - rhs.dx;
    Dual { x, dx }
  }
}
impl<V:Additive> SubAssign<Dual<V>> for Dual<V> {
  #[inline]
  fn sub_assign(&mut self, rhs:Self) {
    *self = *self - rhs;
  }
}
impl<V:Ring> Mul<Dual<V>> for Dual<V> {
  type Output = Self;
  #[inline]
  fn mul(self, rhs:Self) -> Self {
    let x = self.x * rhs.x;
    let dx = self.x*rhs.dx + self.dx*rhs.x;
    Dual { x, dx }
  }
}
impl<V:Ring> MulAssign<Dual<V>> for Dual<V> {
  #[inline]
  fn mul_assign(&mut self, rhs:Self) {
    *self = *self * rhs;
  }
}
impl<V:DivisionRing> Div<Dual<V>> for Dual<V> {
  type Output = Self;
  #[inline]
  fn div(self, rhs:Self) -> Self {
    let den = rhs.x.sqr();
    let x = (self.x * rhs.dx) / den;
    let dx = (self.dx*rhs.x - self.x*rhs.dx) / den;
    Dual { x, dx }
  }
}
impl<V:DivisionRing> DivAssign<Dual<V>> for Dual<V> {
  #[inline]
  fn div_assign(&mut self, rhs:Self) {
    *self = *self / rhs;
  }
}
impl<V:DivisionRing> Rem<Dual<V>> for Dual<V> {
  type Output = Self;
  #[inline]
  fn rem(self, rhs:Self) -> Self {
    unimplemented!("Dual::rem()")
  }
}
impl<V:DivisionRing> RemAssign<Dual<V>> for Dual<V> {
  #[inline]
  fn rem_assign(&mut self, rhs:Self) {
    *self = *self / rhs;
  }
}

impl<V:Shl<isize,Output=V>> Shl<isize> for Dual<V> {
  type Output=Self;
  fn shl(self, n:isize) -> Self {
    Dual { x:self.x.shl(n), dx:self.dx.shl(n) }
  }
}
impl<V:ShlAssign<isize>> ShlAssign<isize> for Dual<V> {
  fn shl_assign(&mut self, n:isize) {
    self.x.shl_assign(n);
    self.dx.shl_assign(n);
  }
}
impl<V:Shr<isize,Output=V>> Shr<isize> for Dual<V> {
  type Output=Self;
  fn shr(self, n:isize) -> Self {
    Dual { x:self.x.shr(n), dx:self.dx.shr(n) }
  }
}
impl<V:ShrAssign<isize>> ShrAssign<isize> for Dual<V> {
  fn shr_assign(&mut self, n:isize) {
    self.x.shr_assign(n);
    self.dx.shr_assign(n);
  }
}

impl<V:Base> Base for Dual<V> {}
impl<V:Zero> Zero for Dual<V> { const zero: Dual<V> = Dual{x:V::zero, dx:V::zero}; }
impl<V:Additive> Addition for Dual<V> {}
impl<V:Additive> Subtraction for Dual<V> {}
impl<V:Additive> Additive for Dual<V> {}
impl<V:One+Zero> One for Dual<V> { const one: Dual<V> = Dual{x:V::one, dx:V::zero}; }
impl<V:Ring> Multiplication for Dual<V> {}
impl<V:DivisionRing> Division for Dual<V> {}
impl<V:DivisionRing> Multiplicative for Dual<V> {}

