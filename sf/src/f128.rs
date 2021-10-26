use std::cmp::{Ordering,PartialEq,PartialOrd};
use std::ops::{Add,Sub,Mul,Div};
use std::ops::{AddAssign,SubAssign,MulAssign,DivAssign};
use std::ops::{Neg};

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct f128(u128);
// (1)(15)(112)

////////////////////////////////////////////////////////////////////////////////

const BIAS : i32 = 16383;
const SGNB : u128 = 0x8000_0000_0000_0000__0000_0000_0000_0000;
const EXPB : u128 = 0x7FFF_0000_0000_0000__0000_0000_0000_0000;
const MANB : u128 = 0x0000_FFFF_FFFF_FFFF__FFFF_FFFF_FFFF_FFFF;
const IMPB : u128 = 0x0001_0000_0000_0000__0000_0000_0000_0000;
const SHIFT : u32 = 112;
const MAXRAWEXP : u32 = 0x7FFF;

const ZERO        : f128 = f128(0);
const NEGZERO     : f128 = f128(SGNB);
const NEGINFINITY : f128 = f128(SGNB|EXPB);
const INFINITY    : f128 = f128(EXPB);
const NAN         : f128 = f128(EXPB|(1<<111));
const NEGNAN      : f128 = f128(SGNB|EXPB|(1<<111));
const ONE         : f128 = f128((BIAS as u128)<<SHIFT);
const THREE       : f128 = f128(0x4000_8000_0000_0000__0000_0000_0000_0000);
const FOUR        : f128 = f128(0x4001_0000_0000_0000__0000_0000_0000_0000);
const TEN         : f128 = f128(0x4002_4000_0000_0000__0000_0000_0000_0000);

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

