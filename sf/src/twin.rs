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
  + Default
  //+ From
{
  fn SPLIT() -> Self;
  fn mul_add(self, b:Self, c:Self) -> Self;
  fn HAS_MUL_ADD() -> bool;
  fn recip(self) -> Self;
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
  fn ci(c:isize) -> Self;
  fn cf(c:f64) -> Self;
}

impl Base for f32 {
  #[inline] fn SPLIT() -> Self { 4097.0 }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { self.mul_add(b, c) }
  #[inline] fn HAS_MUL_ADD() -> bool { true }
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ci(c:isize) -> Self { c as f32 }
  #[inline] fn cf(c:f64) -> Self { c as f32 }
}
impl Base for f64 {
  #[inline] fn SPLIT() -> Self { 134217729.0 }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { self.mul_add(b, c) }
  #[inline] fn HAS_MUL_ADD() -> bool { true }
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ci(c:isize) -> Self { c as f64 }
  #[inline] fn cf(c:f64) -> Self { c as f64 }
}
impl<F:Base> Base for Twin<F> {
  #[inline] fn SPLIT() -> Twin<F> { Twin::new((F::SPLIT()-F::ci(1))*(F::SPLIT()-F::ci(1)), F::ci(1)) }
  #[inline] fn mul_add(self, b:Self, c:Self) -> Self { unimplemented!() }
  #[inline] fn HAS_MUL_ADD() -> bool { false }
  #[inline] fn recip(self) -> Self { self.recip() }
  #[inline] fn sqrt(self) -> Self { self.sqrt() }
  #[inline] fn cbrt(self) -> Self { self.cbrt() }
  #[inline] fn ci(c:isize) -> Self { Twin::new(F::ci(c),F::default()) }
  #[inline] fn cf(c:f64) -> Self { Twin::new(F::cf(c),F::default()) }
}

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
  if F::HAS_MUL_ADD() {
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
