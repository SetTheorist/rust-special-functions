use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::traits::*;

use sf_hex_float::hexf;
macro_rules! Wide { ($x:tt) => { hexf!(:2:Wide:$x) } }

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)] // Eq,Ord
pub struct Wide(pub f64, pub f64);

impl From<r64> for Wide {
  fn from(x:r64) -> Wide { Wide(x.0, 0.0) }
}
impl From<f64> for Wide {
  fn from(x:f64) -> Wide { Wide(x, 0.0) }
}
impl From<isize> for Wide {
  fn from(x:isize) -> Wide { Wide(x as f64, 0.0) }
}

// requires |a|>=|b|
#[inline]
fn qtsum(a: f64, b: f64) -> (f64, f64) {
  let s = a + b;
  let e = b + (a - s); // = b-(s-a)
  (s, e)
}

// general
#[inline]
fn ddsum(a: f64, b: f64) -> (f64, f64) {
  let s = a + b;
  let v = s - a;
  let e = (a + (v - s)) + (b - v); // = (a-(s-v))+(b-v)
  (s, e)
}

#[inline]
fn split(a: f64) -> (f64, f64) {
  let t = 134217729.0 * a;
  let ahi = t - (t - a);
  let alo = a - ahi;
  (ahi, alo)
}

#[inline]
fn ddprod(a: f64, b: f64) -> (f64, f64) {
  let (ahi, alo) = split(a);
  let (bhi, blo) = split(b);
  let p = a * b;
  let e = (((ahi * bhi - p) + ahi * blo) + alo * bhi) + alo * blo;
  (p, e)
}

#[inline]
fn qdadd(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let (shi, slo) = ddsum(y, xhi);
  let (hhi, hlo) = qtsum(shi, slo + xlo);
  let (hi, lo) = qtsum(hhi, hlo);
  Wide(hi, lo)
}

#[inline]
fn dqadd(x: f64, y: Wide) -> Wide { qdadd(y, x) }

#[inline]
fn qqadd(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let (hs, he) = ddsum(xhi, yhi);
  let (ls, le) = ddsum(xlo, ylo);
  let (h, k) = qtsum(hs, he + ls);
  let (hi, lo) = qtsum(h, le + k);
  Wide(hi, lo)
}

#[inline]
fn qnegate(Wide(hi, lo): Wide) -> Wide { Wide(-hi, -lo) }

#[inline]
fn qdprod(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let (thi, tlo) = ddprod(xhi, y);
  let (hi, lo) = qtsum(thi, tlo + y * xlo);
  Wide(hi, lo)
}

#[inline]
fn dqprod(x: f64, y: Wide) -> Wide { qdprod(y, x) }

#[inline]
fn qqprod(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let (p, e) = ddprod(xhi, yhi);
  let (hi, lo) = qtsum(p, e + (xhi * ylo + xlo * yhi));
  Wide(hi, lo)
}