impl std::fmt::Display for f128 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut z = *self;
    if z < ZERO {
      z = -z;
      write!(f, "-")?;
    }
    let mut e = 0;
    while z >= TEN {
      e += 1;
      z = z / TEN;
    }
    while z < ONE {
      e -= 1;
      z = z * TEN;
    }
    for n in 0..35 {
      if n == 1 {
        write!(f, ".")?;
      }
      let d = f64::from(z.floor()); // TODO: make & use f128::rint()
      if d<0.0 || d>=10.0 { eprintln!("<<{}>>", d); }
      let dd = ((d as u8) + b'0') as char;
      write!(f, "{}", dd)?;
      let d0 = f128::from(d);
      z = (z - d0) * TEN;
    }
    if e != 0 { write!(f, "e{}", e)?; }
    write!(f, "")
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

  #[inline]
  pub fn recip(self) -> f128 {
    f128(recip(self.0))
  }

  pub fn ceil(self) -> f128 {
    if self.is_nan() || self.is_infinite() || self.is_zero() { return self; }
    if sign(self.0) { return -(-self).floor(); }
    // TODO: this is a dumb way to do it
    let f = self.floor();
    if f != self { f + ONE } else { f }
  }

  pub fn floor(self) -> f128 {
    if self.is_nan() || self.is_infinite() || self.is_zero() { return self; }
    if sign(self.0) { return -(-self).ceil(); }
    let e = exp(self.0);
    if e < 0 { return ZERO; }
    if e > 112 { return self; }
    return f128(self.0 & !((1<<(112-e))-1));
  }

  #[inline]
  pub fn frexp(self) -> (f128, i32) {
    let e = exp(self.0);
    let f = f128((self.0&!EXPB)|((BIAS as u128)<<SHIFT));
    (f, e)
  }

  #[inline]
  pub fn ldexp(self, n:i32) -> f128 {
    if self.is_nan() { return self; }
    if self.is_infinite() { return self; }
    if self.is_zero() { return if n%2==0 {self} else {-self}; }
    // TODO: overflow/underflow/etc.
    let e = exp(self.0);
    f128(self.0.wrapping_add((n as u128)<<SHIFT))
  }

  #[inline]
  // TODO: ldexp, edge cases, special cases, etc.
  fn mul2(self) -> f128 {
    f128(self.0 + (1 << SHIFT))
  }

  #[inline]
  // TODO: ldexp, edge cases, special cases, etc.
  fn div2(self) -> f128 {
    f128(self.0.wrapping_add(!0 << SHIFT))
  }

  #[inline]
  fn sqr(self) -> f128 {
    // TODO: can we specialize this more efficiently?
    self*self
  }

  #[inline]
  fn cub(self) -> f128 {
    // TODO: can we specialize this more efficiently?
    self.sqr()*self
  }

  #[inline]
  fn powu(self, n:usize) -> f128 {
    let mut n = n;
    let mut x = self;
    let mut v = ONE;
    while n != 0 {
      if n % 2 == 1 { v *= x; }
      x = x.sqr();
      n >>= 1;
    }
    v
  }

  #[inline]
  fn powi(self, n:isize) -> f128 {
    if n < 0 {
      self.powu(-n as usize).recip()
    } else {
      self.powu(n as usize)
    }
  }


  pub fn sqrt(self) -> f128 {
    // TODO: special cases, negatives
    let (f_,e_) = self.frexp();
    let f;
    let e;
    if e_%2==1 {
      f = f_.mul2();
      e = (e_-1)/2;
    } else {
      f = f_;
      e = e_/2;
    }
    let z = f128::from(f64::from(f).sqrt());
    let z = (z + f/z).div2();
    let z = (z + f/z).div2();
    let z = (z + f/z).div2();
    f128(z.0.wrapping_add((e as u128) << 112))
  }

  pub fn sqrt_recip(self) -> f128 {
    // TODO: special cases, negatives
    let (f_,e_) = self.frexp();
    let f;
    let e;
    if e_%2==1 {
      f = f_.mul2();
      e = -(e_-1)/2;
    } else {
      f = f_;
      e = -e_/2;
    }
    let z = f128::from(f64::from(f).sqrt().recip());
    let z = ((THREE - f*z*z)*z).div2();
    let z = ((THREE - f*z*z)*z).div2();
    let z = ((THREE - f*z*z)*z).div2();
    f128(z.0.wrapping_add((e as u128) << 112))
  }

  pub fn cbrt(self) -> f128 {
    let x = f128::from(f64::from(self).cbrt());
    let x = (x.mul2() + self/x.sqr())/f128::from(3.0_f64);
    let x = (x.mul2() + self/x.sqr())/f128::from(3.0_f64);
    x
  }

  pub fn cbrt_recip(self) -> f128 {
    let x = f128::from(f64::from(self).cbrt().recip());
    let x = x*(FOUR-self*x.cub())/THREE;
    let x = x*(FOUR-self*x.cub())/THREE;
    x
  }

  pub fn nth_root(self, n:isize) -> f128 {
    let x = f128::from(f64::from(self).powf((n as f64).recip()));
    let n1 = f128::from(n-1);
    let nn = f128::from(n);
    let x = (x*n1 + self/x.powi(n-1))/nn;
    let x = (x*n1 + self/x.powi(n-1))/nn;
    x
  }

}

////////////////////////////////////////////////////////////////////////////////

macro_rules! from_int {
  ($u:ty,$i:ty) => {
    impl From<$u> for f128 {
      #[inline]
      fn from(x:$u) -> f128 {
        let l = x.log2();
        let m = ((x as u128) << (112-l)) & MANB;
        let e = ((l as i32) + BIAS) as u128;
        f128(m | (e<<SHIFT))
      }
    }
    impl From<$i> for f128 {
      #[inline]
      fn from(x:$i) -> f128 {
        let f = f128::from(x.abs() as $u);
        if x >= 0 { f } else { -f }
      }
    }
  }
}
from_int!{u8,i8}
from_int!{u16,i16}
from_int!{u32,i32}
from_int!{u64,i64}
from_int!{usize,isize}

