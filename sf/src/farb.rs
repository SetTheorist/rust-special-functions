use std::cmp::{Ord,Ordering,PartialOrd};
use std::ops::{Add,Sub,Mul,Div,Neg,Rem};

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub enum Type {
  Normal,
  Zero,
  Nan,
  Infinite,
}

impl Default for Type {
  #[inline] fn default() -> Self { Type::Normal }
}

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub enum Sign {
  Negative,
  Positive,
}

impl Neg for Sign {
  type Output = Sign;
  fn neg(self) -> Sign {
    match self {
      Sign::Positive => Sign::Negative,
      Sign::Negative => Sign::Positive,
    }
  }
}

impl Default for Sign {
  #[inline] fn default() -> Self { Sign::Positive }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,PartialEq)]
pub struct I<const N:usize>(pub [u32; N]);

impl<const N:usize> I<N> {
  #[inline]
  fn leading_zeros(&self) -> u32 {
    let mut z = 0;
    for i in (0..N).rev() {
      let c = self.0[i].leading_zeros();
      z += c;
      if c!=32 {break;}
    }
    z
  }
}

impl<const N:usize> std::fmt::Debug for I<N> {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[")?;
    for i in (0..N).rev() {
      //write!(f, "{}{:08x}", (if(i==N-1){""}else{"_"}), self.0[i])?;
      write!(f, "{:08x}", self.0[i])?;
    }
    write!(f, "]")
  }
}

////////////////////////////////////////////////////////////////////////////////

// TODO: more efficient layout, etc.
#[derive(Clone,Copy,Debug, PartialEq)]
pub struct farb<const N:usize> {
  pub t:Type,
  pub s:Sign,
  pub e:i32,
  pub m:I<N>,
}

impl<const N:usize> farb<N> {
  pub fn ldexp(self, n:i32) -> farb<N> {
    // TODO: types, underflow/overflow
    let t = self.t;
    let s = self.s;
    let e = self.e + n;
    let m = self.m;
    farb { t, s, e, m }
  }
}

impl<const N:usize> farb<N>
  where [u32;2*N] : Sized
{
  // TODO: use iteration:
  //  e = 1 - a*x
  //  f = e*e + e
  //  x' = f*x + x
  // uses: x' = x*(2-a*x)
  pub fn recip(self) -> farb<N> {
    // TODO: types, etc.
    let mut x = Self::from(f64::from(self).recip());
    let t = Self::from(2.0_f32);
    for _ in 0 .. (N.log2()+1) {
      x = x*(t - self*x);
    }
    x
  }

  pub fn sqrt(self) -> farb<N> {
    let mut z = Self::from(f64::from(self).sqrt());
    for _ in 0 .. (N.log2()+1) {
      z = (z + self*z.recip()).ldexp(-1);
    }
    z
  }
}

// f16:  1: 5:(1): 10; bias 15, -14/+15
// f32:  1: 8:(1): 23; bias 127, -126/+127
// f64:  1:11:(1): 52; bias 1023, -1022/+1023
// f128: 1:15:(1):112; bias 16383, -16382/16383
// f256: 1:19:(1):236; bias 262143, -262142/+262143

impl<const N:usize> From<f32> for farb<N> {
  fn from(x:f32) -> Self {
    let b = x.to_bits();
    // TODO: non-normals
    let t = Type::Normal;
    let s = if b&(1<<31)==0 {Sign::Positive} else {Sign::Negative};
    let e = (((b >> 23) & 0xFF) as i32) - 127;
    let m0 = (b & 0x007F_FFFF) | 0x0080_0000;
    let mut m = I([0; N]);
    m.0[N-1] = m0 << 8;
    farb { t, s, e, m }
  }
}
impl<const N:usize> From<farb<N>> for f32 {
  fn from(x:farb<N>) -> f32 {
    let s = if x.s==Sign::Positive {0} else {1<<31};
    if x.t == Type::Zero {
      return f32::from_bits(s);
    }
    if x.t == Type::Infinite {
      return f32::from_bits(s|0x7F80_0000); // infinity
    }
    if x.e < -126 {
      // TODO: subnormals
      return f32::from_bits(s); // zero
    } else if x.e > 127 {
      return f32::from_bits(s|0x7F80_0000); // infinity
    }
    let e = ((x.e + 127) as u32) << 23;
    // TODO: rounding
    let m = (x.m.0[N-1] >> 8) & 0x007F_FFFF;
    f32::from_bits(s|e|m)
  }
}
impl<const N:usize> From<f64> for farb<N> {
  fn from(x:f64) -> Self {
    let b = x.to_bits();
    // TODO: non-normals, etc.
    let t = Type::Normal;
    let s = if b&(1<<63)==0 {Sign::Positive} else {Sign::Negative};
    let e = (((b >> 52) & 0x7FF) as i32) - 1023;
    let m0 = (b & 0x000F_FFFF__FFFF_FFFF) | 0x0010_0000__0000_0000;
    let mut m = I([0; N]);
    m.0[N-1] = ((m0 << 11) >> 32) as u32;
    if N>=2 {
      m.0[N-2] = (m0 << 11) as u32;
    } else {
      // ROUNDING...
    }
    farb { t, s, e, m }
  }
}
impl<const N:usize> From<farb<N>> for f64 {
  fn from(x:farb<N>) -> f64 {
    let s = if x.s==Sign::Positive {0} else {1<<63};
    if x.t == Type::Zero {
      return f64::from_bits(s);
    }
    if x.t == Type::Infinite {
      return f64::from_bits(s|0x7FF0_0000__0000_0000); // infinity
    }
    if x.e < -1022 {
      // TODO: subnormals
      return f64::from_bits(s); // zero
    } else if x.e > 1023 {
      return f64::from_bits(s|0x7FF0_0000__0000_0000); // infinity
    }
    let e = ((x.e + 1023) as u64) << 52;
    // TODO: rounding
    let mut m0 = ((x.m.0[N-1] as u64) << 21);
    if N>=2 {
      m0 |= (x.m.0[N-2] as u64) >> 11;
    }
    let m = m0 & 0x000F_FFFF__FFFF_FFFF;
    f64::from_bits(s|e|m)
  }
}

////////////////////////////////////////////////////////////////////////////////

impl<const N:usize> std::fmt::Display for farb<N>
  where [u32;2*N]:Sized
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut z = *self;
    if z < Self::from(0.0) {
      z = -z;
      write!(f, "-")?;
    }
    let one = Self::from(1.0);
    let ten = Self::from(10.0);
    let tenth = ten.recip();
    let mut e = 0;
    while z >= ten {
      e += 1;
      z = z * tenth;
    }
    while z < one {
      e -= 1;
      z = z * ten;
    }
    for n in 0..(N*10) {
      if n == 1 {
        write!(f, ".")?;
      }
      let d = f64::from(z).floor();
      if d<0.0 || d>=10.0 { print!("<<>>"); }
      let dd = ((d as u8) + b'0') as char;
      write!(f, "{}", dd)?;
      z = (z - Self::from(d)) * ten;
    }
    if e != 0 { write!(f, "e{}", e)?; }
    write!(f, "")
  }
}

