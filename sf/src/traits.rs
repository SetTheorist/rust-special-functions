use core::ops::{Add,Sub,Mul,Div,Rem,Neg};
use core::ops::{AddAssign,SubAssign,MulAssign,DivAssign,RemAssign};
use core::ops::{Shl,ShlAssign,Shr,ShrAssign};
use num::complex::{Complex};
use crate::algorithm::{power_i, power_u};

// we assume for convenience that our basic values
// are all Copy-able.
// This excludes, for example, arbitrary-precision floats,
// but we are not targeting such use cases...
pub trait Base
  : Copy + Sized
  + PartialEq
  + Default + std::fmt::Debug
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
  fn sqr(self) -> Self { self*self }
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
  fn recip(self) -> Self { Self::one / self }
  fn powi(self, n:isize) -> Self { power_i(self,n) }
  fn ldexp(self, n:isize) -> Self { self << n }
}
//pub fn recip<M:Division>(x:M) -> M { x.recip() }
//pub fn powi<M:Division>(x:M, n:isize) -> M { x.powi(n) }

pub trait Multiplicative
  : Multiplication +  Division
{
}

pub trait Embeds<T>
  : Base
  + Add<T,Output=Self> + AddAssign<T>
  + Sub<T,Output=Self> + SubAssign<T>
  + Mul<T,Output=Self> + MulAssign<T>
  + Div<T,Output=Self> + DivAssign<T>
  + Rem<T,Output=Self> + RemAssign<T>
  + From<T>
  //where T:Add<Self,Output=Self> // ?!
{
}
// need swapped versions also, but weird compiler issues

pub trait Field
  : Additive + Multiplicative
  + Embeds<isize> + Embeds<f64>
{
  // self * (-1)^n
  fn pari(self, n:isize) -> Self { if n%2==0 {Self::one} else {-Self::one} }
}

pub trait Roots
: Field
{
  fn sqrt(self) -> Self;
  fn cbrt(self) -> Self;
  fn sqrt_recip(self) -> Self { self.sqrt().recip() }
  fn cbrt_recip(self) -> Self { self.cbrt().recip() }
  fn nth_root(self, n:isize) -> Self;
}


pub trait Bounded
{
  const MIN_VALUE : Self;
  const MAX_VALUE : Self;
}

pub trait Ordered
  : Base + PartialOrd<Self>
{
  fn min(self,b:Self) -> Self { if self<b {self} else {b} }
  fn max(self,b:Self) -> Self { if self>b {self} else {b} }

  fn floor(self) -> Self;
  fn ceil(self) -> Self;
  fn round(self) -> Self;
  fn trunc(self) -> Self;
  fn rint(self) -> isize;
}

pub trait Normed
  : Base
{
  type NT : Ordered;
  fn abs(self) -> Self::NT;
  fn fabs(self) -> f64;
  // self/|self|
  fn signum(self) -> Self;
}
pub fn abs<T:Normed>(x:T) -> T::NT { x.abs() }
pub fn fabs<T:Normed>(x:T) -> f64 { x.fabs() }

pub trait ComplexType
: Base
  + Normed<NT=Self::RT>
  + Embeds<Self::RT>
  + Embeds<Complex<f64>>
{
  type RT : Field+Ordered;
  fn real(self) -> Self::RT;
  fn imag(self) -> Self::RT;
  fn arg(self) -> Self::RT;
  fn rect(re:Self::RT,im:Self::RT) -> Self;
  fn polar(r:Self::RT,arg:Self::RT) -> Self;
}

pub trait RealType
: Base
  + Normed<NT=Self>
{
}

pub trait Value
  : Field + Normed
{
}

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



