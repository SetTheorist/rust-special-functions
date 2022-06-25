use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use super::*;

impl Add<c64> for c64 {
  type Output = c64;
  #[inline]
  fn add(self, rhs: c64) -> c64 {
    let re = self.re + rhs.re;
    let im = self.im + rhs.im;
    c64 { re, im }
  }
}
impl Sub<c64> for c64 {
  type Output = c64;
  #[inline]
  fn sub(self, rhs: c64) -> c64 {
    let re = self.re - rhs.re;
    let im = self.im - rhs.im;
    c64 { re, im }
  }
}
impl Mul<c64> for c64 {
  type Output = c64;
  #[inline]
  fn mul(self, rhs: c64) -> c64 {
    let re = self.re * rhs.re - self.im * rhs.im;
    let im = self.re * rhs.im + self.im * rhs.re;
    c64 { re, im }
  }
}
impl Div<c64> for c64 {
  type Output = c64;
  #[inline]
  fn div(self, rhs: c64) -> c64 {
    if rhs.im == 0 {
      let re = self.re / rhs.re;
      let im = self.im / rhs.re;
      c64 { re, im }
    } else if rhs.re == 0 {
      let re = self.im / rhs.im;
      let im = -self.re / rhs.im;
      c64 { re, im }
    } else {
      // TODO: robustify for extreme cases
      let den = rhs.re * rhs.re + rhs.im * rhs.im;
      let re = (self.re * rhs.re + self.im * rhs.im) / den;
      let im = (self.im * rhs.re - self.re * rhs.im) / den;
      c64 { re, im }
    }
  }
}

impl Rem<c64> for c64 {
  type Output = c64;
  #[inline]
  fn rem(self, rhs: c64) -> c64 {
    let n = (self.re*rhs.re + self.im*rhs.im) / (rhs.re.sqr() + rhs.im.sqr());
    self - rhs*n.floor()
  }
}

impl Rem<(c64,c64)> for c64 {
  type Output = c64;
  #[inline]
  fn rem(self, (a,b):(c64,c64)) -> c64 {
    let a = a%b;
    let b = b%a;
    (self%a)%b
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
  fn shl(self, n: isize) -> c64 { self * (2.0_f64.powi(n as i32)) }
}
impl ShlAssign<isize> for c64 {
  #[inline]
  fn shl_assign(&mut self, n: isize) { *self *= 2.0_f64.powi(n as i32); }
}
impl Shr<isize> for c64 {
  type Output = c64;
  #[inline]
  fn shr(self, n: isize) -> c64 { self / (2.0_f64.powi(n as i32)) }
}
impl ShrAssign<isize> for c64 {
  #[inline]
  fn shr_assign(&mut self, n: isize) { *self /= 2.0_f64.powi(n as i32); }
}

macro_rules! scalar_impls {
  ($t:ty) => {
    impl const Add<$t> for c64 {
      type Output = c64;
      #[inline]
      fn add(self, rhs: $t) -> c64 {
        let re = self.re + rhs;
        let im = self.im;
        c64 { re, im }
      }
    }
    impl const Add<c64> for $t {
      type Output = c64;
      #[inline]
      fn add(self, rhs: c64) -> c64 {
        let re = self + rhs.re;
        let im = rhs.im;
        c64 { re, im }
      }
    }
    impl const Sub<$t> for c64 {
      type Output = c64;
      #[inline]
      fn sub(self, rhs: $t) -> c64 {
        let re = self.re - rhs;
        let im = self.im;
        c64 { re, im }
      }
    }
    impl const Sub<c64> for $t {
      type Output = c64;
      #[inline]
      fn sub(self, rhs: c64) -> c64 {
        let re = self - rhs.re;
        let im = -rhs.im;
        c64 { re, im }
      }
    }
    impl const Mul<$t> for c64 {
      type Output = c64;
      #[inline]
      fn mul(self, rhs: $t) -> c64 {
        let re = self.re * rhs;
        let im = self.im * rhs;
        c64 { re, im }
      }
    }
    impl const Mul<c64> for $t {
      type Output = c64;
      #[inline]
      fn mul(self, rhs: c64) -> c64 {
        let re = self * rhs.re;
        let im = self * rhs.im;
        c64 { re, im }
      }
    }
    impl const Div<$t> for c64 {
      type Output = c64;
      #[inline]
      fn div(self, rhs: $t) -> c64 {
        let re = self.re / rhs;
        let im = self.im / rhs;
        c64 { re, im }
      }
    }
    impl Div<c64> for $t {
      type Output = c64;
      #[inline]
      fn div(self, rhs: c64) -> c64 {
        c64{re:ι(self), im:ι(0)} / rhs
      }
    }
    impl const Rem<$t> for c64 {
      type Output = c64;
      #[inline]
      fn rem(self, rhs: $t) -> c64 {
        let re = self.re % rhs;
        let im = self.im;
        c64 { re, im }
      }
    }
    impl Rem<c64> for $t {
      type Output = c64;
      #[inline]
      fn rem(self, rhs: c64) -> c64 {
        c64{re:ι(self), im:ι(0)} % rhs
      }
    }
  };
}

macro_rules! assign_impls {
  ($t:ty) => {
    impl AddAssign<$t> for c64 {
      #[inline]
      fn add_assign(&mut self, rhs: $t) { *self = *self + rhs; }
    }
    impl SubAssign<$t> for c64 {
      #[inline]
      fn sub_assign(&mut self, rhs: $t) { *self = *self - rhs; }
    }
    impl MulAssign<$t> for c64 {
      #[inline]
      fn mul_assign(&mut self, rhs: $t) { *self = *self * rhs; }
    }
    impl DivAssign<$t> for c64 {
      #[inline]
      fn div_assign(&mut self, rhs: $t) { *self = *self / rhs; }
    }
    impl RemAssign<$t> for c64 {
      #[inline]
      fn rem_assign(&mut self, rhs: $t) { *self = *self % rhs; }
    }
  };
}

scalar_impls!(r64);
scalar_impls!(f64);
scalar_impls!(isize);

assign_impls!(c64);
assign_impls!(r64);
assign_impls!(f64);
assign_impls!(isize);

impl PartialEq<isize> for c64 {
  fn eq(&self, rhs: &isize) -> bool { self.eq(&(ι(*rhs): c64)) }
}

impl PartialEq<f64> for c64 {
  fn eq(&self, rhs: &f64) -> bool { self.eq(&(ι(*rhs): c64)) }
}

impl PartialEq<r64> for c64 {
  fn eq(&self, rhs: &r64) -> bool { self.eq(&(ι(*rhs): c64)) }
}
