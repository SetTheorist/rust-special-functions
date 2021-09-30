use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)] // Eq,Ord
pub struct Wide(f64, f64);

impl From<f64> for Wide {
  fn from(x: f64) -> Wide { Wide(x, 0.0) }
}
impl From<isize> for Wide {
  fn from(x: isize) -> Wide { Wide(x as f64, 0.0) }
}

// requires |a|>=|b|
#[inline]
fn qtsum(a: f64, b: f64) -> (f64, f64) {
  let s = a + b;
  let e = b + (a - s); // = b-(s-a)
  (s, e)
}

// general
#[inline]
fn ddsum(a: f64, b: f64) -> (f64, f64) {
  let s = a + b;
  let v = s - a;
  let e = (a + (v - s)) + (b - v); // = (a-(s-v))+(b-v)
  (s, e)
}

#[inline]
fn split(a: f64) -> (f64, f64) {
  let t = 134217729.0 * a;
  let ahi = t - (t - a);
  let alo = a - ahi;
  (ahi, alo)
}

#[inline]
fn ddprod(a: f64, b: f64) -> (f64, f64) {
  let (ahi, alo) = split(a);
  let (bhi, blo) = split(b);
  let p = a * b;
  let e = (((ahi * bhi - p) + ahi * blo) + alo * bhi) + alo * blo;
  (p, e)
}

#[inline]
fn qdadd(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let (shi, slo) = ddsum(y, xhi);
  let (hhi, hlo) = qtsum(shi, slo + xlo);
  let (hi, lo) = qtsum(hhi, hlo);
  Wide(hi, lo)
}

#[inline]
fn dqadd(x: f64, y: Wide) -> Wide { qdadd(y, x) }

#[inline]
fn qqadd(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let (hs, he) = ddsum(xhi, yhi);
  let (ls, le) = ddsum(xlo, ylo);
  let (h, k) = qtsum(hs, he + ls);
  let (hi, lo) = qtsum(h, le + k);
  Wide(hi, lo)
}

#[inline]
fn qnegate(Wide(hi, lo): Wide) -> Wide { Wide(-hi, -lo) }

#[inline]
fn qdprod(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let (thi, tlo) = ddprod(xhi, y);
  let (hi, lo) = qtsum(thi, tlo + y * xlo);
  Wide(hi, lo)
}

#[inline]
fn dqprod(x: f64, y: Wide) -> Wide { qdprod(y, x) }

#[inline]
fn qqprod(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let (p, e) = ddprod(xhi, yhi);
  let (hi, lo) = qtsum(p, e + (xhi * ylo + xlo * yhi));
  Wide(hi, lo)
}

