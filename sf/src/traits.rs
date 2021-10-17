use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
//use num::complex::{Complex};
use crate::algorithm::{power_i, power_u};

#[inline]
pub fn ι<A, B:From<A>>(a: A) -> B { B::from(a) }

// we assume for convenience that our basic values
// are all Copy-able.
// This excludes, for example, arbitrary-precision floats,
// but we are not targeting such use cases...
pub trait Base: Copy+Sized+PartialEq+Default+std::fmt::Debug+std::fmt::Display+'static {}

pub trait Power<P=Self>: Base {
  fn pow(self, p: P) -> Self;
}
#[inline]
pub fn sf_pow<P,V:Power<P>>(x:V, p:P) -> V { x.pow(p) }

pub trait Zero: Base {
  const zero: Self;
}

pub trait Addition:
  Base
  + Zero
  + Add<Self, Output=Self>
  + AddAssign<Self>
{}

pub trait Subtraction:
  Base
  + Addition
  + Sub<Self, Output=Self>
  + SubAssign<Self>
  + Neg<Output=Self>
{}

pub trait Additive:
  Addition
  + Subtraction
{}

pub trait One: Base {
  const one: Self;
}

// absorb isize & f64 into operations also...

pub trait Multiplication: Base + One + Mul<Self, Output = Self> + MulAssign<Self> {
  #[inline]
  fn sqr(self) -> Self { self * self }
  fn cub(self) -> Self { self.sqr() * self }
}
pub fn sf_sqr<M:Multiplication>(x:M) -> M { x.sqr() }
pub fn sf_cub<M:Multiplication>(x:M) -> M { x.cub() }

impl<T: Multiplication> Power<usize> for T {
  #[inline]
  fn pow(self, u: usize) -> Self { power_u(self, u) }
}

pub trait Division:
  Base
  + Multiplication
  + Div<Self, Output = Self>
  + DivAssign<Self>
  + Rem<Self, Output = Self>
  + RemAssign<Self>
  + Shl<isize, Output = Self>
  + ShlAssign<isize>
  + Shr<isize, Output = Self>
  + ShrAssign<isize>
{
  #[inline]
  fn recip(self) -> Self { Self::one / self }
}

impl<T: Division> Power<isize> for T {
  #[inline]
  fn pow(self, i: isize) -> Self { power_i(self, i) }
}

pub trait Ring: Base + Additive + Multiplication {}

impl<T: Base + Additive + Multiplication> Ring for T {}

pub trait Multiplicative: Multiplication + Division {}

pub trait DivisionRing: Ring + Division {}

impl<T: Ring + Division> DivisionRing for T {}


// left-embedding has issues due to current compiler constraints
// c.f. https://github.com/rust-lang/rust/issues/86635
// can we get trait alias to work?
pub trait Embeds<T>:
  Base
  + Add<T, Output = Self>
  + AddAssign<T>
  + Sub<T, Output = Self>
  + SubAssign<T>
  + Mul<T, Output = Self>
  + MulAssign<T>
  + Div<T, Output = Self>
  + DivAssign<T>
  + Rem<T, Output = Self>
  + RemAssign<T>
  + From<T>
  + PartialEq<T>
{ }

impl<T> Embeds<T> for T
  where T:Base
  + Add<T,Output=T> + AddAssign<T>
  + Sub<T,Output=T> + SubAssign<T>
  + Mul<T,Output=T> + MulAssign<T>
  + Div<T,Output=T> + DivAssign<T>
  + Rem<T, Output=T> + RemAssign<T>
  + From<T> + PartialEq<T>
{ }

pub trait Field: Additive + Multiplicative + Embeds<isize> + Embeds<f64> {
  // self * (-1)^n
  #[inline]
  fn pari(self, n: isize) -> Self {
    if n % 2 == 0 { self } else { -self }
  }
}

pub trait Roots: Field {
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
  #[inline]
  fn sqrt_recip(self) -> Self { self.sqrt().recip() }
  #[inline]
  fn cbrt_recip(self) -> Self { self.cbrt().recip() }
  fn nth_root(self, n: isize) -> Self;
}
pub fn sf_sqrt<V:Roots>(x:V) -> V { x.sqrt() }
pub fn sf_sqrt_recip<V:Roots>(x:V) -> V { x.sqrt_recip() }

