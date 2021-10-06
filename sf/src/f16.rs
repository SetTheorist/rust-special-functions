use std::ops::{Add,Sub,Mul,Div,Rem,Neg};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};

#[derive(Clone,Copy,Default,Hash)]
pub struct f16(pub u16);
//[s:1][e:5][m:10(+1)]

const SIGNB : u16 = 0b1000_0000_0000_0000;
const EXPB  : u16 = 0b0111_1100_0000_0000;
const MANB  : u16 = 0b0000_0011_1111_1111;
const IMPB  : u16 = 0b0000_0100_0000_0000;
const BIAS  : i16 = 15;
const MAXE  : i16 = 15;
const MINE  : i16 = -14;
const INF   : u16 = 0b0111_1100_0000_0000;

impl std::fmt::Debug for f16 {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    // TODO: inf, nan, zero
    write!(f, "[{:04X}:{}1.{:010b}p{:+}~{:e}]", self.0,
      if self.0&SIGNB==0{'+'}else{'-'},
      self.0&MANB,
      (((self.0&EXPB)>>10) as i16)-BIAS,
      self.to_f32()
      )
  }
}

impl f16 {
  #[inline] pub fn exponent(self) -> i16 { ((self.0&EXPB)>>10)as i16 - BIAS }
  #[inline] pub fn to_bits(self) -> u16 { self.0 }
  #[inline] pub fn from_bits(b:u16) -> f16 { f16(b) }

  // TODO: macro-ify this to make cleaner
  #[inline]
  pub fn to_f32(self) -> f32 {
    // TODO: this is only correct for normal numbers
    let s : u32 = ((self.0&SIGNB) as u32);
    let e : u32 = (((((self.0&EXPB)>>10)as i16 - BIAS)+127) as u32);
    let m : u32 = ((self.0&MANB) as u32);
    f32::from_bits(s<<16|e<<23|m<<13)
  }
  #[inline]
  pub fn from_f32(x:f32) -> f16 {
    // TODO: this is only correct for normal numbers
    // TODO: I'm not sure it's correct at all!
    let b = x.to_bits();
    let s : u16 = ((b&(1<<31))>>16) as u16;
    let e : u16 = ((((((b>>23)&0xFF) as i16)-127)+BIAS) as u16)<<10;
    let m : u16 = ((b&0x3FFFFF)>>12) as u16;
    f16(s|e|m)
  }
  #[inline]
  pub fn to_f64(self) -> f64 {
    // TODO: this is only correct for normal numbers
    let s : u64 = ((self.0&SIGNB) as u64);
    let e : u64 = (((((self.0&EXPB)>>10)as i16 - BIAS)+1023) as u64);
    let m : u64 = ((self.0&MANB) as u64);
    f64::from_bits(s<<48|e<<52|m<<42)
  }

  #[inline]
  pub fn negative(self) -> bool {
    self.0&SIGNB != 0
  }
}

//[s:1][e:5][m:10(+1)]

#[inline]
fn s(x:u16) -> u16 {
  x & SIGNB
}
#[inline]
fn e(x:u16) -> i16 {
  (((x & EXPB) >> 10) as i16) - BIAS
}
#[inline]
fn m(x:u16) -> u16 {
  if (x & EXPB) == 0 {
    x & MANB
  } else {
    (x & MANB) | IMPB
  }
}
fn split(x:u16) -> (u16,i16,u16) {
  let s = s(x);
  let mut e = e(x);
  let mut m = m(x);
  if m != 0 {
    while m & IMPB == 0 {
      m <<= 1;
      e -= 1;
    }
  }
  (s, e, m)
}

// rounds to 4th bit based on lsb 3 bits (guard/round/sticky)
// (round-to-nearest, ties-to-even)
#[inline]
pub fn round(m:u16) -> u16 {
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

// s: 0 or SIGNB
// e: signed, un-biased unshifted exponent
// m: 11 bits including implicit (1=implicit, 10=mantissa)
#[inline]
pub fn make(s:u16, e:i16, m:u16) -> u16 {
  if m == 0 { return s; } // zero
  let mut e = e;
  let mut m = m;
  // normalize to get mantissa in exactly bottom 11 bits
  while m & !(IMPB|MANB) != 0 {
    m >>= 1;
    e += 1;
  }
  while m & IMPB == 0 {
    m <<= 1;
    e -= 1;
  }
  if e > MAXE {
    // infinity
    return s | INF;
  } else if e < MINE {
    // denormal or zero
    m >>= (MINE - e).min(11);
    return s | m;
  }
  s | (((e + BIAS) as u16) << 10) | (m & MANB)
}

#[inline]
fn neg(a:u16) -> u16 {
  a ^ SIGNB
}
#[inline]
// TODO: doesn't handle nan or inf
fn add(a:u16, b:u16) -> u16 {
  if s(a) != s(b) { return sub(a, neg(b)); }
  if e(a) < e(b) { return add(b, a); }
  let (sa, ea, ma) = split(a);
  let (sb, eb, mb) = split(b);
  if ea > eb+14 { return a; }
  let ma = ma << 3;
  // TODO: sticky-bit!
  let mb = (mb << 3) >> (ea - eb);
  let s = sa;
  let e = ea;
  let m = ma + mb;
  let m = round(m) >> 3;
  make(s, e, m)
}
#[inline]
fn sub(a:u16, b:u16) -> u16 {
  todo!()
}
#[inline]
// TODO: doesn't handle nan or inf
fn mul(a:u16, b:u16) -> u16 {
  let (sa, ea, ma) = split(a);
  let (sb, eb, mb) = split(b);
  let s = sa ^ sb;
  let e = ea + eb;
  let mx = (ma as u32) * (mb as u32);
  if mx == 0 { return s; } 
  let sb = if mx & 0b1111111 != 0 {1} else {0};
  let m = ((mx >> 7) | sb) as u16;
  let m = round(m) >> 3;
  make(s, e, m)
}
#[inline]
fn div(a:u16, b:u16) -> u16 {
  todo!()
}

impl Neg for f16 {
  type Output = f16;
  fn neg(self) -> f16 {
    f16(neg(self.0))
  }
}

impl Add for f16 {
  type Output = f16;
  fn add(self, rhs:f16) -> f16 {
    f16(add(self.0, rhs.0))
  }
}

impl Mul for f16 {
  type Output = f16;
  fn mul(self, rhs:f16) -> f16 {
    f16(mul(self.0, rhs.0))
  }
}


