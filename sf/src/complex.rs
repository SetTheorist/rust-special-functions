use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use core::ops::{Shl,ShlAssign,Shr,ShrAssign};

use crate::real::{r64};
use crate::traits::{*};

#[derive(Debug,Default,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
pub struct c64{pub re:r64, pub im:r64}

impl std::fmt::Display for c64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.im < ι(0) {
          write!(f, "{}{}ι", self.re, self.im)
        } else {
          write!(f, "{}+{}ι", self.re, self.im)
        }
    }
}

impl From<r64> for c64 { #[inline] fn from(x:r64) -> c64 { c64{re:x, im:ι(0)} } }
impl From<f64> for c64 { #[inline] fn from(x:f64) -> c64 { c64{re:ι(x), im:ι(0)} } }
impl From<isize> for c64 { #[inline] fn from(x:isize) -> c64 { c64{re:ι(x), im:ι(0)} } }

impl Add<c64> for c64 {
  type Output = c64;
  #[inline]
  fn add(self, rhs:c64) -> c64 {
    let re = self.re + rhs.re;
    let im = self.im + rhs.im;
    c64 { re, im }
  }
}
impl Sub<c64> for c64 {
  type Output = c64;
  #[inline]
  fn sub(self, rhs:c64) -> c64 {
    let re = self.re - rhs.re;
    let im = self.im - rhs.im;
    c64 { re, im }
  }
}
impl Mul<c64> for c64 {
  type Output = c64;
  #[inline]
  fn mul(self, rhs:c64) -> c64 {
    let re = self.re*rhs.re - self.im*rhs.im;
    let im = self.re*rhs.im + self.im*rhs.re;
    c64 { re, im }
  }
}
impl Div<c64> for c64 {
  type Output = c64;
  #[inline]
  fn div(self, rhs:c64) -> c64 {
    let den = self.re*rhs.re + self.im*rhs.im;
    let re = (self.re*rhs.re + self.im*rhs.im) / den;
    let im = (self.re*rhs.im - self.im*rhs.re) / den;
    c64 { re, im }
  }
}

impl AddAssign<c64> for c64 {
  #[inline]
  fn add_assign(&mut self, rhs:c64) { *self = *self + rhs; }
}
impl SubAssign<c64> for c64 {
  #[inline]
  fn sub_assign(&mut self, rhs:c64) { *self = *self - rhs; }
}
impl MulAssign<c64> for c64 {
  #[inline]
  fn mul_assign(&mut self, rhs:c64) { *self = *self * rhs; }
}
impl DivAssign<c64> for c64 {
  #[inline]
  fn div_assign(&mut self, rhs:c64) { *self = *self / rhs; }
}

impl Add<r64> for c64 {
  type Output = c64;
  #[inline]
  fn add(self, rhs:r64) -> c64 {
    let re = self.re + rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Sub<r64> for c64 {
  type Output = c64;
  #[inline]
  fn sub(self, rhs:r64) -> c64 {
    let re = self.re - rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Mul<r64> for c64 {
  type Output = c64;
  #[inline]
  fn mul(self, rhs:r64) -> c64 {
    let re = self.re * rhs;
    let im = self.im * rhs;
    c64 { re, im }
  }
}
impl Div<r64> for c64 {
  type Output = c64;
  #[inline]
  fn div(self, rhs:r64) -> c64 {
    let re = self.re / rhs;
    let im = self.im / rhs;
    c64 { re, im }
  }
}

impl Add<f64> for c64 {
  type Output = c64;
  #[inline]
  fn add(self, rhs:f64) -> c64 {
    let re = self.re + rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Sub<f64> for c64 {
  type Output = c64;
  #[inline]
  fn sub(self, rhs:f64) -> c64 {
    let re = self.re - rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Mul<f64> for c64 {
  type Output = c64;
  #[inline]
  fn mul(self, rhs:f64) -> c64 {
    let re = self.re * rhs;
    let im = self.im * rhs;
    c64 { re, im }
  }
}
impl Div<f64> for c64 {
  type Output = c64;
  #[inline]
  fn div(self, rhs:f64) -> c64 {
    let re = self.re / rhs;
    let im = self.im / rhs;
    c64 { re, im }
  }
}

impl Add<isize> for c64 {
  type Output = c64;
  #[inline]
  fn add(self, rhs:isize) -> c64 {
    let re = self.re + rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Sub<isize> for c64 {
  type Output = c64;
  #[inline]
  fn sub(self, rhs:isize) -> c64 {
    let re = self.re - rhs;
    let im = self.im;
    c64 { re, im }
  }
}
impl Mul<isize> for c64 {
  type Output = c64;
  #[inline]
  fn mul(self, rhs:isize) -> c64 {
    let re = self.re * rhs;
    let im = self.im * rhs;
    c64 { re, im }
  }
}
impl Div<isize> for c64 {
  type Output = c64;
  #[inline]
  fn div(self, rhs:isize) -> c64 {
    let re = self.re / rhs;
    let im = self.im / rhs;
    c64 { re, im }
  }
}