pub trait Bounded {
  const MIN_VALUE: Self;
  const MAX_VALUE: Self;
}

pub trait Ordered: Base + PartialOrd<Self> {
  #[inline]
  fn min(self, b: Self) -> Self {
    if self < b { self } else { b }
  }
  #[inline]
  fn max(self, b: Self) -> Self {
    if self > b { self } else { b }
  }

  fn floor(self) -> Self;
  fn ceil(self) -> Self;
  fn round(self) -> Self;
  fn trunc(self) -> Self;
  fn rint(self) -> isize;

  #[inline]
  fn є(self, a:Self, b:Self) -> bool { a<=self && self<=b }
  #[inline]
  fn є_cc(self, a:Self, b:Self) -> bool { a.є(a,b) }
  #[inline]
  fn є_oc(self, a:Self, b:Self) -> bool { a<self && self<=b }
  #[inline]
  fn є_co(self, a:Self, b:Self) -> bool { a<=self && self<b }
  #[inline]
  fn є_oo(self, a:Self, b:Self) -> bool { a<self && self<b }
}
pub fn sf_min<V:Ordered>(a:V, b:V) -> V { a.min(b) }
pub fn sf_max<V:Ordered>(a:V, b:V) -> V { a.max(b) }
pub fn sf_floor<V:Ordered>(a:V) -> V { a.floor() }
pub fn sf_ceil<V:Ordered>(a:V) -> V { a.ceil() }
pub fn sf_round<V:Ordered>(a:V) -> V { a.round() }
pub fn sf_trunc<V:Ordered>(a:V) -> V { a.trunc() }
pub fn sf_rint<V:Ordered>(a:V) -> isize { a.rint() }

pub trait Normed: Base + From<Self::NT> {
  type NT: Field + Ordered;
  const epsilon: Self::NT;
  fn abs(self) -> Self::NT;
  fn vabs(self) -> Self;
  fn fabs(self) -> f64;
  // self/|self|
  fn signum(self) -> Self;
  fn mu(self) -> Self::NT;
}
#[inline]
pub fn abs<T: Normed>(x: T) -> T::NT { x.abs() }
#[inline]
pub fn vabs<T: Normed>(x: T) -> T { x.vabs() }
#[inline]
pub fn fabs<T: Normed>(x: T) -> f64 { x.fabs() }
#[inline]
pub fn signum<T: Normed>(x: T) -> T { x.signum() }
#[inline]
pub fn μ<T: Normed>(x: T) -> T::NT { x.mu() }

pub trait ComplexType: Base + Normed<NT = Self::RT> + Embeds<Self::RT>
{
  type RT: RealValue<CT=Self>;
  const I: Self;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn arg(self) -> Self::RT;
  fn conj(self) -> Self;
  fn rect(re: Self::RT, im: Self::RT) -> Self;
  fn polar(r: Self::RT, arg: Self::RT) -> Self;
  fn to_rect(self) -> (Self::RT, Self::RT) { (self.real(), self.imag()) }
  fn to_polar(self) -> (Self::RT, Self::RT) {
    let a = self.abs();
    if a == 0 {
      (a, a)
    } else {
      (a, self.arg())
    }
  }
  fn root_of_unity(n:isize) -> Self;
}

pub trait RealType: Base + Normed<NT = Self> + Ordered {
  type CT: ComplexValue<RT=Self>;
}

pub trait Classify {
  fn is_nan(self) -> bool;
  fn is_infinite(self) -> bool;
  fn is_finite(self) -> bool;

  fn is_zero(self) -> bool;
  fn is_negzero(self) -> bool;
  fn is_real(self) -> bool;
  fn is_imag(self) -> bool;

  fn is_negreal(self) -> bool;
  fn is_posreal(self) -> bool;
  fn is_nonnegreal(self) -> bool;
  fn is_nonposreal(self) -> bool;

  fn is_int(self) -> bool;
  fn is_posint(self) -> bool;
  fn is_negint(self) -> bool;
  fn is_nonposint(self) -> bool;
  fn is_nonnegint(self) -> bool;
  fn is_evenint(self) -> bool;
  fn is_oddint(self) -> bool;

  fn is_halfint(self) -> bool;

  // upper-half complex plane (positive imag part)?
  // positive real part?
}