////////////////////////////////////////////////////////////////////////////////

// normalize x to have msb=1 and rounded correctly (round-to-nearest, ties-to-even)
// returns exponent adjustment
pub fn normalize<const N:usize>(x:&mut I<N>, c:bool, low:u32) -> i32 {
  let mut ex : i32 = 0;
  let mut low = low;

  // normalize to normal form
  if c {
    // shift 1 bit right
    let o = shr(x, 1);
    x.0[N-1] |= 0x8000_0000;
    low = (low>>1) | (low&1) | o;
    ex += 1;
  } else {
    let n = x.leading_zeros();
    shl(x, &mut low, n as u32);
    ex -= n as i32;
  }
  // rounding (round-to-nearest, ties-to-even)
  if low > 0x8000_0000
    || (low == 0x8000_0000 && (x.0[0]&1 == 1)) {
    let c = add_mi(x, 1);
    if c {
      let o = shr(x, 1);
      x.0[N-1] |= 0x8000_0000;
      //low = (low>>1) | (low&1) | o;
      ex += 1;
    }
  }
  ex
}

// adds in-place to x, returns true if carry
pub fn add_mm<const N:usize>(x:&mut I<N>, y:&I<N>) -> bool {
  let mut c = false;
  for i in 0..N {
    (x.0[i], c) = x.0[i].carrying_add(y.0[i], c);
  }
  c
}

// subtracts in-place to x, returns true if borrow
pub fn sub_mm<const N:usize>(x:&mut I<N>, y:&I<N>) -> bool {
  let mut c = false;
  for i in 0..N {
    (x.0[i], c) = x.0[i].borrowing_sub(y.0[i], c);
  }
  c
}