#[inline]
fn qqdivide(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let cc = xhi / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = ((((xhi - uu) - u) + xlo) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Wide(hi, lo)
}

#[inline]
fn dqdivide(x: f64, Wide(yhi, ylo): Wide) -> Wide {
  let cc = x / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = (((x - uu) - u) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Wide(hi, lo)
}

#[inline]
fn qddivide(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let xdy = xhi / y;
  let (uu, u) = ddprod(xdy, y);
  let c = (((xhi - uu) - u) + xlo) / y;
  let (hi, lo) = qtsum(xdy, c);
  Wide(hi, lo)
}

impl Wide {
  // construction
  #[inline]
  pub fn new(a: f64, b: f64) -> Wide {
    let (hi, lo) = ddsum(a, b);
    Wide(hi, lo)
  }

  // deconstruction
  #[inline]
  pub fn parts(Wide(hi, lo): Self) -> (f64, f64) { (hi, lo) }
  #[inline]
  pub fn hi(self) -> f64 { self.0 }
  #[inline]
  pub fn lo(self) -> f64 { self.1 }
  
  // apply "correct" rounding to high part...
  // TODO: pub fn to_f64(self) -> f64 {}

  // misc

  #[inline]
  pub fn abs(self) -> Wide {
    if self.0 < 0.0 {
      -self
    } else {
      self
    }
  }

  #[inline]
  pub fn scale2(self, i: isize) -> Wide {
    // TODO: replace with ldexp() functionality
    Wide(self.0 * 2.0_f64.powi(i as i32), self.1 * 2.0_f64.powi(i as i32))
  }

  pub fn scale10(self, i: isize) -> Wide {
    if i < 0 {
      let mut q = self;
      for _ in 0..(-i) { q /= 10.0; }
      q
    } else if i > 0 {
      let mut q = self;
      for _ in 0..i { q *= 10.0; }
      q
    } else {
      self
    }
  }
}

impl Add<Wide> for Wide {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { qqadd(self, y) }
}
impl Sub<Wide> for Wide {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { qqadd(self, -y) }
}
impl Mul<Wide> for Wide {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { qqprod(self, y) }
}
impl Div<Wide> for Wide {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { qqdivide(self, y) }
}
impl Rem<Wide> for Wide {
  type Output = Wide;
  fn rem(self, y: Wide) -> Wide { unimplemented!() }
}
impl Neg for Wide {
  type Output = Wide;
  fn neg(self) -> Wide { qnegate(self) }
}

impl Add<f64> for Wide {
  type Output = Wide;
  fn add(self, y: f64) -> Wide { qdadd(self, y) }
}
impl Sub<f64> for Wide {
  type Output = Wide;
  fn sub(self, y: f64) -> Wide { qdadd(self, -y) }
}
impl Mul<f64> for Wide {
  type Output = Wide;
  fn mul(self, y: f64) -> Wide { qdprod(self, y) }
}
impl Div<f64> for Wide {
  type Output = Wide;
  fn div(self, y: f64) -> Wide { qddivide(self, y) }
}
impl Rem<f64> for Wide {
  type Output = Wide;
  fn rem(self, y: f64) -> Wide { unimplemented!() }
}

impl Add<isize> for Wide {
  type Output = Wide;
  fn add(self, y: isize) -> Wide { qdadd(self, y as f64) }
}
impl Sub<isize> for Wide {
  type Output = Wide;
  fn sub(self, y: isize) -> Wide { qdadd(self, -y as f64) }
}
impl Mul<isize> for Wide {
  type Output = Wide;
  fn mul(self, y: isize) -> Wide { qdprod(self, y as f64) }
}
impl Div<isize> for Wide {
  type Output = Wide;
  fn div(self, y: isize) -> Wide { qddivide(self, y as f64) }
}
impl Rem<isize> for Wide {
  type Output = Wide;
  fn rem(self, y: isize) -> Wide { unimplemented!() }
}

impl Add<Wide> for f64 {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { dqadd(self, y) }
}
impl Sub<Wide> for f64 {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { dqadd(self, -y) }
}
impl Mul<Wide> for f64 {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { dqprod(self, y) }
}
impl Div<Wide> for f64 {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { dqdivide(self, y) }
}

impl Add<Wide> for isize {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { dqadd(self as f64, y) }
}
impl Sub<Wide> for isize {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { dqadd(self as f64, -y) }
}
impl Mul<Wide> for isize {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { dqprod(self as f64, y) }
}
impl Div<Wide> for isize {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { dqdivide(self as f64, y) }
}

impl AddAssign<Wide> for Wide {
  fn add_assign(&mut self, y: Wide) { *self = qqadd(*self, y); }
}
impl SubAssign<Wide> for Wide {
  fn sub_assign(&mut self, y: Wide) { *self = qqadd(*self, -y); }
}
impl MulAssign<Wide> for Wide {
  fn mul_assign(&mut self, y: Wide) { *self = qqprod(*self, y); }
}
impl DivAssign<Wide> for Wide {
  fn div_assign(&mut self, y: Wide) { *self = qqdivide(*self, y); }
}
impl RemAssign<Wide> for Wide {
  fn rem_assign(&mut self, y: Wide) { unimplemented!() }
}

impl AddAssign<f64> for Wide {
  fn add_assign(&mut self, y: f64) { *self = qdadd(*self, y); }
}
impl SubAssign<f64> for Wide {
  fn sub_assign(&mut self, y: f64) { *self = qdadd(*self, -y); }
}
impl MulAssign<f64> for Wide {
  fn mul_assign(&mut self, y: f64) { *self = qdprod(*self, y); }
}
impl DivAssign<f64> for Wide {
  fn div_assign(&mut self, y: f64) { *self = qddivide(*self, y); }
}
impl RemAssign<f64> for Wide {
  fn rem_assign(&mut self, y: f64) { unimplemented!() }
}

impl AddAssign<isize> for Wide {
  fn add_assign(&mut self, y: isize) { *self = qdadd(*self, y as f64); }
}
impl SubAssign<isize> for Wide {
  fn sub_assign(&mut self, y: isize) { *self = qdadd(*self, -y as f64); }
}
impl MulAssign<isize> for Wide {
  fn mul_assign(&mut self, y: isize) { *self = qdprod(*self, y as f64); }
}
impl DivAssign<isize> for Wide {
  fn div_assign(&mut self, y: isize) { *self = qddivide(*self, y as f64); }
}
impl RemAssign<isize> for Wide {
  fn rem_assign(&mut self, y: isize) { unimplemented!() }
}

impl Shl<isize> for Wide {
  type Output = Wide;
  fn shl(self, n: isize) -> Wide { self.scale2(n) }
}
impl Shr<isize> for Wide {
  type Output = Wide;
  fn shr(self, n: isize) -> Wide { self.scale2(n) }
}
impl ShlAssign<isize> for Wide {
  fn shl_assign(&mut self, n: isize) { *self = *self << n; }
}
impl ShrAssign<isize> for Wide {
  fn shr_assign(&mut self, n: isize) { *self = *self >> n; }
}

impl PartialEq<f64> for Wide {
  fn eq(&self, n: &f64) -> bool { *self == Wide(*n, 0.0) }
}

impl PartialEq<isize> for Wide {
  fn eq(&self, n: &isize) -> bool { *self == Wide(*n as f64, 0.0) }
}

impl std::str::FromStr for Wide {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, ()> {
    if s.is_empty() {
      return Err(());
    }
    let mut neg = false;
    let mut dec = false;
    let mut e = 0;
    let mut q = Wide(0.0, 0.0);
    for c in s.chars() {
      match c {
        '-' => { neg = true; }
        '+' => {}
        '.' => { dec = true; }
        //'e' => { }
        d => {
          let v = ((d as u8) - b'0') as f64;
          if !(0.0<=v && v<=9.0) {
            return Err(());
          }
          q = q * 10.0 + v;
          if dec { e -= 1; }
        }
      }
    }
    q = q.scale10(e);
    Ok(if neg { -q } else { q })
  }
}

impl std::fmt::Display for Wide {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // local simple flooring function
    // assumes q >= 0.0
    fn floor0(q:&Wide) -> f64 {
      let q0f = q.0.floor();
      if -q.1 > (q.0 - q0f) {
        q0f - 1.0
      } else {
        q0f
      }
    }

    write!(f, "ξ")?;
    if self.0 == 0.0 {
      return write!(f, "0.0");
    }
    let mut q = *self;
    if q.0 < 0.0 {
      q = -q;
      write!(f, "-")?;
    }

    let mut e = 0;
    while q >= Wide(10.0, 0.0) {
      e += 1;
      q /= 10.0;
    }
    while q < Wide(1.0, 0.0) {
      e -= 1;
      q *= 10.0;
    }

    for n in 0..33 {
      if n == 1 {
        write!(f, ".")?;
      }
      let d = floor0(&q);
      let dd = ((d as u8) + b'0') as char;
      write!(f, "{}", dd)?;
      q = (q - d) * 10.0;
    }

    if e != 0 {
      write!(f, "e{}", e)?;
    }
    write!(f, "")
  }
}

use crate::traits::*;
impl Base for Wide {}
impl Zero for Wide {
  const zero: Wide = Wide(0.0, 0.0);
}
impl Addition for Wide {}
impl Subtraction for Wide {}
impl Additive for Wide {}
impl One for Wide {
  const one: Wide = Wide(1.0, 0.0);
}
impl Multiplication for Wide {}
impl Division for Wide {}
impl Multiplicative for Wide {}
impl Embeds<isize> for Wide {}
impl Embeds<f64> for Wide {}
impl Field for Wide {}

/*
-- basic implementation: range-reduce & series
qexp q
  | q<0 = 1/(qexp (-q))
  | otherwise =
    let !nd = q / ln2_q
        !nn = floor.hi_ $ nd
        !r = q - ln2_q * (fromIntegral nn)
        !s = sm r 0.0 1.0 1
    in Wide (scaleFloat nn $ hi_ s) (scaleFloat nn $ lo_ s)
  where sm !r !s !t !n
          | s+t==s = s
          | otherwise = sm r (s+t) (t*r/(fromIntegral n)) (n+1)

*/