// for "floating-point" type (real) values
pub trait Float : Base {
  // Split into normalized mantissa and exponent
  fn frexp(self) -> (Self, isize);
  // extract only the exponent
  fn ilogb(self) -> isize;

  // self * 2^n
  fn ldexp(self, n:isize) -> Self;

  // magnitude of self, but with sign-bit from x
  fn copysign(self, x:Self) -> Self;

  // next representable number larger
  fn next_up(self) -> Self;
  // prev representable number smaller
  fn next_dn(self) -> Self;

  // checks for bitwise identity
  fn identical(self, rhs:Self) -> bool;

  const infinity: Self;
  const neg_infinity: Self;
  const neg_zero: Self;
}

pub trait Constants {
  const nan: Self;

  // $e^1$
  const E: Self;
  // $e^{-1}$
  const FRAC_1_E: Self;
  // $\pi$
  const PI: Self;
  // $1/\pi$
  const FRAC_1_PI: Self;
  // $\pi/2$
  const FRAC_PI_2: Self;
  // $\sqrt(\pi)$
  const SQRTPI: Self;
  // $\sqrt(2\pi)$
  const SQRT2PI: Self;
  // $1/\sqrt(2\pi)$
  const FRAC_1_SQRT2PI: Self;
  // $1/\sqrt(\pi)$
  const FRAC_1_SQRTPI: Self;
  // $\log(2)$
  const LOG2: Self;
  // $1/\log(2)$
  const FRAC_1_LOG2: Self;
  // $\log(2\pi)/2 = \log(\sqrt{2\pi})$
  const FRAC_LOG2PI_2: Self;
  // Euler's gamma $\gamma$
  const EULER_GAMMA: Self;
}


pub trait Value: Base+Field+Normed+Roots+Constants+Classify+Power<isize> {}

pub trait RealValue: Value+RealType {}
impl<T> RealValue for T where T:Value+RealType {}

pub trait ComplexValue: Value+ComplexType {}
impl<T> ComplexValue for T where T:Value+ComplexType { }

// TODO: maybe?
//pub trait RealComplexPair<R:RealValue,C:ComplexValue<RT=R>> {}
// TODO: maybe?
pub trait Widenable : Value {
  type Wider : Value;
}

////////////////////////////////////////////////////////////////////////////////

impl Base for isize {}
impl Zero for isize { const zero: isize = 0; }
impl Addition for isize {}
impl Subtraction for isize {}
impl Additive for isize {}
impl One for isize { const one: isize = 1; }
impl Multiplication for isize {}
impl Division for isize {}
impl Multiplicative for isize {}

impl Classify for isize {
  #[inline] fn is_nan(self) -> bool { false }
  #[inline] fn is_infinite(self) -> bool { false }
  #[inline] fn is_finite(self) -> bool { true }
  #[inline] fn is_zero(self) -> bool { self == 0 }
  #[inline] fn is_negzero(self) -> bool { false }
  #[inline] fn is_real(self) -> bool { true }
  #[inline] fn is_imag(self) -> bool { false }
  #[inline] fn is_negreal(self) -> bool { self < 0 }
  #[inline] fn is_posreal(self) -> bool { self > 0 }
  #[inline] fn is_nonnegreal(self) -> bool { self >= 0 }
  #[inline] fn is_nonposreal(self) -> bool { self <= 0 }
  #[inline] fn is_int(self) -> bool { true }
  #[inline] fn is_posint(self) -> bool { self > 0 }
  #[inline] fn is_negint(self) -> bool { self < 0 }
  #[inline] fn is_nonposint(self) -> bool { self <= 0 }
  #[inline] fn is_nonnegint(self) -> bool { self >= 0 }
  #[inline] fn is_evenint(self) -> bool { self%2 == 0 }
  #[inline] fn is_oddint(self) -> bool { self%2 == 1 }
  #[inline] fn is_halfint(self) -> bool { true }
}

////////////////////////////////////////////////////////////////////////////////

impl Base for f64 {}
impl Zero for f64 { const zero: f64 = 0.0; }
impl Addition for f64 {}
impl Subtraction for f64 {}
impl Additive for f64 {}
impl One for f64 { const one: f64 = 1.0; }
impl Multiplication for f64 {}
//impl Division for f64 { }
//impl Multiplicative for f64 { }