#[inline]
fn qqdivide(Wide(xhi, xlo): Wide, Wide(yhi, ylo): Wide) -> Wide {
  let cc = xhi / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = ((((xhi - uu) - u) + xlo) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Wide(hi, lo)
}

#[inline]
fn dqdivide(x: f64, Wide(yhi, ylo): Wide) -> Wide {
  let cc = x / yhi;
  let (uu, u) = ddprod(cc, yhi);
  let c = (((x - uu) - u) - cc * ylo) / yhi;
  let (hi, lo) = qtsum(cc, c);
  Wide(hi, lo)
}

#[inline]
fn qddivide(Wide(xhi, xlo): Wide, y: f64) -> Wide {
  let xdy = xhi / y;
  let (uu, u) = ddprod(xdy, y);
  let c = (((xhi - uu) - u) + xlo) / y;
  let (hi, lo) = qtsum(xdy, c);
  Wide(hi, lo)
}

impl Wide {
  // construction
  #[inline]
  pub fn new(a: f64, b: f64) -> Wide {
    let (hi, lo) = ddsum(a, b);
    Wide(hi, lo)
  }

  // deconstruction
  #[inline]
  pub fn parts(Wide(hi, lo): Self) -> (f64, f64) { (hi, lo) }
  #[inline]
  pub fn hi(self) -> f64 { self.0 }
  #[inline]
  pub fn lo(self) -> f64 { self.1 }
  
  // apply "correct" rounding to high part...
  // TODO: pub fn to_f64(self) -> f64 {}

  // misc

  #[inline]
  pub fn abs(self) -> Wide {
    if self.0 < 0.0 {
      -self
    } else {
      self
    }
  }

  // TODO: more efficient
  #[inline]
  pub fn sqr(self) -> Wide {
    self*self
  }

  pub fn sqrt(self) -> Wide {
    let q0 = self.0.sqrt();
    let x = Wide::new(q0, self.1/(q0*2.0));
    let x = (x+self/x)*0.5; // TODO: ldexp
    x
  }

  pub fn sqrt_recip(self) -> Wide {
    let q0 = self.0.sqrt().recip();
    let x = Wide::new(q0, -q0*self.1/(self.0*2.0));
    let x = x*(3 - self*x.sqr())*0.5; // TODO: ldexp
    x
  }

  pub fn cbrt(self) -> Wide {
    let q0 = self.0.cbrt();
    let x = Wide::new(q0, self.1/(q0*q0*3.0));
    let x = (x*2.0 + self/x.sqr())/3.0; // TODO: ldexp
    x
  }

  pub fn cbrt_recip(self) -> Wide {
    let q0 = self.0.cbrt().recip();
    let x = Wide::new(q0, -q0*self.1/(self.0*3.0));
    let x = x*(4.0-self*x.pow(3_isize))/3.0;
    let x = x*(4.0-self*x.pow(3_isize))/3.0;
    x
  }

  #[inline]
  pub fn recip(self) -> Wide {
    1.0 / self
  }

  pub fn nth_root(self, n:isize) -> Wide {
    let q0 = self.0.powf((n as f64).recip());
    let x = Wide::new(q0, q0/self.0*self.1/(n as f64));
    let x = (x*(n-1) + self/x.pow(n-1))/n;
    x
  }

  // TODO: hack implementation
  pub fn powf(self, e:Wide) -> Wide {
    (e * self.log()).exp()
  }

  #[inline]
  pub fn scale2(self, i: isize) -> Wide {
    // TODO: replace with ldexp() functionality
    Wide(self.0 * 2.0_f64.powi(i as i32), self.1 * 2.0_f64.powi(i as i32))
  }

  pub fn scale10(self, i: isize) -> Wide {
    if i < 0 {
      let mut q = self;
      for _ in 0..(-i) { q /= 10.0; }
      q
    } else if i > 0 {
      let mut q = self;
      for _ in 0..i { q *= 10.0; }
      q
    } else {
      self
    }
  }

  pub fn exp(self) -> Wide {
    if self.0 < 0.0 { return 1.0 / (-self).exp(); }
    #[inline] fn floor0(q:Wide) -> f64 {
      let q0f = q.0.floor();
      if -q.1 > (q.0 - q0f) { q0f - 1.0 } else { q0f }
    }
    let x = self;
    let n = floor0(x.abs() / Wide::LOG2);
    let r = x - Wide::LOG2 * n;
    let mut sum = Wide::one;
    let mut t = Wide::one;
    for i in 1..100 {
      let old_sum = sum;
      t *= r / (i as f64);
      sum += t;
      if sum == old_sum {break;}
    }
    sum.scale2(n as isize)
  }

  pub fn exp_m1(self) -> Wide {
    if self.abs() < Wide::LOG2 {
      let mut sum = Wide::zero;
      let mut t = Wide::one;
      for i in 1..100 {
        let old_sum = sum;
        t *= self / i;
        sum += t;
        if sum == old_sum {break;}
      }
      sum
    } else {
      self.exp() - Wide::one
    }
  }

  pub fn log(self) -> Wide {
    let (m0,e0) = crate::algorithm::frexp1(self.0);
    let x = Wide(m0, self.1.ldexp(-e0)) - 1;
    let terms = (2..).map(|n: isize| (x * (n>>1).sqr(), ι(n)));
    Wide::LOG2*e0 + (x / crate::algorithm::contfrac_modlentz(ι(1), terms, Wide::epsilon))
  }
}