impl From<u128> for f128 {
  #[inline]
  fn from(x:u128) -> f128 {
    let l = x.log2();
    let m =
      if l <= 112 {
        (x << (112-l)) & MANB
      } else {
        // TODO: rounding (shifted-off bits)
        (x >> (l-112)) & MANB
      };
    let e = ((l as i32) + BIAS) as u128;
    f128(m | (e<<SHIFT))
  }
}
impl From<i128> for f128 {
  #[inline]
  fn from(x:i128) -> f128 {
    let f = f128::from(x.abs() as u128);
    if x >= 0 { f } else { -f }
  }
}

impl From<f64> for f128 {
  fn from(x:f64) -> f128 {
    if x.is_nan() { return if x.is_sign_negative() {NEGNAN} else {NAN}; }
    if x.is_infinite() { return if x.is_sign_negative() {NEGINFINITY} else {INFINITY}; }
    if x==0.0 { return if x.is_sign_negative() {NEGZERO} else {ZERO}; }
    let b = x.to_bits();
    let s = ((b & 0x8000_0000_0000_0000) as u128) << 64;
    let e = (((((b & 0x7FF0_0000_0000_0000) >> 52) as i32 - 1023) + BIAS) as u128) << 112;
    let m = ((b & 0x000F_FFFF_FFFF_FFFF) as u128) << 60;
    f128(s | e | m)
  }
}

impl From<f128> for f64 {
  // TODO: no error-checking / special cases!
  fn from(x:f128) -> f64 {
    let b = x.to_bits();
    if x.is_zero() { return f64::from_bits((b>>64) as u64); }
    let s = (signb(b) >> 64) as u64;
    let e = exp(b) + 1023;
    if e < 0 {
      // TODO: subnormal
      return f64::from_bits(s);
    } else if e > 2047 {
      return if sign(b) {f64::NEG_INFINITY} else {f64::INFINITY};
    }
    let mut e = ((exp(b) + 1023) as u64) << 52;
    let mb = man(b);
    let low = mb & 0x0000_0000_0000_0000__0FFF_FFFF_FFFF_FFFF;
    let sb = {
        if low > 0x0000_0000_0000_0000__0800_0000_0000_0000 {1} // round-up
        else if low < 0x0000_0000_0000_0000__0800_0000_0000_0000 {0} // round-down
        else { (mb>>60)&1 } // round-to-even
      };
    let mut m0 = mb + (sb << 60);
    if m0 & (1<<113) != 0 { m0 >>= 1; e += 1; }
    let m = ((m0 >> 60) & 0x000F_FFFF_FFFF_FFFF) as u64;
    f64::from_bits(s|e|m)
  }
}

////////////////////////////////////////////////////////////////////////////////

impl Default for f128 {
  #[inline]
  fn default() -> f128 {
    f128(0)
  }
}

////////////////////////////////////////////////////////////////////////////////

impl PartialEq for f128 {
  #[inline]
  fn eq(&self, rhs:&f128) -> bool {
    if self.is_nan() || rhs.is_nan() {
      false
    } else if self.is_zero() && rhs.is_zero() {
      true
    } else {
      self.0 == rhs.0
    }
  }
}

