use std::ops::{Add,Sub,Mul,Div,Rem};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use std::ops::{Neg};

#[derive(Clone,Copy,Debug,Default,PartialEq,PartialOrd)] // Eq,Ord
pub struct Twin<F>{hi:F, lo:F}

pub trait Base: Sized + Copy
  + Add<Output=Self>
  + Sub<Output=Self>
  + Mul<Output=Self>
  + Div<Output=Self>
  + Neg<Output=Self>
  + PartialOrd + PartialEq
  + Default
{
  fn SPLIT() -> Self;
  fn mul_add(self, b:Self, c:Self) -> Self;
  const HAS_MUL_ADD : bool;
  fn recip(self) -> Self;
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
  fn ceil(self) -> Self;
  fn floor(self) -> Self;
  fn round(self) -> Self;
  fn trunc(self) -> Self;
  fn abs(self) -> Self;
  fn ci(c:isize) -> Self;
  fn cf(c:f64) -> Self;
  fn to64(self) -> f64;
  fn epsilon() -> Self;
}

impl Base for f32 {
  #[inline] fn SPLIT() -> Self { 4097.0 }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { self.mul_add(b, c) }
  const HAS_MUL_ADD : bool = true;
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ceil(self) -> Self { self.ceil() }
  #[inline] fn floor(self) -> Self { self.floor() }
  #[inline] fn round(self) -> Self { self.round() }
  #[inline] fn trunc(self) -> Self { self.trunc() }
  #[inline] fn abs(self) -> Self { self.abs() }
  #[inline] fn ci(c:isize) -> Self { c as f32 }
  #[inline] fn cf(c:f64) -> Self { c as f32 }
  #[inline] fn to64(self) -> f64 { self as f64 }
  #[inline] fn epsilon() -> Self { f32::EPSILON }
}
impl Base for f64 {
  #[inline] fn SPLIT() -> Self { 134217729.0 }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { self.mul_add(b, c) }
  const HAS_MUL_ADD : bool = true;
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ceil(self) -> Self { self.ceil() }
  #[inline] fn floor(self) -> Self { self.floor() }
  #[inline] fn round(self) -> Self { self.round() }
  #[inline] fn trunc(self) -> Self { self.trunc() }
  #[inline] fn abs(self) -> Self { self.abs() }
  #[inline] fn ci(c:isize) -> Self { c as f64 }
  #[inline] fn cf(c:f64) -> Self { c as f64 }
  #[inline] fn to64(self) -> f64 { self }
  #[inline] fn epsilon() -> Self { f64::EPSILON }
}
impl<F:Base> Base for Twin<F> {
  #[inline] fn SPLIT() -> Twin<F> { Twin::new((F::SPLIT()-F::ci(1))*(F::SPLIT()-F::ci(1)), F::ci(1)) }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { unimplemented!() }
  const HAS_MUL_ADD : bool = false;
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ceil(self) -> Self { self.ceil() }
  #[inline] fn floor(self) -> Self { self.floor() }
  #[inline] fn round(self) -> Self { self.round() }
  #[inline] fn trunc(self) -> Self { self.trunc() }
  #[inline] fn abs(self) -> Self { self.abs() }
  #[inline] fn ci(c:isize) -> Self { Twin::new(F::ci(c),F::default()) }
  #[inline] fn cf(c:f64) -> Self { Twin::new(F::cf(c),F::default()) }
  #[inline] fn to64(self) -> f64 { self.hi.to64() }
  #[inline] fn epsilon() -> Self {
    Twin{hi:F::epsilon(),lo:F::default()}*Twin{hi:F::epsilon(),lo:F::default()}
  }
}

