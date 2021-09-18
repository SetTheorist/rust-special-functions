use std::ops::{Add,Sub,Mul,Div,Neg,AddAssign,SubAssign,MulAssign,DivAssign};

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)] // Eq,Ord
pub struct Quad(f64, f64);

// requires |a|>=|b|
#[inline]
fn qtsum(a:f64, b:f64) -> (f64,f64) {
  let s = a+b;
  let e = b+(a-s); // = b-(s-a)
  (s,e)
}

// general
#[inline]
fn ddsum(a:f64, b:f64) -> (f64,f64) {
  let s = a+b;
  let v = s-a;
  let e = (a+(v-s))+(b-v); // = (a-(s-v))+(b-v)
  (s,e)
}

#[inline]
fn split(a:f64) -> (f64,f64) {
  let t = 134217729.0*a;
  let ahi = t - (t - a);
  let alo = a - ahi;
  (ahi,alo)
}

#[inline]
fn ddprod(a:f64, b:f64) -> (f64,f64) {
  let (ahi,alo) = split(a);
  let (bhi,blo) = split(b);
  let p = a*b;
  let e = (((ahi*bhi - p) + ahi*blo) + alo*bhi) + alo*blo;
  (p,e)
}

#[inline]
fn qdadd(Quad(xhi,xlo):Quad, y:f64) -> Quad {
  let (shi,slo) = ddsum(y  , xhi      );
  let (hhi,hlo) = qtsum(shi, slo + xlo);
  let ( hi, lo) = qtsum(hhi, hlo      );
  Quad(hi,lo)
}

#[inline]
fn dqadd(x:f64, y:Quad) -> Quad { qdadd(y, x) }

#[inline]
fn qqadd(Quad(xhi,xlo):Quad, Quad(yhi,ylo):Quad) -> Quad {
  let (hs,he) = ddsum(xhi, yhi);
  let (ls,le) = ddsum(xlo, ylo);
  let (h ,k ) = qtsum(hs , he+ls);
  let (hi,lo) = qtsum(h  , le+k);
  Quad(hi,lo)
}

#[inline]
fn qnegate(Quad(hi,lo):Quad) -> Quad { Quad(-hi,-lo) }

#[inline]
fn qdprod(Quad(xhi,xlo):Quad, y:f64) -> Quad {
  let (thi,tlo) = ddprod(xhi, y        );
  let ( hi, lo) = qtsum( thi, tlo+y*xlo);
  Quad(hi,lo)
}

#[inline]
fn dqprod(x:f64, y:Quad) -> Quad { qdprod(y, x) }

#[inline]
fn qqprod(Quad(xhi,xlo):Quad, Quad(yhi,ylo):Quad) -> Quad {
  let (p ,e ) = ddprod(xhi, yhi);
  let (hi,lo) = qtsum(p, e + (xhi*ylo + xlo*yhi));
  Quad(hi,lo)
}

#[inline]
fn qqdivide(Quad(xhi,xlo):Quad, Quad(yhi,ylo):Quad) -> Quad {
  let cc = xhi / yhi;
  let (uu,u) = ddprod(cc, yhi);
  let c = ((((xhi-uu)-u)+xlo)-cc*ylo)/yhi;
  let (hi,lo) = qtsum(cc, c);
  Quad(hi,lo)
}

#[inline]
fn dqdivide(x:f64, Quad(yhi,ylo):Quad) -> Quad {
  let cc = x / yhi;
  let (uu,u) = ddprod(cc, yhi);
  let c = ((((x-uu)-u))-cc*ylo)/yhi;
  let (hi,lo) = qtsum(cc, c);
  Quad(hi,lo)
}

#[inline]
fn qddivide(Quad(xhi,xlo):Quad, y:f64) -> Quad {
  let xdy = xhi / y;
  let (uu,u) = ddprod(xdy, y);
  let c = (((xhi-uu)-u)+xlo)/y;
  let (hi,lo) = qtsum(xdy, c);
  Quad(hi,lo)
}

impl Quad {
  // construction
  #[inline]
  pub fn new(a:f64, b:f64) -> Quad {
    let (hi,lo) = ddsum(a, b);
    Quad(hi,lo)
  }

  // deconstruction
  #[inline]
  pub fn parts(Quad(hi,lo):Self) -> (f64,f64) { (hi,lo) }
  #[inline]
  pub fn hi(self) -> f64 { self.0 }
  #[inline]
  pub fn lo(self) -> f64 { self.1 }

  // misc

  #[inline]
  pub fn abs(self) -> Quad {
    if self.0<0.0 { -self } else { self }
  }

  #[inline]
  pub fn scale2(self, i:isize) -> Quad {
    Quad(libm::ldexp(self.0, i as i32),libm::ldexp(self.1, i as i32))
  }