impl Classify for f64 {
  #[inline] fn is_nan(self) -> bool { self.is_nan() }
  #[inline] fn is_infinite(self) -> bool { self.is_infinite() }
  #[inline] fn is_finite(self) -> bool { self.is_finite() }
  #[inline] fn is_zero(self) -> bool { self == 0.0 }
  #[inline] fn is_negzero(self) -> bool { self == 0.0 && 1.0_f64.copysign(self) < 0.0}
  #[inline] fn is_real(self) -> bool { true }
  #[inline] fn is_imag(self) -> bool { false }
  #[inline] fn is_negreal(self) -> bool { self < 0.0 }
  #[inline] fn is_posreal(self) -> bool { self > 0.0 }
  #[inline] fn is_nonnegreal(self) -> bool { self >= 0.0 }
  #[inline] fn is_nonposreal(self) -> bool { self <= 0.0 }
  #[inline] fn is_int(self) -> bool { self == self.trunc() }
  #[inline] fn is_posint(self) -> bool { self.is_int() && self.is_posreal() }
  #[inline] fn is_negint(self) -> bool { self.is_int() && self.is_negreal() }
  #[inline] fn is_nonposint(self) -> bool { self.is_int() && self.is_nonposreal() }
  #[inline] fn is_nonnegint(self) -> bool { self.is_int() && self.is_nonnegreal() }
  #[inline] fn is_evenint(self) -> bool {
    const last_odd : f64 = 9007199254740991_f64; // 2^53-1
    self.is_int()
    && (self.abs() > last_odd
        || (self.abs().trunc() as i64)%2 == 0)
  }
  #[inline] fn is_oddint(self) -> bool {
    const last_odd : f64 = 9007199254740991_f64; // 2^53-1
    self.is_int()
    && self.abs() <= last_odd
    && (self.abs().trunc() as i64)%2 == 1
  }
  #[inline] fn is_halfint(self) -> bool { (self*2.0).is_int() }
}

impl Float for f64 {
  // Split into normalized mantissa and exponent
  #[inline]
  fn frexp(self) -> (Self, isize) {
    let b = self.to_bits();
    let e = (((b >> 52) & 0x7FFF) as isize) - 1023;
    let n = (b & !(0x7FFF<<52)) | (1023<<52);
    let f = f64::from_bits(n);
    (f, e)
  }
  // extract only the exponent
  #[inline]
  fn ilogb(self) -> isize {
    (((self.to_bits() >> 52) & 0x7FFF) as isize) - 1023
  }
  // self * 2^n
  #[inline]
  fn ldexp(self, n:isize) -> Self {
    if self == 0.0 || !self.is_finite() {return self;}
    // TODO: this will fail on edge cases
    let b = self.to_bits();
    let e = ((((b >> 52) & 0x7FF) as i64) + (n as i64)) as u64;
    f64::from_bits( (b & !(0x7FF<<52))|(e<<52) )
  }

  // magnitude of self, but with sign-bit from x
  #[inline]
  fn copysign(self, x:Self) -> Self {
    self.copysign(x)
  }

  // next representable number larger
  #[inline]
  fn next_up(self) -> Self {
    f64::from_bits(f64::to_bits(self)+1)
  }
  // prev representable number smaller
  #[inline]
  fn next_dn(self) -> Self {
    f64::from_bits(f64::to_bits(self)-1)
  }

  // checks for bitwise identity
  #[inline]
  fn identical(self, rhs:Self) -> bool {
    self.to_bits() == rhs.to_bits()
  }

  const infinity: Self = f64::INFINITY;
  const neg_infinity: Self = f64::NEG_INFINITY;
  const neg_zero: Self = unsafe{std::mem::transmute(0x8000_0000_0000_0000_u64)};
}

impl Ordered for f64 {
  #[inline] fn min(self, b:Self) -> Self { self.min(b) }
  #[inline] fn max(self, b:Self) -> Self { self.max(b) }
  #[inline] fn floor(self) -> Self { self.floor() }
  #[inline] fn ceil(self) -> Self { self.ceil() }
  #[inline] fn round(self) -> Self { self.round() }
  #[inline] fn trunc(self) -> Self { self.trunc() }
  #[inline] fn rint(self) -> isize { self.round() as isize }
}

////////////////////////////////////////////////////////////////////////////////