impl Add<Wide> for Wide {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { qqadd(self, y) }
}
impl Sub<Wide> for Wide {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { qqadd(self, -y) }
}
impl Mul<Wide> for Wide {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { qqprod(self, y) }
}
impl Div<Wide> for Wide {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { qqdivide(self, y) }
}
impl Rem<Wide> for Wide {
  type Output = Wide;
  fn rem(self, y: Wide) -> Wide { unimplemented!() }
}
impl Neg for Wide {
  type Output = Wide;
  fn neg(self) -> Wide { qnegate(self) }
}

impl Add<f64> for Wide {
  type Output = Wide;
  fn add(self, y: f64) -> Wide { qdadd(self, y) }
}
impl Sub<f64> for Wide {
  type Output = Wide;
  fn sub(self, y: f64) -> Wide { qdadd(self, -y) }
}
impl Mul<f64> for Wide {
  type Output = Wide;
  fn mul(self, y: f64) -> Wide { qdprod(self, y) }
}
impl Div<f64> for Wide {
  type Output = Wide;
  fn div(self, y: f64) -> Wide { qddivide(self, y) }
}
impl Rem<f64> for Wide {
  type Output = Wide;
  fn rem(self, y: f64) -> Wide { unimplemented!() }
}

impl Add<isize> for Wide {
  type Output = Wide;
  fn add(self, y: isize) -> Wide { qdadd(self, y as f64) }
}
impl Sub<isize> for Wide {
  type Output = Wide;
  fn sub(self, y: isize) -> Wide { qdadd(self, -y as f64) }
}
impl Mul<isize> for Wide {
  type Output = Wide;
  fn mul(self, y: isize) -> Wide { qdprod(self, y as f64) }
}
impl Div<isize> for Wide {
  type Output = Wide;
  fn div(self, y: isize) -> Wide { qddivide(self, y as f64) }
}
impl Rem<isize> for Wide {
  type Output = Wide;
  fn rem(self, y: isize) -> Wide { unimplemented!() }
}

impl Add<Wide> for f64 {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { dqadd(self, y) }
}
impl Sub<Wide> for f64 {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { dqadd(self, -y) }
}
impl Mul<Wide> for f64 {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { dqprod(self, y) }
}
impl Div<Wide> for f64 {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { dqdivide(self, y) }
}

impl Add<Wide> for isize {
  type Output = Wide;
  fn add(self, y: Wide) -> Wide { dqadd(self as f64, y) }
}
impl Sub<Wide> for isize {
  type Output = Wide;
  fn sub(self, y: Wide) -> Wide { dqadd(self as f64, -y) }
}
impl Mul<Wide> for isize {
  type Output = Wide;
  fn mul(self, y: Wide) -> Wide { dqprod(self as f64, y) }
}
impl Div<Wide> for isize {
  type Output = Wide;
  fn div(self, y: Wide) -> Wide { dqdivide(self as f64, y) }
}

impl AddAssign<Wide> for Wide {
  fn add_assign(&mut self, y: Wide) { *self = qqadd(*self, y); }
}
impl SubAssign<Wide> for Wide {
  fn sub_assign(&mut self, y: Wide) { *self = qqadd(*self, -y); }
}
impl MulAssign<Wide> for Wide {
  fn mul_assign(&mut self, y: Wide) { *self = qqprod(*self, y); }
}
impl DivAssign<Wide> for Wide {
  fn div_assign(&mut self, y: Wide) { *self = qqdivide(*self, y); }
}
impl RemAssign<Wide> for Wide {
  fn rem_assign(&mut self, y: Wide) { unimplemented!() }
}