/*
use crate::f128::*;
impl Base for f128 {
  #[inline] fn SPLIT() -> Self { f128::from_bits(0x4037_0000_0000_0000__0100_0000_0000_0000) } // 72057594037927937
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { unimplemented!() }
  const HAS_MUL_ADD : bool = false;
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ceil(self) -> Self { self.ceil() }
  #[inline] fn floor(self) -> Self { self.floor() }
  #[inline] fn round(self) -> Self { self.round() }
  #[inline] fn trunc(self) -> Self { self.trunc() }
  #[inline] fn abs(self) -> Self { self.abs() }
  #[inline] fn ci(c:isize) -> Self { f128::from(c) }
  #[inline] fn cf(c:f64) -> Self { f128::from(c) }
  #[inline] fn to64(self) -> f64 { f64::from(self) }
  #[inline] fn epsilon() -> Self { f128::from(f64::EPSILON).sqr() }
}
*/

////////////////////////////////////////////////////////////////////////////////

// requires |a|>=|b|
#[inline]
fn qtsum<F:Base>(a:F, b:F) -> (F, F) {
  let s = a + b;
  let e = b + (a - s); // = b-(s-a)
  (s, e)
}

// general
#[inline]
fn ddsum<F:Base>(a:F, b:F) -> (F, F) {
  let s = a + b;
  let v = s - a;
  let e = (a + (v - s)) + (b - v); // = (a-(s-v))+(b-v)
  (s, e)
}

#[inline]
fn split<F:Base>(a:F) -> (F, F) {
  let t = F::SPLIT() * a;
  let ahi = t - (t - a);
  let alo = a - ahi;
  (ahi, alo)
}

#[inline]
fn ddprod<F:Base>(a:F, b:F) -> (F, F) {
  if F::HAS_MUL_ADD {
    let p = a * b;
    let e = a.mul_add(b, -p);
    (p, e)
  } else {
    let (ahi, alo) = split(a);
    let (bhi, blo) = split(b);
    let p = a * b;
    let e = (((ahi * bhi - p) + ahi * blo) + alo * bhi) + alo * blo;
    (p, e)
  }
}

#[inline]
fn qdadd<F:Base>(Twin{hi:xhi, lo:xlo}:Twin<F>, y:F) -> Twin<F> {
  let (shi, slo) = ddsum(y, xhi);
  let (hhi, hlo) = qtsum(shi, slo + xlo);
  let (hi, lo) = qtsum(hhi, hlo);
  Twin{hi, lo}
}

#[inline]
fn dqadd<F:Base>(x:F, y:Twin<F>) -> Twin<F> { qdadd(y, x) }

#[inline]
fn qqadd<F:Base>(Twin{hi:xhi, lo:xlo}:Twin<F>, Twin{hi:yhi, lo:ylo}:Twin<F>) -> Twin<F> {
  let (hs, he) = ddsum(xhi, yhi);
  let (ls, le) = ddsum(xlo, ylo);
  let (h, k) = qtsum(hs, he + ls);
  let (hi, lo) = qtsum(h, le + k);
  Twin{hi, lo}
}

#[inline]
fn qnegate<F:Base>(Twin{hi, lo}:Twin<F>) -> Twin<F> { Twin{hi:-hi, lo:-lo} }

#[inline]
fn qdprod<F:Base>(Twin{hi:xhi, lo:xlo}: Twin<F>, y:F) -> Twin<F> {
  let (thi, tlo) = ddprod(xhi, y);
  let (hi, lo) = qtsum(thi, tlo + y * xlo);
  Twin{hi, lo}
}

#[inline]
fn dqprod<F:Base>(x:F, y: Twin<F>) -> Twin<F> { qdprod(y, x) }

#[inline]
fn qqprod<F:Base>(Twin{hi:xhi, lo:xlo}:Twin<F>, Twin{hi:yhi, lo:ylo}:Twin<F>) -> Twin<F> {
  let (p, e) = ddprod(xhi, yhi);
  let (hi, lo) = qtsum(p, e + (xhi * ylo + xlo * yhi));
  Twin{hi, lo}
}

