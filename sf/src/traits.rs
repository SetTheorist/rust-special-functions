use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use core::ops::{Shl,ShlAssign,Shr,ShrAssign};
use num::complex::{Complex};
use crate::algorithm::{power_i, power_u};

#[inline]
pub fn ι<A,B:From<A>>(a:A) -> B { B::from(a) }

// we assume for convenience that our basic values
// are all Copy-able.
// This excludes, for example, arbitrary-precision floats,
// but we are not targeting such use cases...
pub trait Base
  : Copy + Sized
  + PartialEq
  + Default + std::fmt::Debug
  + 'static
{ }


pub trait Zero : Base {
  const zero : Self;
}

pub trait Addition
  : Base + Zero
  + Add<Self,Output=Self> + AddAssign<Self>
{
}

pub trait Subtraction
  : Base + Addition
  + Sub<Self,Output=Self> + SubAssign<Self>
  + Neg<Output=Self>
{
}

pub trait Additive
  : Addition + Subtraction
{
}

pub trait One : Base {
  const one : Self;
}

// absorb isize & f64 into operations also...

pub trait Multiplication
  : Base + One
  + Mul<Self,Output=Self> + MulAssign<Self>
{
  #[inline]
  fn sqr(self) -> Self { self*self }
  #[inline]
  fn powu(self, n:usize) -> Self { power_u(self,n) }
}
//pub fn sqr<M:Multiplication>(x:M) -> M { x.sqr() }
//pub fn powu<M:Multiplication>(x:M, n:usize) -> M { x.powu(n) }
//pub fn ldexp<M:Multiplication>(x:M, n:isize) -> M { x.ldexp(n) }

pub trait Division
  : Base + Multiplication
  + Div<Self,Output=Self> + DivAssign<Self>
  + Rem<Self,Output=Self> + RemAssign<Self>
  + Shl<isize,Output=Self> + ShlAssign<isize>
  + Shr<isize,Output=Self> + ShrAssign<isize>
{
  #[inline]
  fn recip(self) -> Self { Self::one / self }
  #[inline]
  fn powi(self, n:isize) -> Self { power_i(self,n) }
  #[inline]
  fn ldexp(self, n:isize) -> Self { self << n }
}
//pub fn recip<M:Division>(x:M) -> M { x.recip() }
//pub fn powi<M:Division>(x:M, n:isize) -> M { x.powi(n) }

pub trait Multiplicative
  : Multiplication +  Division
{
}

// left-embedding has issues due to current compiler constraints
// c.f. https://github.com/rust-lang/rust/issues/86635
pub trait Embeds<T>
  : Base
  + Add<T,Output=Self> + AddAssign<T>
  + Sub<T,Output=Self> + SubAssign<T>
  + Mul<T,Output=Self> + MulAssign<T>
  + Div<T,Output=Self> + DivAssign<T>
  + Rem<T,Output=Self> + RemAssign<T>
  + From<T>
  + PartialEq<T>
{
}

pub trait Field
  : Additive + Multiplicative
  + Embeds<isize> + Embeds<f64>
{
  // self * (-1)^n
  #[inline]
  fn pari(self, n:isize) -> Self { if n%2==0 {self} else {-self} }
}

pub trait Roots
: Field
{
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
  #[inline]
  fn sqrt_recip(self) -> Self { self.sqrt().recip() }
  #[inline]
  fn cbrt_recip(self) -> Self { self.cbrt().recip() }
  fn nth_root(self, n:isize) -> Self;
}


pub trait Bounded
{
  const MIN_VALUE : Self;
  const MAX_VALUE : Self;
}

pub trait Power<P=Self>
  : Base
{
  fn pow(self, p:P) -> Self;
}

impl<T:Multiplication> Power<usize> for T {
  fn pow(self, p:usize) -> Self {
    self.powu(p)
  }
}

impl<T:Division> Power<isize> for T {
  fn pow(self, p:isize) -> Self {
    self.powi(p)
  }
}

pub trait Ordered
  : Base + PartialOrd<Self>
{
  #[inline]
  fn min(self,b:Self) -> Self { if self<b {self} else {b} }
  #[inline]
  fn max(self,b:Self) -> Self { if self>b {self} else {b} }

  fn floor(self) -> Self;
  fn ceil(self) -> Self;
  fn round(self) -> Self;
  fn trunc(self) -> Self;
  fn rint(self) -> isize;
}

pub trait Normed
  : Base+From<Self::NT>
{
  type NT : Field+Ordered;
  const epsilon : Self::NT;
  fn abs(self) -> Self::NT;
  fn fabs(self) -> f64;
  // self/|self|
  fn signum(self) -> Self;

  fn mu(self) -> Self::NT;
  const mu_epsilon : Self::NT;
}
#[inline]
pub fn abs<T:Normed>(x:T) -> T::NT { x.abs() }
#[inline]
pub fn fabs<T:Normed>(x:T) -> f64 { x.fabs() }
#[inline]
pub fn signum<T:Normed>(x:T) -> T { x.signum() }
#[inline]
pub fn mu<T:Normed>(x:T) -> T::NT { x.mu() }
pub fn μ<T:Normed>(x:T) -> T::NT { x.mu() }

pub trait ComplexType
  : Base
  + Normed<NT=Self::RT>
  + Embeds<Self::RT>
  //+ Embeds<Complex<f64>>
{
  type RT : Field+Ordered;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn arg(self) -> Self::RT;
  fn conj(self) -> Self;
  fn rect(re:Self::RT,im:Self::RT) -> Self;
  fn polar(r:Self::RT,arg:Self::RT) -> Self;
  fn to_rect(self) -> (Self::RT,Self::RT) { (self.real(), self.imag()) }
  fn to_polar(self) -> (Self::RT,Self::RT) {
    let a = self.abs();
    if a == 0 { (a,a) }
    else { (a, self.arg()) }
  }
}

pub trait RealType
  : Base
  + Normed<NT=Self>
  + Ordered
{
}

pub trait Value
  : Field + Normed + Roots
{
}

pub trait RealValue : Value + RealType { }
impl<T> RealValue for T where T:Value + RealType { }

pub trait ComplexValue : Value + ComplexType { }
impl<T> ComplexValue for T where T:Value + ComplexType { }

////////////////////////////////////////////////////////////////////////////////

/*
pub trait ExpLog : ...
{
  fn powf(self,e:Self) -> Self { (e*self.log()).exp() }
  fn exp(self) -> Self; // etc.
  fn log(self) -> Self; // etc.
}

pub trait Constants
{
  const EPSILON : Self;
  const TINY : Self; // ~ O(sqrt(min-positive normal)) for example

  // $e^1$
  const E : Self;

  // $\pi$
  const PI : Self;
  // $1/\pi$
  const FRAC_1_PI : Self;
  // $\pi/2$
  const FRAC_PI_2 : Self;
  // $\sqrt(2\pi)$
  const SQRT2PI : Self;
  // $1/\sqrt(2\pi)$
  const FRAC_1_SQRT2PI : Self;
  // $1/\sqrt(\pi)$
  const FRAC_1_SQRTPI : Self;

  // $\log(2)$
  const LOG2 : Self;
  // $1/\log(2)$
  const FRAC_1_LOG2 : Self;

  // $log(2\pi)/2$
  const FRAC_LOG2PI_2 : Self;
}
pub trait Float {
  const EPSILON : Self;
  const NAN : Self;
  const INFINITY : Self;
  fn is_nan(self) -> bool;
  fn is_infinite(self) -> bool;
  fn is_finite(self) -> bool;
}

*/
////////////////////////////////////////////////////////////////////////////////