// adds in-place to x, returns true if carry
pub fn add_mi<const N:usize>(x:&mut I<N>, y:u32) -> bool {
  let mut c;
  (x.0[0], c) = x.0[0].overflowing_add(1);
  for i in 1..N {
    if !c { break; }
    (x.0[i], c) = x.0[i].overflowing_add(1);
  }
  c
}

// multiplies y in-place to x, returns carry word
pub fn mul_mi<const N:usize>(x:&mut I<N>, y:u32) -> u32 {
  let mut c = 0;
  for i in 0..N {
    (x.0[i], c) = x.0[i].carrying_mul(y, c);
  }
  c
}

// multiplies x,y into res
pub fn mul_mm<const N:usize>(res:&mut I<{2*N}>, x:&I<N>, y:&I<N>) 
  where [u32;2*N]:Sized
{
  for i in 0..N {
    let mut c = 0;
    for j in 0..N {
      let xy;
      (xy, c) = x.0[j].carrying_mul(y.0[i], c);
      let c2;
      (res.0[i+j], c2) = res.0[i+j].overflowing_add(xy);
      if c2 {c+=1;}
    }
    res.0[i+N] += c;
  }
}

// shifts-left in-place, discarding anything shifted off
// shifts extra low word in, except for lsb (sticky) bit
pub fn shl<const N:usize>(x:&mut I<N>, low:&mut u32, n:u32) -> () {
  if n == 0 { return; }
  let nw = (n / 32) as usize;
  let nb = (n % 32) as usize;
  if nw >= N {
    // entire value shifted away
    for i in 0..N {
      x.0[i] = 0;
    }
  } else {
    // normal case
    for i in (1..(N-nw)).rev() {
      x.0[i+nw] = (x.0[i] << nb) | (if nb>0 {x.0[i-1] >> (32-nb)} else {0});
    }
    x.0[nw] = (x.0[0] << nb);
    for i in 0..nw {
      x.0[i] = 0;
    }
  }
  // deal with extra low word
  if *low != 0 {
    if nw <= N {
      if nb > 0 {
        x.0[nw] |= (*low&!1)>>(32-nb);
      }
      if nw > 0 {
        x.0[nw-1] |= (*low&!1)<<nb;
      }
      if nw >= 1 {
        *low &= 1;
      } else if nw == 0 {
        *low = (*low<<nb) | (*low&1);
      }
    }
  }
}

// shifts-right in-place, returning extra word with sticky lsb
pub fn shr<const N:usize>(x:&mut I<N>, n:u32) -> u32 {
  if n == 0 { return 0; }
  let nw = (n / 32) as usize;
  let nb = (n % 32) as usize;
  if nw > N {
    // get sticky bit and clear
    let mut sticky = 0;
    for i in 0..N {
      if x.0[i] != 0 {
        sticky = 1;
      }
      x.0[i] = 0;
    }
    return sticky;
  } else if nw == N {
    todo!()
  } else {
    let mut over = 0;
    // sticky bit
    if nw > 0 {
      for i in 0..(nw-1) {
        if x.0[i] != 0 {
          over |= 1;
          break;
        }
      }
      if nb > 0 {
        if x.0[nw-1] << (32-nb) != 0 {
          over |= 1;
        }
      }
    }
    // overflow
    if nw > 0 { over |= (x.0[nw-1]>>nb); }
    if nb > 0 { over |= x.0[nw] << (32-nb); }
    // normal shift
    for i in (0..(N-nw)) {
      x.0[i] = (x.0[i+nw] >> nb) | (if i+nw<N-1 && nb>0 {(x.0[i+nw+1] << (32-nb))} else {0});
    }
    for i in (N-nw)..N {
      x.0[i] = 0;
    }
    return over;
  }
}

////////////////////////////////////////////////////////////////////////////////

/*
impl<const N:usize> PartialEq<farb<N>> for farb<N> {
  fn eq(&self, rhs:&farb<N>) -> bool {
    // TODO!
  }
}
*/

impl<const N:usize> PartialOrd<farb<N>> for farb<N> {
  fn partial_cmp(&self, rhs:&farb<N>) -> Option<Ordering> {
    // type
    if self.t != Type::Normal || rhs.t != Type::Normal {
      // TODO!
      return None;
    }
    // sign
    if self.s < rhs.s {
      return Some(Ordering::Less);
    } else if self.s > rhs.s {
      return Some(Ordering::Greater);
    }
    // exponent
    if self.e < rhs.e {
      return if self.s == Sign::Positive {Some(Ordering::Less)}
        else {Some(Ordering::Greater)};
    } else if self.e > rhs.e {
      return if self.s == Sign::Positive {Some(Ordering::Greater)}
        else {Some(Ordering::Less)};
    }
    // mantissa
    for i in (0..N).rev() {
      if self.m.0[i] > rhs.m.0[i] {
        return Some(Ordering::Greater);
      } else if self.m.0[i] < rhs.m.0[i] {
        return Some(Ordering::Less);
      }
    }
    return Some(Ordering::Equal);
  }
}