impl AddAssign<f64> for Wide {
  fn add_assign(&mut self, y: f64) { *self = qdadd(*self, y); }
}
impl SubAssign<f64> for Wide {
  fn sub_assign(&mut self, y: f64) { *self = qdadd(*self, -y); }
}
impl MulAssign<f64> for Wide {
  fn mul_assign(&mut self, y: f64) { *self = qdprod(*self, y); }
}
impl DivAssign<f64> for Wide {
  fn div_assign(&mut self, y: f64) { *self = qddivide(*self, y); }
}
impl RemAssign<f64> for Wide {
  fn rem_assign(&mut self, y: f64) { unimplemented!() }
}

impl AddAssign<isize> for Wide {
  fn add_assign(&mut self, y: isize) { *self = qdadd(*self, y as f64); }
}
impl SubAssign<isize> for Wide {
  fn sub_assign(&mut self, y: isize) { *self = qdadd(*self, -y as f64); }
}
impl MulAssign<isize> for Wide {
  fn mul_assign(&mut self, y: isize) { *self = qdprod(*self, y as f64); }
}
impl DivAssign<isize> for Wide {
  fn div_assign(&mut self, y: isize) { *self = qddivide(*self, y as f64); }
}
impl RemAssign<isize> for Wide {
  fn rem_assign(&mut self, y: isize) { unimplemented!() }
}

impl Shl<isize> for Wide {
  type Output = Wide;
  fn shl(self, n: isize) -> Wide { self.scale2(n) }
}
impl Shr<isize> for Wide {
  type Output = Wide;
  fn shr(self, n: isize) -> Wide { self.scale2(n) }
}
impl ShlAssign<isize> for Wide {
  fn shl_assign(&mut self, n: isize) { *self = *self << n; }
}
impl ShrAssign<isize> for Wide {
  fn shr_assign(&mut self, n: isize) { *self = *self >> n; }
}

impl PartialEq<f64> for Wide {
  fn eq(&self, n: &f64) -> bool { *self == Wide(*n, 0.0) }
}

impl PartialEq<isize> for Wide {
  fn eq(&self, n: &isize) -> bool { *self == Wide(*n as f64, 0.0) }
}

impl std::str::FromStr for Wide {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, ()> {
    if s.is_empty() { return Err(()); }
    let mut neg = false;
    let mut dec = false;
    let mut e = 0;
    let mut q = Wide(0.0, 0.0);
    let mut mantissa = true;
    let mut exp_neg = false;
    let mut expo = 0;
    for c in s.chars() {
      if mantissa {
        match c {
          '-' => { neg = true; }
          '+' => {}
          '.' => { dec = true; }
          'e' => { mantissa = false; }
          d => {
            let v = ((d as u8) - b'0') as f64;
            if !(0.0<=v && v<=9.0) { return Err(()); }
            q = q * 10.0 + v;
            if dec { e -= 1; }
          }
        }
      } else {
        match c {
          '-' => { exp_neg = true; }
          '+' => {}
          d => {
            let v = ((d as u8) - b'0') as isize;
            if !(0<=v && v<=9) { return Err(()); }
            expo = expo * 10 + v;
          }
        }
      }
    }
    if exp_neg { expo = -expo; }
    q = q.scale10(e + expo);
    Ok(if neg { -q } else { q })
  }
}

impl std::fmt::Display for Wide {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // local simple flooring function
    // assumes q >= 0.0
    fn floor0(q:&Wide) -> f64 {
      let q0f = q.0.floor();
      if -q.1 > (q.0 - q0f) {
        q0f - 1.0
      } else {
        q0f
      }
    }

