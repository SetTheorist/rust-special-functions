use std::ops::{Add,Sub,Mul,Div,Neg,Rem};

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub enum Type {
  Normal,
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

impl Default for Sign {
  #[inline] fn default() -> Self { Sign::Positive }
}

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct I<const N:usize>(pub [u32; N]);

impl<const N:usize> std::fmt::Debug for I<N> {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[")?;
    for i in (0..N).rev() {
      write!(f, " {:08x}", self.0[i]);
    }
    write!(f, " ]")
  }
}

// TODO: more efficient layout, etc.
#[derive(Clone,Copy,Debug)]
pub struct farb<const N:usize> {
  pub t:Type,
  pub s:Sign,
  pub e:i32,
  pub m:I<N>,
}

pub fn add_mm<const N:usize>(x:&I<N>, y:&I<N>) -> (I<N>, bool) {
  let mut res = I([0;N]);
  let mut c = false;
  for i in 0..N {
    (res.0[i], c) = x.0[i].carrying_add(y.0[i], c);
  }
  (res, c)
}

pub fn add_mi<const N:usize>(x:&I<N>, y:u32) -> (I<N>, bool) {
  let mut res = I([0;N]);
  let mut c;
  (res.0[0], c) = x.0[0].overflowing_add(y);
  if !c { return (res, false); }
  for i in 1..N {
    (res.0[i], c) = x.0[i].overflowing_add(1);
    if !c { return (res, false); }
  }
  (res, true)
}

// shifts right, extra word for carry, including "sticky" bit in lsb
// TODO: reduce to 3 MSB with sticky, rather than entire word(?)
pub fn shr<const N:usize>(x:&I<N>, a:u32) -> (I<N>, u32) {
  let mut res = I([0;N]);
  let nw = (a/32) as usize;
  let nb = (a%32);
  // copy low-order bits
  if nw <= N {
    for i in 0..(N-nw) {
      res.0[i] = x.0[i+nw] >> nb;
    }
  }
  // merge hi-order bits
  if nb > 0 && nw < N {
    for i in 0..(N-nw-1) {
      res.0[i] |= x.0[i+nw+1] << (32-nb);
    }
  }
  let over =
    (if nw > 0 && nw <= N { x.0[nw-1] } else { 0 })
    | (if nw < N && nb > 0 { x.0[nw] << (32-nb) } else { 0 });

  // TODO: this is too much work for sticky bit:
  // we just need to know if it's set or zero...
  let mut sticky = (if nw > 0 && nw <= N && nb > 0 { x.0[nw-1] << (32-nb) } else { 0 });
  if nw > 0 {
    for i in 0..(nw-1).min(N) {
      sticky |= x.0[i];
    }
  }
  (res, over|(if sticky == 0 {0} else {1}))
}

impl<const N:usize> Add for farb<N> {
  type Output = farb<N>;
  fn add(self, rhs:farb<N>) -> farb<N> {
    // TODO: special cases (handle types, etc.)
    // TODO: check signs, etc.
    if self.e == rhs.e {
      let t = self.t;
      let s = self.s;
      let mut e = self.e;
      let (mut m, c) = add_mm(&self.m, &rhs.m);
      if c {
        e += 1;
        let (mx, o) = shr(&m, 1);
        m = mx;
        m.0[N-1] |= 0x8000_0000;
        // TODO: rounding
      }
      farb { t, s, e, m }
    } else if self.e > rhs.e {
      todo!()
    } else {
      todo!()
    }
  }
}

