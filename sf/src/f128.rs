use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use std::ops::{Neg};

#[derive(Clone,Copy,Default)]
pub struct f128(u128);
// (1)(15)(112)

const BIAS : i32 = 16384;
const SGNB : u128 = 0x8000_0000_0000_0000__0000_0000_0000_0000;
const EXPB : u128 = 0x7FFF_0000_0000_0000__0000_0000_0000_0000;
const MANB : u128 = 0x0000_FFFF_FFFF_FFFF__FFFF_FFFF_FFFF_FFFF;
const IMPB : u128 = 0x0001_0000_0000_0000__0000_0000_0000_0000;
const SHIFT : u32 = 112;
const MAXRAWEXP : u32 = 0x7FFF;

const ZERO : f128 = f128(0);
const NEGZERO : f128 = f128(SGNB);
const NEGINFINITY : f128 = f128(SGNB|EXPB);
const INFINITY : f128 = f128(EXPB);
const NAN : f128 = f128(EXPB|(1<<111));

impl std::fmt::Debug for f128 {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{:04x}[{:+6}]:({}){:028x}",
      if sign(self.0) {'-'} else {'+'},
      rawexp(self.0),
      exp(self.0),
      if rawexp(self.0)==0 {'0'}
      else if rawexp(self.0)==MAXRAWEXP {'_'}
      else {'1'},
      rawman(self.0)
      )
  }
}

impl f128 {
  #[inline] pub fn to_bits(&self) -> u128 { self.0 }
  #[inline] pub fn from_bits(b:u128) -> f128 { f128(b) }

  #[inline]
  pub fn is_subnormal(&self) -> bool {
    rawexp(self.0)==0 && rawman(self.0)!=0
  }
  #[inline]
  pub fn is_zero(&self) -> bool {
    rawexp(self.0)==0 && rawman(self.0)==0
  }
  #[inline]
  pub fn is_nan(&self) -> bool {
    rawexp(self.0)==MAXRAWEXP && rawman(self.0)!=0
  }
  #[inline]
  pub fn is_infinite(&self) -> bool {
    rawexp(self.0)==MAXRAWEXP && rawman(self.0)==0
  }
  #[inline]
  pub fn is_finite(&self) -> bool {
    rawexp(self.0) != MAXRAWEXP
  }
  #[inline]
  pub fn is_normal(&self) -> bool {
    match rawexp(self.0) {
      0 => { rawman(self.0)==0 }
      MAXRAWEXP => { false }
      _ => { true }
    }
  }

  // TODO: all special cases!
  pub fn from_f64(x:f64) -> f128 {
    let b = x.to_bits();
    let s = ((b & 0x8000_0000_0000_0000) as u128) << 64;
    let e = (((((b & 0x7FF0_0000_0000_0000) >> 52) as i32 - 1023) + BIAS) as u128) << 112;
    let m = ((b & 0x000F_FFFF_FFFF_FFFF) as u128) << 60;
    f128(s | e | m)
  }

  // TODO: no error-checking / special cases!
  pub fn to_f64(self) -> f64 {
    let b = self.to_bits();
    if self.is_zero() { return if sign(b) {-0.0} else {0.0}; }
    let s = (signb(b) >> 64) as u64;
    let e = ((exp(b) + 1023) as u64) << 52;
    // TODO: rounding
    let m = ((man(b) >> 60) & 0x000F_FFFF_FFFF_FFFF) as u64;
    f64::from_bits(s|e|m)
  }
}

impl Neg for f128 {
  type Output = f128;
  fn neg(self) -> f128 {
    f128(neg(self.0))
  }
}

impl Add for f128 {
  type Output = f128;
  fn add(self, rhs:f128) -> f128 {
    f128(add(self.0, rhs.0))
  }
}

impl Sub for f128 {
  type Output = f128;
  fn sub(self, rhs:f128) -> f128 {
    f128(sub(self.0, rhs.0))
  }
}

#[inline]
fn sign(x:u128) -> bool {
  signb(x) != 0
}