    write!(f, "ξ")?;
    if self.0 == 0.0 {
      return write!(f, "0.0");
    }
    let mut q = *self;
    if q.0 < 0.0 {
      q = -q;
      write!(f, "-")?;
    }

    let mut e = 0;
    while q >= Wide(10.0, 0.0) {
      e += 1;
      q /= 10.0;
    }
    while q < Wide(1.0, 0.0) {
      e -= 1;
      q *= 10.0;
    }

    for n in 0..33 {
      if n == 1 {
        write!(f, ".")?;
      }
      let d = floor0(&q);
      let dd = ((d as u8) + b'0') as char;
      write!(f, "{}", dd)?;
      q = (q - d) * 10.0;
    }

    if e != 0 {
      write!(f, "e{}", e)?;
    }
    write!(f, "")
  }
}

impl Base for Wide {}
impl Zero for Wide {
  const zero: Wide = Wide(0.0, 0.0);
}
impl Addition for Wide {}
impl Subtraction for Wide {}
impl Additive for Wide {}
impl One for Wide {
  const one: Wide = Wide(1.0, 0.0);
}
impl Multiplication for Wide {}
impl Division for Wide {}
impl Multiplicative for Wide {}
impl Embeds<isize> for Wide {}
impl Embeds<f64> for Wide {}
impl Field for Wide {}
impl Roots for Wide {
  #[inline]
  fn sqrt(self) -> Self { self.sqrt() }
  #[inline]
  fn cbrt(self) -> Self { self.cbrt() }
  #[inline]
  fn nth_root(self, n: isize) -> Self { self.nth_root(n) }
}

impl Power for Wide {
  #[inline]
  fn pow(self, e:Self) -> Self { self.powf(e) }
}

impl Constants for Wide {
  const nan: Self = Wide(f64::NAN,f64::NAN);

  // $e^1$
  // 2.7182818284590452353602874713526624977572470937000
  const E: Self = Wide!("2.b7e151628aed2a6abf7158809cf4f3c762e7160f4");

  // $e^{-1}$
  // 0.36787944117144232159552377016146086744581113103177
  const FRAC_1_E: Self = Wide!("0.5e2d58d8b3bcdf1abadec7829054f90dda9805aab");

  // $\pi$
  // 3.1415926535897932384626433832795028841971693993751
  const PI: Self = Wide!("3.243f6a8885a308d313198a2e03707344a40938223");

  // $1/\pi$
  // 0.31830988618379067153776752674502872406891929148091
  const FRAC_1_PI: Self = Wide!("0.517cc1b727220a94fe13abe8fa9a6ee06db14acca");

  // $\pi/2$
  // 1.5707963267948966192313216916397514420985846996876
  const FRAC_PI_2: Self = Wide!("1.921fb54442d18469898cc51701b839a252049c111");

  // $\sqrt(\pi)$
  // 1.7724538509055160272981674833411451827975494561224
  const SQRTPI: Self = Wide!("1.c5bf891b4ef6aa79c3b0520d5db9383fe3921546f");

  // $\sqrt(2\pi)$
  // 2.5066282746310005024157652848110452530069867406099
  const SQRT2PI: Self = Wide!("2.81b263fec4e0b2caf9483f5ce459dc5f19f3ea641");

  // $1/\sqrt(2\pi)$
  // 0.39894228040143267793994605993438186847585863116493
  const FRAC_1_SQRT2PI: Self = Wide!("0.662114cf50d942343f2cf1402eae38bfd3829f305");

  // $1/\sqrt(\pi)$
  // 0.56418958354775628694807945156077258584405062932900
  const FRAC_1_SQRTPI: Self = Wide!("0.906eba8214db688d71d48a7f6bfec3441409a0ebb");

  // $\log(2)$
  // 0.69314718055994530941723212145817656807550013436026
  const LOG2: Self = Wide!("0.b17217f7d1cf79abc9e3b39803f2f6af40f343267");

