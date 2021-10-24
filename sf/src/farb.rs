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
  Positive,
  Negative,
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

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct I<const N:usize>(pub [u32; N]);

impl<const N:usize> std::fmt::Debug for I<N> {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[")?;
    for i in (0..N).rev() {
      write!(f, " {:08x}", self.0[i])?;
    }
    write!(f, " ]")
  }
}

////////////////////////////////////////////////////////////////////////////////

// TODO: more efficient layout, etc.
#[derive(Clone,Copy,Debug)]
pub struct farb<const N:usize> {
  pub t:Type,
  pub s:Sign,
  pub e:i32,
  pub m:I<N>,
}

impl<const N:usize> farb<N> {
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

// normalize x to have msb=1 and rounded correctly (round-to-nearest, ties-to-even)
// returns exponent adjustment
pub fn normalize<const N:usize>(x:&mut I<N>, c:bool, low:u32) -> i32 {
  println!(">> {:?}", x);
  let mut ex = 0;
  let mut low = low;
  if c {
    // shift 1 bit right
    low = (low>>1)|(low&1)|(x.0[0]<<31);
    for i in (0..(N-1)).rev() {
      x.0[i] = (x.0[i]>>1) | (x.0[i+1]<<31);
    }
    x.0[N-1] = (x.0[N-1]>>1) | 0x8000_0000;
    ex = 1;
  } else {
    let mut n = 0;
    for i in (0..N).rev() {
      let z = x.0[i].leading_zeros();
      n += z;
      if z != 0 { break; }
    }
    shl(x, n as u32);
  }
  println!("<< {:?}", x);
  todo!()
}

// adds in-place to x, returns true if carry
pub fn add_mm<const N:usize>(x:&mut I<N>, y:&I<N>) -> bool {
  let mut c = false;
  for i in 0..N {
    (x.0[i], c) = x.0[i].carrying_add(y.0[i], c);
  }
  c
}

// shifts-left in-place, discarding anything shifted off
pub fn shl<const N:usize>(x:&mut I<N>, n:u32) -> () {
  if n == 0 { return; }
  let nw = (n / 32) as usize;
  let nb = (n % 32) as usize;
  if nw >= N {
    for i in 0..N {
      x.0[i] = 0;
    }
  } else {
    for i in (1..(N-nw)).rev() {
      x.0[i+nw] = (x.0[i] << nb) | (x.0[i-1] >> (32-nb));
    }
    x.0[nw] = (x.0[0] << nb);
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
      x.0[i] = (x.0[i+nw] >> nb) | (if i+nw<N-1 {(x.0[i+nw+1] << (32-nb))} else {0});
    }
    for i in (N-nw)..N {
      x.0[i] = 0;
    }
    return over;
  }
}

////////////////////////////////////////////////////////////////////////////////

impl<const N:usize> Add<farb<N>> for farb<N> {
  type Output = farb<N>;
  fn add(mut self, mut rhs:farb<N>) -> farb<N> {
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