#[inline]
fn signb(x:u128) -> u128 {
  x & SGNB
}
#[inline]
fn exp(x:u128) -> i32 {
  (rawexp(x) as i32) - BIAS
}

#[inline]
fn rawexp(x:u128) -> u32 {
  ((x & EXPB) >> SHIFT) as u32
}

#[inline]
fn rawman(x:u128) -> u128 {
  x & MANB
}

#[inline]
fn man(x:u128) -> u128 {
  if rawexp(x) == 0 {
    rawman(x)
  } else {
    rawman(x) | IMPB
  }
}

#[inline]
// TODO: infinities, subnormals
fn build(s:bool, e:i32, m:u128) -> u128 {
  let s = if s {SGNB} else {0};
  if m == 0 { return s; }
  let z = m.leading_zeros() as i32;
  let n;
  if z > 15 {
    n = (m << (z - 15)) & MANB;
  } else if z < 15 {
    // TODO: rounding?!
    n = (m >> (15 - z)) & MANB;
  } else {
    n = m & MANB;
  } 
  let e = (((e + (15 - z)) + BIAS) as u128) << SHIFT;
  s | e | n
}

// rounds-to-nearest (tie-to-even)
// rounds to 4th lsb based on lowest 3 bits
fn round(m:u128) -> u128 {
  let m0 = m & 0b111;
  let mx = m & !0b111;
  if m0 > 0b100 {
    // round up
    mx + 0b1000
  } else if m0 == 0b100 {
    // round to even
    mx + (m & 0b1000)
  } else {
    // round down
    mx
  }
}

#[inline]
pub fn neg(x:u128) -> u128 {
  x ^ SGNB
}

#[inline]
pub fn add(x:u128,y:u128) -> u128 {
  // TODO: cleanup the special cases
  if x==0 { return y; }
  if y==0 { return x; }
  if f128(x).is_nan() { return x; }
  if f128(y).is_nan() { return y; }
  match (f128(x).is_infinite(), f128(y).is_infinite()) {
    (true,false) => { return x; }
    (false,true) => { return y; }
    (true,true) => { return if sign(x)==sign(y) {x} else {NAN.0}; }
    _ => {}
  }

  if sign(x) != sign(y) {
    return sub(x, neg(y))
  }

  let s = sign(x);
  let ex = exp(x);
  let ey = exp(y);
  let e = ex.max(ey);
  let mx = (man(x) << 3) >> (e - ex); // TODO: sticky bit
  let my = (man(y) << 3) >> (e - ey); // TODO: sticky bit
  let m = mx + my;
  let m = round(m) >> 3;
  build(s, e, m)
}

#[inline]
pub fn sub(x:u128,y:u128) -> u128 {
  // TODO: cleanup the special cases
  if x==0 { return neg(y); }
  if y==0 { return x; }
  if f128(x).is_nan() { return x; }
  if f128(y).is_nan() { return neg(y); }
  match (f128(x).is_infinite(), f128(y).is_infinite()) {
    (true,false) => { return x; }
    (false,true) => { return neg(y); }
    (true,true) => { return if sign(x)!=sign(y) {x} else {NAN.0}; }
    _ => {}
  }

  if sign(x) != sign(y) {
    return add(x, neg(y))
  }

  if x>y {
    let s = sign(x);
    let ex = exp(x);
    let ey = exp(y);
    let e = ex.max(ey);
    let mx = man(x) << 3;
    let my = (man(y) << 3) >> (e - ey); // TODO: sticky bit
    let m = mx - my;
    let m = round(m) >> 3;
    build(s, e, m)
  } else if x<y {
    let s = !sign(y);
    let ex = exp(x);
    let ey = exp(y);
    let e = ex.max(ey);
    let mx = (man(x) << 3) >> (e - ex); // TODO: sticky bit
    let my = man(y) << 3;
    let m = my - mx;
    let m = round(m) >> 3;
    build(s, e, m)
  } else {
    0
  }
}