  // $1/\log(2)$
  // 1.4426950408889634073599246810018921374266459541530
  const FRAC_1_LOG2: Self = Wide!("1.71547652b82fe1777d0ffda0d23a7d11d6aef551c");

  // $\log(2\pi)/2 = \log(\sqrt{2\pi})$
  // 0.91893853320467274178032973640561763986139747363778
  const FRAC_LOG2PI_2: Self = Wide!("0.eb3f8e4325f5a53494bc900144192023cfb08f8d1");

  // Euler's gamma $\gamma$
  // 0.57721566490153286060651209008240243104215933593992
  const EULER_GAMMA: Self = Wide!("0.93c467e37db0c7a4d1be3f810152cb56a1cecc3af");
}

use crate::real::r64;
impl Normed for Wide {
  type NT = r64;
  const epsilon : r64 = r64::epsilon*r64::epsilon*2.0;
  fn abs(self) -> Self::NT {
    r64(self.0.abs())
  }
  fn vabs(self) -> Self {
    if self.0 < 0.0 { -self } else { self }
  }
  fn fabs(self) -> f64 {
    self.0.abs()
  }
  // self/|self|
  fn signum(self) -> Self {
    Wide(self.0.signum(), 0.0)
  }
  fn mu(self) -> Self::NT {
    r64(self.0.abs())
  }
}

impl Classify for Wide {
  fn is_nan(self) -> bool {
    self.0.is_nan()
  }
  fn is_infinite(self) -> bool {
    self.0.is_infinite()
  }
  fn is_finite(self) -> bool {
    self.0.is_finite()
  }

  fn is_zero(self) -> bool {
    self.0 == 0.0
  }
  fn is_negzero(self) -> bool {
    self.is_zero() && self.0.is_sign_negative()
  }
  fn is_real(self) -> bool {
    true
  }
  fn is_imag(self) -> bool {
    false
  }

  fn is_negreal(self) -> bool {
    self.0 < 0.0
  }
  fn is_posreal(self) -> bool {
    self.0 > 0.0
  }
  fn is_nonnegreal(self) -> bool {
    self.0 <= 0.0
  }
  fn is_nonposreal(self) -> bool {
    self.0 >= 0.0
  }

  fn is_int(self) -> bool {
    self.trunc() == self
  }
  fn is_posint(self) -> bool {
    self.is_posreal() && self.is_int()
  }
  fn is_negint(self) -> bool {
    self.is_negreal() && self.is_int()
  }
  fn is_nonposint(self) -> bool {
    self.is_nonposreal() && self.is_int()
  }
  fn is_nonnegint(self) -> bool {
    self.is_nonnegreal() && self.is_int()
  }
  fn is_evenint(self) -> bool {
    todo!()
  }
  fn is_oddint(self) -> bool {
    todo!()
  }

  fn is_halfint(self) -> bool {
    (self*2).is_int()
  }

  // upper-half complex plane (positive imag part)?
  // positive real part?
}

impl Ordered for Wide {
  #[inline]
  fn floor(self) -> Self {
    todo!()
  }
  #[inline]
  fn ceil(self) -> Self {
    todo!()
  }
  #[inline]
  fn round(self) -> Self {
    todo!()
  }
  #[inline]
  fn trunc(self) -> Self {
    let t0 = self.0.trunc();
    if t0 == self.0 {
      Wide(t0, self.1.trunc()) // TODO: can be wrong if sign(lo)!=sign(hi)
    } else {
      Wide(t0, 0.0)
    }
  }
  #[inline]
  fn rint(self) -> isize {
    let r = self.round();
    (r.0 as isize) + (r.1 as isize)
  }
}

impl Bounded for Wide {
  const MIN_VALUE: Wide = Wide(f64::MIN,0.0);
  const MAX_VALUE: Wide = Wide(f64::MAX,0.0); // TODO: not actually the largest!
}

impl Value for Wide {}
//impl RealType for Wide { type CT = c64; }


