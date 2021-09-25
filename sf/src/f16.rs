use std::ops::{Add,Sub,Mul,Div,Rem,Neg};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};

#[derive(Clone,Copy,Debug,Default,Hash)]
pub struct f16(pub u16);
//[s:1][e:5][m:10(+1)]

const SIGNB : u16 = 0b1000_0000_0000_0000;
const EXPB  : u16 = 0b0111_1100_0000_0000;
const MANB  : u16 = 0b0000_0011_1111_1111;
const MAN1  : u16 = 0b0000_0100_0000_0000;
const BIAS  : i16 = 15;

impl f16 {
  #[inline]
  pub fn exponent(self) -> i16 {
    ((self.0&EXPB)>>10)as i16 - BIAS
  }

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

impl Mul for f16 {
  type Output = f16;
  fn mul(self, rhs:f16) -> f16 {
    // TODO: this is only correct for normal numbers
    let s = (self.0&SIGNB)^(rhs.0&SIGNB);
    let e = ((self.exponent()+rhs.exponent()+BIAS) as u16)<<10;
    let m0 = ((self.0&MANB)|MAN1) as u32;
    let m1 = ((rhs.0&MANB)|MAN1) as u32;
    let mut e = e;
    let mut m = (((m0 * m1)>>10)as u32);
    while m&((MAN1 as u32)<<1) != 0 {
      m >>= 1;
      e += (1<<10);
    }
    m = (m&(MANB as u32));
    let m = m as u16;
    f16(s|e|m)
  }
}

impl Neg for f16 {
  type Output = f16;
  fn neg(self) -> f16 {
    f16(self.0^SIGNB)
  }
}
