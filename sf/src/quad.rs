use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)] // Eq,Ord
pub struct Quad(f64, f64);

impl From<f64> for Quad {
  fn from(x: f64) -> Quad { Quad(x, 0.0) }
}
impl From<isize> for Quad {
  fn from(x: isize) -> Quad { Quad(x as f64, 0.0) }
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
fn qdadd(Quad(xhi, xlo): Quad, y: f64) -> Quad {
  let (shi, slo) = ddsum(y, xhi);
  let (hhi, hlo) = qtsum(shi, slo + xlo);
  let (hi, lo) = qtsum(hhi, hlo);
  Quad(hi, lo)
}

#[inline]
fn dqadd(x: f64, y: Quad) -> Quad { qdadd(y, x) }

#[inline]
fn qqadd(Quad(xhi, xlo): Quad, Quad(yhi, ylo): Quad) -> Quad {
  let (hs, he) = ddsum(xhi, yhi);
  let (ls, le) = ddsum(xlo, ylo);
  let (h, k) = qtsum(hs, he + ls);
  let (hi, lo) = qtsum(h, le + k);
  Quad(hi, lo)
}

#[inline]
fn qnegate(Quad(hi, lo): Quad) -> Quad { Quad(-hi, -lo) }

#[inline]
fn qdprod(Quad(xhi, xlo): Quad, y: f64) -> Quad {
  let (thi, tlo) = ddprod(xhi, y);
  let (hi, lo) = qtsum(thi, tlo + y * xlo);
  Quad(hi, lo)
}

#[inline]
fn dqprod(x: f64, y: Quad) -> Quad { qdprod(y, x) }

#[inline]
fn qqprod(Quad(xhi, xlo): Quad, Quad(yhi, ylo): Quad) -> Quad {
  let (p, e) = ddprod(xhi, yhi);
  let (hi, lo) = qtsum(p, e + (xhi * ylo + xlo * yhi));
  Quad(hi, lo)
}

#[inline]
fn qqdivide(Quad(xhi, xlo): Quad, Quad(yhi, ylo): Quad) -> Quad {
  let cc = xhi / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = ((((xhi - uu) - u) + xlo) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Quad(hi, lo)
}

#[inline]
fn dqdivide(x: f64, Quad(yhi, ylo): Quad) -> Quad {
  let cc = x / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = (((x - uu) - u) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Quad(hi, lo)
}

#[inline]
fn qddivide(Quad(xhi, xlo): Quad, y: f64) -> Quad {
  let xdy = xhi / y;
  let (uu, u) = ddprod(xdy, y);
  let c = (((xhi - uu) - u) + xlo) / y;
  let (hi, lo) = qtsum(xdy, c);
  Quad(hi, lo)
}

impl Quad {
  // construction
  #[inline]
  pub fn new(a: f64, b: f64) -> Quad {
    let (hi, lo) = ddsum(a, b);
    Quad(hi, lo)
  }

  // deconstruction
  #[inline]
  pub fn parts(Quad(hi, lo): Self) -> (f64, f64) { (hi, lo) }
  #[inline]
  pub fn hi(self) -> f64 { self.0 }
  #[inline]
  pub fn lo(self) -> f64 { self.1 }

  // misc

  #[inline]
  pub fn abs(self) -> Quad {
    if self.0 < 0.0 {
      -self
    } else {
      self
    }
  }

  #[inline]
  pub fn scale2(self, i: isize) -> Quad {
    // TODO: replace with ldexp() functionality
    Quad(self.0 * 2.0_f64.powi(i as i32), self.1 * 2.0_f64.powi(i as i32))
  }

  pub fn scale10(self, i: isize) -> Quad {
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

impl Add<Quad> for Quad {
  type Output = Quad;
  fn add(self, y: Quad) -> Quad { qqadd(self, y) }
}
impl Sub<Quad> for Quad {
  type Output = Quad;
  fn sub(self, y: Quad) -> Quad { qqadd(self, -y) }
}
impl Mul<Quad> for Quad {
  type Output = Quad;
  fn mul(self, y: Quad) -> Quad { qqprod(self, y) }
}
impl Div<Quad> for Quad {
  type Output = Quad;
  fn div(self, y: Quad) -> Quad { qqdivide(self, y) }
}
impl Rem<Quad> for Quad {
  type Output = Quad;
  fn rem(self, y: Quad) -> Quad { unimplemented!() }
}
impl Neg for Quad {
  type Output = Quad;
  fn neg(self) -> Quad { qnegate(self) }
}

impl Add<f64> for Quad {
  type Output = Quad;
  fn add(self, y: f64) -> Quad { qdadd(self, y) }
}
impl Sub<f64> for Quad {
  type Output = Quad;
  fn sub(self, y: f64) -> Quad { qdadd(self, -y) }
}
impl Mul<f64> for Quad {
  type Output = Quad;
  fn mul(self, y: f64) -> Quad { qdprod(self, y) }
}
impl Div<f64> for Quad {
  type Output = Quad;
  fn div(self, y: f64) -> Quad { qddivide(self, y) }
}
impl Rem<f64> for Quad {
  type Output = Quad;
  fn rem(self, y: f64) -> Quad { unimplemented!() }
}

impl Add<isize> for Quad {
  type Output = Quad;
  fn add(self, y: isize) -> Quad { qdadd(self, y as f64) }
}
impl Sub<isize> for Quad {
  type Output = Quad;
  fn sub(self, y: isize) -> Quad { qdadd(self, -y as f64) }
}
impl Mul<isize> for Quad {
  type Output = Quad;
  fn mul(self, y: isize) -> Quad { qdprod(self, y as f64) }
}
impl Div<isize> for Quad {
  type Output = Quad;
  fn div(self, y: isize) -> Quad { qddivide(self, y as f64) }
}
impl Rem<isize> for Quad {
  type Output = Quad;
  fn rem(self, y: isize) -> Quad { unimplemented!() }
}

impl Add<Quad> for f64 {
  type Output = Quad;
  fn add(self, y: Quad) -> Quad { dqadd(self, y) }
}
impl Sub<Quad> for f64 {
  type Output = Quad;
  fn sub(self, y: Quad) -> Quad { dqadd(self, -y) }
}
impl Mul<Quad> for f64 {
  type Output = Quad;
  fn mul(self, y: Quad) -> Quad { dqprod(self, y) }
}
impl Div<Quad> for f64 {
  type Output = Quad;
  fn div(self, y: Quad) -> Quad { dqdivide(self, y) }
}

impl Add<Quad> for isize {
  type Output = Quad;
  fn add(self, y: Quad) -> Quad { dqadd(self as f64, y) }
}
impl Sub<Quad> for isize {
  type Output = Quad;
  fn sub(self, y: Quad) -> Quad { dqadd(self as f64, -y) }
}
impl Mul<Quad> for isize {
  type Output = Quad;
  fn mul(self, y: Quad) -> Quad { dqprod(self as f64, y) }
}
impl Div<Quad> for isize {
  type Output = Quad;
  fn div(self, y: Quad) -> Quad { dqdivide(self as f64, y) }
}

impl AddAssign<Quad> for Quad {
  fn add_assign(&mut self, y: Quad) { *self = qqadd(*self, y); }
}
impl SubAssign<Quad> for Quad {
  fn sub_assign(&mut self, y: Quad) { *self = qqadd(*self, -y); }
}
impl MulAssign<Quad> for Quad {
  fn mul_assign(&mut self, y: Quad) { *self = qqprod(*self, y); }
}
impl DivAssign<Quad> for Quad {
  fn div_assign(&mut self, y: Quad) { *self = qqdivide(*self, y); }
}
impl RemAssign<Quad> for Quad {
  fn rem_assign(&mut self, y: Quad) { unimplemented!() }
}

impl AddAssign<f64> for Quad {
  fn add_assign(&mut self, y: f64) { *self = qdadd(*self, y); }
}
impl SubAssign<f64> for Quad {
  fn sub_assign(&mut self, y: f64) { *self = qdadd(*self, -y); }
}
impl MulAssign<f64> for Quad {
  fn mul_assign(&mut self, y: f64) { *self = qdprod(*self, y); }
}
impl DivAssign<f64> for Quad {
  fn div_assign(&mut self, y: f64) { *self = qddivide(*self, y); }
}
impl RemAssign<f64> for Quad {
  fn rem_assign(&mut self, y: f64) { unimplemented!() }
}

impl AddAssign<isize> for Quad {
  fn add_assign(&mut self, y: isize) { *self = qdadd(*self, y as f64); }
}
impl SubAssign<isize> for Quad {
  fn sub_assign(&mut self, y: isize) { *self = qdadd(*self, -y as f64); }
}
impl MulAssign<isize> for Quad {
  fn mul_assign(&mut self, y: isize) { *self = qdprod(*self, y as f64); }
}
impl DivAssign<isize> for Quad {
  fn div_assign(&mut self, y: isize) { *self = qddivide(*self, y as f64); }
}
impl RemAssign<isize> for Quad {
  fn rem_assign(&mut self, y: isize) { unimplemented!() }
}

impl Shl<isize> for Quad {
  type Output = Quad;
  fn shl(self, n: isize) -> Quad { self.scale2(n) }
}
impl Shr<isize> for Quad {
  type Output = Quad;
  fn shr(self, n: isize) -> Quad { self.scale2(n) }
}
impl ShlAssign<isize> for Quad {
  fn shl_assign(&mut self, n: isize) { *self = *self << n; }
}
impl ShrAssign<isize> for Quad {
  fn shr_assign(&mut self, n: isize) { *self = *self >> n; }
}

impl PartialEq<f64> for Quad {
  fn eq(&self, n: &f64) -> bool { *self == Quad(*n, 0.0) }
}

impl PartialEq<isize> for Quad {
  fn eq(&self, n: &isize) -> bool { *self == Quad(*n as f64, 0.0) }
}

impl std::str::FromStr for Quad {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, ()> {
    if s.is_empty() {
      return Err(());
    }
    let mut neg = false;
    let mut dec = false;
    let mut e = 0;
    let mut q = Quad(0.0, 0.0);
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

impl std::fmt::Display for Quad {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // local simple flooring function
    // assumes q >= 0.0
    fn floor0(q:&Quad) -> f64 {
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
    while q >= Quad(10.0, 0.0) {
      e += 1;
      q /= 10.0;
    }
    while q < Quad(1.0, 0.0) {
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
impl Base for Quad {}
impl Zero for Quad {
  const zero: Quad = Quad(0.0, 0.0);
}
impl Addition for Quad {}
impl Subtraction for Quad {}
impl Additive for Quad {}
impl One for Quad {
  const one: Quad = Quad(1.0, 0.0);
}
impl Multiplication for Quad {}
impl Division for Quad {}
impl Multiplicative for Quad {}
impl Embeds<isize> for Quad {}
impl Embeds<f64> for Quad {}
impl Field for Quad {}

/*
-- basic implementation: range-reduce & series
qexp q
  | q<0 = 1/(qexp (-q))
  | otherwise =
    let !nd = q / ln2_q
        !nn = floor.hi_ $ nd
        !r = q - ln2_q * (fromIntegral nn)
        !s = sm r 0.0 1.0 1
    in Quad (scaleFloat nn $ hi_ s) (scaleFloat nn $ lo_ s)
  where sm !r !s !t !n
          | s+t==s = s
          | otherwise = sm r (s+t) (t*r/(fromIntegral n)) (n+1)

*/