#[inline]
fn qqdivide<F:Base>(Twin{hi:xhi, lo:xlo}:Twin<F>, Twin{hi:yhi, lo:ylo}:Twin<F>) -> Twin<F> {
  let cc = xhi / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = ((((xhi - uu) - u) + xlo) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Twin{hi, lo}
}

#[inline]
fn dqdivide<F:Base>(x:F, Twin{hi:yhi, lo:ylo}:Twin<F>) -> Twin<F> {
  let cc = x / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = (((x - uu) - u) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Twin{hi, lo}
}

#[inline]
fn qddivide<F:Base>(Twin{hi:xhi, lo:xlo}:Twin<F>, y:F) -> Twin<F> {
  let xdy = xhi / y;
  let (uu, u) = ddprod(xdy, y);
  let c = (((xhi - uu) - u) + xlo) / y;
  let (hi, lo) = qtsum(xdy, c);
  Twin{hi, lo}
}

////////////////////////////////////////////////////////////////////////////////

impl<F:Base> Twin<F> {
  // construction
  #[inline]
  pub fn new(a:F, b:F) -> Self {
    let (hi, lo) = ddsum(a, b);
    Twin{hi, lo}
  }

  #[inline]
  // Does not check preconditions
  pub unsafe fn new_raw(a:F, b:F) -> Self {
    Twin{hi:a, lo:b}
  }

  // deconstruction
  #[inline]
  pub fn parts(Twin{hi, lo}: Self) -> (F, F) { (hi, lo) }
  #[inline]
  pub fn hi(self) -> F { self.hi }
  #[inline]
  pub fn lo(self) -> F { self.lo }

  #[inline]
  pub fn recip(self) -> Self {
    // TODO: better
    Self::new(F::ci(1),F::default()) / self
  }


  // TODO: more efficient
  #[inline]
  pub fn sqr(self) -> Self {
    self*self
  }

  pub fn sqrt(self) -> Self {
    let q0 = self.hi.sqrt();
    let x = Self::new(q0, F::default());
    let x = (x+self/x)*F::cf(0.5); // TODO: ldexp
    x
  }

  pub fn sqrt_recip(self) -> Self {
    let z = F::default();
    let c3 = F::ci(3);
    let c1_2 = F::cf(0.5);

    let q0 = self.hi.sqrt().recip();
    let x = Self::new(q0, z);
    //let x = x + x*(1 - self*x.sqr())*0.5; // alternative form
    let x = x*(-self*x.sqr() + c3)*c1_2; // TODO: ldexp
    x
  }

  pub fn cbrt(self) -> Self {
    let z = F::default();
    let c2 = F::ci(2);
    let c3 = F::ci(3);
    let q0 = self.hi.cbrt();
    let x = Self::new(q0, z);
    let x = (x*c2 + self/x.sqr())/c3; // TODO: ldexp
    x
  }

  pub fn cbrt_recip(self) -> Self {
    let z = F::default();
    let c3 = F::ci(3);
    let c4 = F::ci(4);
    let q0 = self.hi.cbrt().recip();
    let x = Self::new(q0, z);
    let x = x*(-self*x*x.sqr() + c4)/c3;
    let x = x*(-self*x*x.sqr() + c4)/c3;
    x
  }

  pub fn floor(self) -> Self {
    let xhi = self.hi.floor();
    if self.hi == xhi {
      let xlo = self.lo.floor();
      let (hi, lo) = qtsum(xhi, xlo);
      Twin{hi, lo}
    } else {
      Twin{hi:xhi, lo:F::default()}
    }
  }

  pub fn ceil(self) -> Self {
    let xhi = self.hi.ceil();
    if self.hi == xhi {
      let xlo = self.lo.ceil();
      let (hi, lo) = qtsum(xhi, xlo);
      Twin{hi, lo}
    } else {
      Twin{hi:xhi, lo:F::default()}
    }
  }

  pub fn round(self) -> Self {
    let xhi = self.hi.round();
    if self.hi == xhi {
      let xlo = self.lo.round();
      let (hi, lo) = qtsum(xhi, xlo);
      Twin{hi, lo}
    } else {
      if (xhi-self.hi).abs()==F::cf(0.5) && self.lo < F::default() {
        Twin{hi:xhi-F::ci(1), lo:F::default()}
      } else {
        Twin{hi:xhi, lo:F::default()}
      }
    }
  }

  pub fn trunc(self) -> Self {
    let xhi = self.hi.trunc();
    if self.hi == xhi {
      let xlo = self.lo.trunc();
      let (hi, lo) = qtsum(xhi, xlo);
      Twin{hi, lo}
    } else {
      Twin{hi:xhi, lo:F::default()}
    }
  }

  #[inline]
  pub fn abs(self) -> Self {
    if self < Self::default() {
      -self
    } else {
      self
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

impl<F:Base> Neg for Twin<F> {
  type Output = Self;
  fn neg(self) -> Self { qnegate(self) }
}
impl<F:Base> Add<Self> for Twin<F> {
  type Output = Self;
  fn add(self, y: Self) -> Self { qqadd(self, y) }
}
impl<F:Base> Sub<Self> for Twin<F> {
  type Output = Self;
  fn sub(self, y: Self) -> Self { qqadd(self, -y) }
}
impl<F:Base> Mul<Self> for Twin<F> {
  type Output = Self;
  fn mul(self, y: Self) -> Self { qqprod(self, y) }
}
impl<F:Base> Div<Self> for Twin<F> {
  type Output = Self;
  fn div(self, y: Self) -> Self { qqdivide(self, y) }
}

impl<F:Base> Add<F> for Twin<F> {
  type Output = Self;
  fn add(self, y:F) -> Self { qdadd(self, y) }
}
impl<F:Base> Sub<F> for Twin<F> {
  type Output = Self;
  fn sub(self, y:F) -> Self { qdadd(self, -y) }
}
impl<F:Base> Mul<F> for Twin<F> {
  type Output = Self;
  fn mul(self, y:F) -> Self { qdprod(self, y) }
}
impl<F:Base> Div<F> for Twin<F> {
  type Output = Self;
  fn div(self, y:F) -> Self { qddivide(self, y) }
}

/*
// Rust restrictions block these (generic) implementation, sigh
impl<F:Base> Add<Twin<F>> for F {
  type Output = Twin<F>;
  fn add(self, y:Twin<F>) -> Self { dqadd(self, y) }
}
impl<F:Base> Sub<Twin<F>> for F {
  type Output = Twin<F>;
  fn sub(self, y:Twin<F>) -> Self { dqadd(self, -y) }
}
impl<F:Base> Mul<Twin<F>> for F {
  type Output = Twin<F>;
  fn mul(self, y:Twin<F>) -> Self { dqprod(self, y) }
}
impl<F:Base> Div<Twin<F>> for F {
  type Output = Twin<F>;
  fn div(self, y:Twin<F>) -> Self { dqdivide(self, y) }
}
*/

////////////////////////////////////////////////////////////////////////////////


impl<F:Base> std::fmt::Display for Twin<F> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let ZERO = Self::default();
    let ONE = Self::ci(1);
    let TEN = Self::ci(10);
    //if self.is_nan() { return write!(f, "NaN"); }
    //if self.is_infinite() { return write!(f, "{}", if self<ZERO {"-Inf"} else {"Inf"}); }
    let mut z = *self;
    if z < ZERO {
      z = -z;
      write!(f, "-")?;
    }
    let mut e = 0;
    if z == ZERO { // don't rescale zero
      while z >= TEN {
        e += 1;
        z = z / TEN;
      }
      while z < ONE {
        e -= 1;
        z = z * TEN;
      }
    }
    let digs = ((Self::epsilon().to64().recip()*1.1).log10().ceil() as isize) + 4;
    println!("{}", digs);
    for n in 0..digs {
      if n == 1 {
        write!(f, ".")?;
      }
      let d = z.floor().to64();
      if d<0.0 || d>=10.0 { eprintln!("<<{}>>", d); }
      let dd = ((d as u8) + b'0') as char;
      write!(f, "{}", dd)?;
      let d0 = Self::cf(d);
      z = (z - d0) * TEN;
    }
    if e != 0 { write!(f, "e{}", e)?; }
    write!(f, "")
  }
}