impl<const N:usize> Neg for farb<N> {
  type Output = farb<N>;
  fn neg(self) -> farb<N> {
    let t = self.t;
    let s = -self.s;
    let e = self.e;
    let m = self.m;
    farb { t, s, e, m }
  }
}

impl<const N:usize> Add<farb<N>> for farb<N> {
  type Output = farb<N>;
  fn add(mut self, mut rhs:farb<N>) -> farb<N> {
    if self.s != rhs.s { return self - (-rhs); }
    // TODO: special cases...
    if self.e >= rhs.e {
      let low = shr(&mut rhs.m, (self.e - rhs.e) as u32);
      let c = add_mm(&mut self.m, &rhs.m);
      let ex = normalize(&mut self.m, c, low);
      let t = self.t;
      let s = self.s;
      let e = self.e + ex;
      let m = self.m;
      farb { t, s, e, m }
    } else {
       rhs.add(self)
    }
  }
}

impl<const N:usize> Sub<farb<N>> for farb<N> {
  type Output = farb<N>;
  fn sub(mut self, mut rhs:farb<N>) -> farb<N> {
    if self.s != rhs.s { return self + (-rhs); }
    // TODO: special cases...
    if self.e >= rhs.e {
      let low = shr(&mut rhs.m, (self.e - rhs.e) as u32);
      let c = sub_mm(&mut self.m, &rhs.m);
      let ex = normalize(&mut self.m, c, low);
      let t = self.t;
      let s = if c {-self.s} else {self.s};
      let e = self.e + ex;
      let m = self.m;
      farb { t, s, e, m }
    } else {
       -(rhs.sub(self))
    }
  }
}

impl<const N:usize> Mul<farb<N>> for farb<N>
  where [u32;2*N] : Sized
{
  type Output = farb<N>;
  fn mul(self, rhs:farb<N>) -> farb<N> {
    // TODO: types; zero, other specials; etc.
    let mut mx = I([0; 2*N]);
    mul_mm(&mut mx, &self.m, &rhs.m);
    let t = self.t;
    let s = if self.s==rhs.s {Sign::Positive} else {Sign::Negative};
    let mut e = self.e + rhs.e;
    let mut m = I([0; N]);
    let z = mx.leading_zeros();
    if z < (N*32) as u32 {
      let low  = shr(&mut mx, (N as u32 * 32) - z);
      m.0.copy_from_slice(&mx.0[0..N]);
      let ex = normalize(&mut m, false, low);
      e += ex + ((N as i32 * 32) - (z as i32) - ((N as i32)*32-1));
    } else if z > (N*32) as u32 {
      if z == (2*N*32) as u32 {
        return farb { t:Type::Zero, s, e:0, m:I([0;N]) };
      }
      todo!()
    } else {
      m.0.copy_from_slice(&mx.0[0..N]);
    }
    farb { t, s, e, m }
  }
}

impl<const N:usize> Mul<farb<N>> for u32 {
  type Output = farb<N>;
  #[inline] fn mul(self, rhs:farb<N>) -> farb<N> { rhs*self }
}

impl<const N:usize> Mul<u32> for farb<N> {
  type Output = farb<N>;
  fn mul(mut self, rhs:u32) -> farb<N> {
    if rhs == 0 {
      return farb { t:Type::Zero, s:self.s, e:0, m:I([0;N]) };
    } else if rhs == 1 {
      return self;
    }
    // TODO: zero, subnormal, types, etc.
    // for normal, non-zero values, c is never zero
    let c = mul_mi(&mut self.m, rhs);
    let z = c.leading_zeros();
    let low = shr(&mut self.m, 32-z);
    if z<32 { self.m.0[N-1] |= c << z; }
    let ex = normalize(&mut self.m, false, low);
    let t = self.t;
    let s = self.s;
    let e = self.e + ex + ((32-z) as i32);
    let m = self.m;
    farb { t, s, e, m }
  }
}

impl<const N:usize> Div<farb<N>> for farb<N>
  where [u32;2*N] : Sized
{
  type Output = farb<N>;
  fn div(self, rhs:farb<N>) -> farb<N> {
    self * rhs.recip()
  }
}