  pub fn scale10(self, i:isize) -> Quad {
    if i<0 {
      let mut q = self;
      for _ in 0..(-i) { q = q / 10.0; }
      q
    } else if i>0 {
      let mut q = self;
      for _ in 0..i { q = q * 10.0; }
      q
    } else {
      self
    }
  }
}

impl Add<Quad> for Quad { type Output=Quad; fn add(self,y:Quad) -> Quad { qqadd(self, y) } }
impl Sub<Quad> for Quad { type Output=Quad; fn sub(self,y:Quad) -> Quad { qqadd(self, -y) } }
impl Mul<Quad> for Quad { type Output=Quad; fn mul(self,y:Quad) -> Quad { qqprod(self, y) } }
impl Div<Quad> for Quad { type Output=Quad; fn div(self,y:Quad) -> Quad { qqdivide(self, y) } }
impl Neg for Quad { type Output=Quad; fn neg(self) -> Quad { qnegate(self) } }

impl Add<f64> for Quad { type Output=Quad; fn add(self,y:f64) -> Quad { qdadd(self, y) } }
impl Sub<f64> for Quad { type Output=Quad; fn sub(self,y:f64) -> Quad { qdadd(self, -y) } }
impl Mul<f64> for Quad { type Output=Quad; fn mul(self,y:f64) -> Quad { qdprod(self, y) } }
impl Div<f64> for Quad { type Output=Quad; fn div(self,y:f64) -> Quad { qddivide(self, y) } }

impl Add<Quad> for f64 { type Output=Quad; fn add(self,y:Quad) -> Quad { dqadd(self, y) } }
impl Sub<Quad> for f64 { type Output=Quad; fn sub(self,y:Quad) -> Quad { dqadd(self, -y) } }
impl Mul<Quad> for f64 { type Output=Quad; fn mul(self,y:Quad) -> Quad { dqprod(self, y) } }
impl Div<Quad> for f64 { type Output=Quad; fn div(self,y:Quad) -> Quad { dqdivide(self, y) } }

impl AddAssign<Quad> for Quad { fn add_assign(&mut self,y:Quad) { *self = qqadd(*self, y); } }
impl SubAssign<Quad> for Quad { fn sub_assign(&mut self,y:Quad) { *self = qqadd(*self, -y); } }
impl MulAssign<Quad> for Quad { fn mul_assign(&mut self,y:Quad) { *self = qqprod(*self, y); } }
impl DivAssign<Quad> for Quad { fn div_assign(&mut self,y:Quad) { *self = qqdivide(*self, y); } }

impl AddAssign<f64> for Quad { fn add_assign(&mut self,y:f64) { *self = qdadd(*self, y); } }
impl SubAssign<f64> for Quad { fn sub_assign(&mut self,y:f64) { *self = qdadd(*self, -y); } }
impl MulAssign<f64> for Quad { fn mul_assign(&mut self,y:f64) { *self = qdprod(*self, y); } }
impl DivAssign<f64> for Quad { fn div_assign(&mut self,y:f64) { *self = qddivide(*self, y); } }

pub fn stoq(s:&str) -> Quad {
  let mut neg = false;
  let mut dec = false;
  let mut e = 0;
  let mut q = Quad(0.0,0.0);
  for c in s.chars() {
    match c {
      '-' => { neg = true; }
      '+' => { }
      '.' => { dec = true; }
      //'e' => { }
      d => {
        let v = ((d as u8) - ('0' as u8)) as f64;
        q = q*10.0 + v;
        if dec { e-=1; }
      }
    }
  }
  q = q.scale10(e);
  if neg { -q } else { q }
}

impl std::fmt::Display for Quad {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f,  "ξ")?;
    if self.0==0.0 { return write!(f,  "0.0"); }
    let mut q = *self;
    if q.0<0.0 { q=-q; write!(f, "-")?; }

    let mut e = 0;
    while q.0>=10.0 {
      e += 1;
      q /= 10.0;
    }
    while q.0<1.0 {
      e -= 1;
      q *= 10.0;
    }

    // TODO: there seems to be a bug here, but it may be a symptom
    // of a bug above (perhaps in qdmul()?)
    // with floor() instead of trunc() this displays invalid characters
    // and with trunc() it is not displaying all significant digits
    // HMMM: probably an issue with negative LO part moving to significance!?
    for n in 0..33 {
      //dbg!(q);
      if n==1 { write!(f, ".")?; }
      //let d = q.0.trunc();
      let d = q.0.floor();
      let dd = ((d as u8) + ('0' as u8)) as char;
      write!(f, "{}", dd)?;
      q = (q - d)*10.0;
    }

    if e!=0 { write!(f, "e{}", e)?; }
    write!(f, "")
  }
}

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
