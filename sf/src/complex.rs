use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use core::ops::{Shl,ShlAssign,Shr,ShrAssign};

use crate::real::{r64};
use crate::traits::{*};

#[derive(Debug,Default,Clone,Copy,PartialOrd,PartialEq)]
#[allow(non_camel_case_types)]
pub struct c64{pub re:r64, pub im:r64}

impl c64 {
  const I : c64 = c64 { re:r64(0.0), im:r64(1.0) };

  #[inline]
  pub fn new(re:r64, im:r64) -> c64 { c64 { re, im } }
}

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
    if rhs.im == ι(0) {
      let re = self.re / rhs.re;
      let im = self.im / rhs.re;
      c64 { re, im }
    } else if rhs.re == ι(0) {
      let re = self.im / rhs.im;
      let im = -self.re / rhs.im;
      c64 { re, im }
    } else {
      // TODO: robustify for extreme cases
      let den = self.re*rhs.re + self.im*rhs.im;
      let re = (self.re*rhs.re + self.im*rhs.im) / den;
      let im = (self.re*rhs.im - self.im*rhs.re) / den;
      c64 { re, im }
    }
  }
}
// TODO: complex remainder
impl Rem<c64> for c64 {
  type Output = c64;
  #[inline]
  fn rem(self, rhs:c64) -> c64 {
    unimplemented!("c64::rem::<c64>({:?},{:?})", self, rhs)
  }
}

impl Neg for c64 {
  type Output = c64;
  #[inline]
  fn neg(self) -> c64 {
    let re = -self.re;
    let im = -self.im;
    c64 { re, im }
  }
}

// TODO: ldexp style implementations
impl Shl<isize> for c64 {
  type Output = c64;
  #[inline]
  fn shl(self, n:isize) -> c64 { self * (2.0_f64.powi(n as i32)) }
}
impl ShlAssign<isize> for c64 {
  #[inline]
  fn shl_assign(&mut self, n:isize) { *self *= 2.0_f64.powi(n as i32); }
}
impl Shr<isize> for c64 {
  type Output = c64;
  #[inline]
  fn shr(self, n:isize) -> c64 { self / (2.0_f64.powi(n as i32)) }
}
impl ShrAssign<isize> for c64 {
  #[inline]
  fn shr_assign(&mut self, n:isize) { *self /= 2.0_f64.powi(n as i32); }
}

macro_rules! scalar_impls {
  ($t:ty) => {
    impl Add<$t> for c64 {
      type Output = c64;
      #[inline]
      fn add(self, rhs:$t) -> c64 {
        let re = self.re + rhs;
        let im = self.im;
        c64 { re, im }
      }
    }
    impl Sub<$t> for c64 {
      type Output = c64;
      #[inline]
      fn sub(self, rhs:$t) -> c64 {
        let re = self.re - rhs;
        let im = self.im;
        c64 { re, im }
      }
    }
    impl Mul<$t> for c64 {
      type Output = c64;
      #[inline]
      fn mul(self, rhs:$t) -> c64 {
        let re = self.re * rhs;
        let im = self.im * rhs;
        c64 { re, im }
      }
    }
    impl Div<$t> for c64 {
      type Output = c64;
      #[inline]
      fn div(self, rhs:$t) -> c64 {
        let re = self.re / rhs;
        let im = self.im / rhs;
        c64 { re, im }
      }
    }
    impl Rem<$t> for c64 {
      type Output = c64;
      #[inline]
      fn rem(self, rhs:$t) -> c64 {
        unimplemented!("c64::Rem::<{}>({:?},{:?})", stringify!($t), self, rhs)
      }
    }
  }
}

macro_rules! assign_impls {
  ($t:ty) => {
    impl AddAssign<$t> for c64 {
      #[inline]
      fn add_assign(&mut self, rhs:$t) { *self = *self + rhs; }
    }
    impl SubAssign<$t> for c64 {
      #[inline]
      fn sub_assign(&mut self, rhs:$t) { *self = *self - rhs; }
    }
    impl MulAssign<$t> for c64 {
      #[inline]
      fn mul_assign(&mut self, rhs:$t) { *self = *self * rhs; }
    }
    impl DivAssign<$t> for c64 {
      #[inline]
      fn div_assign(&mut self, rhs:$t) { *self = *self / rhs; }
    }
    impl RemAssign<$t> for c64 {
      #[inline]
      fn rem_assign(&mut self, rhs:$t) {
        unimplemented!("c64::RemAssign::<{}>({:?},{:?})", stringify!($t), self, rhs)
      }
    }
  }
}


scalar_impls!(r64);
scalar_impls!(f64);
scalar_impls!(isize);

assign_impls!(c64);
assign_impls!(r64);
assign_impls!(f64);
assign_impls!(isize);

////////////////////////////////////////////////////////////////////////////////

impl Base for c64 { }
impl Zero for c64 { const zero : c64 = c64{re:r64(0.0),im:r64(0.0)}; }
impl Addition for c64 { }
impl Subtraction for c64 { }
impl Additive for c64 { }
impl One for c64 { const one : c64 = c64{re:r64(1.0),im:r64(0.0)}; }
impl Multiplication for c64 { }
impl Division for c64 { }
impl Multiplicative for c64 { }
impl Embeds<isize> for c64 { }
impl Embeds<f64> for c64 { }
impl Embeds<r64> for c64 { }
impl Field for c64 { }

impl Roots for c64 {
  #[inline]
  fn sqrt(self) -> Self {
    let (r,a) = self.to_polar();
    c64::polar(r.sqrt(), a/2)
  }
  #[inline]
  fn cbrt(self) -> Self {
    let (r,a) = self.to_polar();
    c64::polar(r.cbrt(), a/3)
  }
  fn nth_root(self, n:isize) -> Self {
    let (r,a) = self.to_polar();
    c64::polar(r.nth_root(n), a/n)
  }
}


impl Normed for c64 {
  type NT = r64;
  const epsilon : Self::NT = r64(f64::EPSILON);
  #[inline]
  fn abs(self) -> Self::NT {
    // TODO: robustify
    (self.re*self.re + self.im*self.im).sqrt()
  }
  #[inline]
  fn fabs(self) -> f64 { self.abs().0 }
  #[inline]
  fn signum(self) -> Self { self / self.abs() }
  #[inline]
  fn mu(self) -> Self::NT {
    self.re*self.re + self.im*self.im
  }
  const mu_epsilon : Self::NT = r64(f64::EPSILON * f64::EPSILON);
}

impl ComplexType for c64 {
  type RT = r64;
  #[inline]
  fn real(self) -> Self::RT {
    self.re
  }
  #[inline]
  fn imag(self) -> Self::RT {
    self.im
  }
  #[inline]
  fn arg(self) -> Self::RT {
    r64(self.im.0.atan2(self.re.0))
  }
  #[inline]
  fn conj(self) -> Self {
    c64 { re:self.re, im:-self.im }
  }
  #[inline]
  fn rect(re:Self::RT,im:Self::RT) -> Self {
    c64 { re, im }
  }
  #[inline]
  fn polar(r:Self::RT,arg:Self::RT) -> Self {
    // TODO: use crate cos/sin ...
    let re = r * arg.0.cos();
    let im = r * arg.0.sin();
    c64 { re, im }
  }
}

impl Value for c64 { }