impl PartialOrd for f128 {
  fn partial_cmp(&self, rhs:&f128) -> Option<Ordering> {
    if self.is_nan() || rhs.is_nan() {
      None
    } else if self.is_zero() && rhs.is_zero() {
      Some(Ordering::Equal)
    } else if self.is_zero() {
      if sign(rhs.0) {Some(Ordering::Greater)}
      else {Some(Ordering::Less)}
    } else if rhs.is_zero() {
      if sign(self.0) {Some(Ordering::Less)}
      else {Some(Ordering::Greater)}
    } else {
      let sa = sign(self.0);
      let sb = sign(rhs.0);
      if sa != sb {
        if sa {Some(Ordering::Less)}
        else {Some(Ordering::Greater)}
      } else if sa {
        rhs.0.partial_cmp(&self.0)
      } else {
        self.0.partial_cmp(&rhs.0)
      }
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

impl Neg for f128 {
  type Output = f128;
  #[inline]
  fn neg(self) -> f128 {
    f128(neg(self.0))
  }
}

impl Add for f128 {
  type Output = f128;
  #[inline]
  fn add(self, rhs:f128) -> f128 {
    f128(add(self.0, rhs.0))
  }
}

impl Sub for f128 {
  type Output = f128;
  #[inline]
  fn sub(self, rhs:f128) -> f128 {
    f128(sub(self.0, rhs.0))
  }
}

impl Mul for f128 {
  type Output = f128;
  #[inline]
  fn mul(self, rhs:f128) -> f128 {
    f128(mul(self.0, rhs.0))
  }
}

impl Div for f128 {
  type Output = f128;
  #[inline]
  fn div(self, rhs:f128) -> f128 {
    f128(mul(self.0, recip(rhs.0)))
  }
}

////////////////////////////////////////////////////////////////////////////////

impl AddAssign for f128 {
  #[inline]
  fn add_assign(&mut self, rhs:f128) {
    *self = *self + rhs;
  }
}
impl SubAssign for f128 {
  #[inline]
  fn sub_assign(&mut self, rhs:f128) {
    *self = *self - rhs;
  }
}
impl MulAssign for f128 {
  #[inline]
  fn mul_assign(&mut self, rhs:f128) {
    *self = *self * rhs;
  }
}
impl DivAssign for f128 {
  #[inline]
  fn div_assign(&mut self, rhs:f128) {
    *self = *self / rhs;
  }
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
fn split128(x:u128) -> (u64, u64) {
  let hi = (x>>64) as u64;
  let lo = x as u64;
  (hi, lo)
}

fn mul2(a:u128, b:u128) -> (u128, u128) {
  let (ah, al) = split128(a);
  let (ah, al) = (ah as u128, al as u128);
  let (bh, bl) = split128(b);
  let (bh, bl) = (bh as u128, bl as u128);

  let ahbh = ah * bh;
  let ahbl = ah * bl;
  let albh = al * bh;
  let albl = al * bl;

  let (mid, c) = ahbl.overflowing_add(albh);
  let (midh, midl) = split128(mid);
  let (midh, midl) = (midh as u128, midl as u128);
  let hi = ahbh + (if c {1<<64} else {0});
  let (lo,c) = albl.overflowing_add(midl<<64);
  let hi = hi + midh + (if c {1} else {0});
  (hi, lo)
}

#[inline]
fn shr128(a:u128, b:u128, n:u32) -> (u128,u128) {
  (a>>n, (b>>n)|(a<<(128-n)))
}

#[inline] fn sign(x:u128) -> bool { signb(x) != 0 }
#[inline] fn signb(x:u128) -> u128 { x & SGNB }
#[inline] fn exp(x:u128) -> i32 { (rawexp(x) as i32) - BIAS }
#[inline] fn rawexp(x:u128) -> u32 { ((x & EXPB) >> SHIFT) as u32 }
#[inline] fn rawman(x:u128) -> u128 { x & MANB }
#[inline] fn man(x:u128) -> u128 { if rawexp(x) == 0 { rawman(x) } else { rawman(x) | IMPB } }

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
  if e + (15-z) + BIAS < 0 {
    // TODO: subnormals
    return s;
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

// shifts right n-3 bits, keeping sticky lsb
#[inline]
fn shr3(x:u128, n:u32) -> u128 {
  if n >= 128+3 {
    if x==0 {0} else {1}
  } else if n <= 3 {
    x << (3 - n)
  } else {
    let lob = x & ((1 << (n - 3))-1);
    let s = if lob==0 {0} else {1};
    (x >> (n - 3)) | s
  }
}

#[inline] pub fn neg(x:u128) -> u128 { x ^ SGNB }

#[inline]
pub fn add(x:u128,y:u128) -> u128 {
  // TODO: cleanup the special cases
  if f128(y).is_zero() { return x; }
  if f128(x).is_zero() { return y; }
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
  // TODO: subnormals
  let s = sign(x);
  let ex = exp(x);
  let ey = exp(y);
  let e = ex.max(ey);
  let mx = shr3(man(x), (e-ex) as u32);
  let my = shr3(man(y), (e-ey) as u32);
  let m = mx + my;
  let m = round(m) >> 3;
  build(s, e, m)
}

#[inline]
pub fn sub(x:u128,y:u128) -> u128 {
  // TODO: cleanup the special cases
  if f128(y).is_zero(){ return x; }
  if f128(x).is_zero(){ return neg(y); }
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

  // TODO: subnormals
  if x>y {
    let s = sign(x);
    let ex = exp(x);
    let ey = exp(y);
    let e = ex.max(ey);
    let mx = shr3(man(x), 0);
    let my = shr3(man(y), (e-ey) as u32);
    let m = mx - my;
    let m = round(m) >> 3;
    build(s, e, m)
  } else if x<y {
    let s = !sign(y);
    let ex = exp(x);
    let ey = exp(y);
    let e = ex.max(ey);
    let mx = shr3(man(x), (e-ex) as u32);
    let my = shr3(man(y), 0);
    let m = my - mx;
    let m = round(m) >> 3;
    build(s, e, m)
  } else {
    0
  }
}

#[inline]
pub fn mul(x:u128,y:u128) -> u128 {
  let s = sign(x) ^ sign(y);

  // TODO: cleanup the special cases
  if f128(x).is_nan() { return x; }
  if f128(y).is_nan() { return y; }
  if f128(x).is_zero() {
    if f128(y).is_infinite() {
      return (if s {NEGNAN.0} else {NAN.0});
    } else {
      return (if s {SGNB} else {0});
    }
  }
  if f128(y).is_zero() {
    if f128(x).is_infinite() {
      return (if s {NEGNAN.0} else {NAN.0});
    } else {
      return (if s {SGNB} else {0});
    }
  }
  if f128(x).is_infinite() { return (if s {neg(x)} else {x}); }
  if f128(y).is_infinite() { return (if s {neg(y)} else {y}); }

  let mut e = exp(x) + exp(y) - 112;
  let (mh,ml) = mul2(man(x), man(y));
  let m;
  if mh == 0 {
    let z = ml.leading_zeros();
    if z < 15-3 {
      let n = 128 + (15-z) - 3;
      e += (n as i32) + 3;
      m = shr3(ml, 15-z);
    } else {
      let n = 128 + (15-z) - 3;
      e += (n as i32) + 3;
      m = ml << (z-15+3);
    }
  } else {
    let z = mh.leading_zeros();
    let n = (128 - z) + 15 - 3;
    e += (n as i32) + 3;
    (_,m) = shr128(mh,ml,n); // TODO: sticky bit!
  }
  let m = round(m) >> 3;
  build(s, e, m)
}

#[inline]
// uses iteration:
//  e = 1 - a*x
//  f = e*e + e
//  x' = f*x + x
pub fn recip(x:u128) -> u128 {
  if f128(x).is_nan() { return x; }
  if f128(x).is_zero() { return (x&SGNB)|INFINITY.0; }
  if f128(x).is_infinite() { return (x&SGNB); }
  // TODO: subnormal
  let s = sign(x);
  let me = -exp(x);
  let x0 = (x&MANB)|(16383<<112);
  let f = f64::from(f128(x0)).recip();
  let z = f128::from(f).0;
  let e = sub(ONE.0, mul(x0, z));
  let f = add(mul(e, e), e);
  let z = add(mul(f, z), z);
  z.wrapping_add((me as u128) << 112) | (if s {SGNB} else {0})
}

